// The Relay crew: builds Sans Fiat one piece at a time, one `claude -p` session per step,
// on GitHub Actions (.github/workflows/relay.yml). Ported from Tortoise Spelling's Jogging.
//
//   node relay/relay.mjs run       # run steps until the budget, the deadline or the work runs out
//   node relay/relay.mjs status    # print the next step and the pacing verdict, run nothing
//   node relay/relay.mjs event     # act on the owner's comment (relay-events.yml)
//   node relay/relay.mjs report <message>   # comment on the Relay log issue
//
// GitHub holds the state. A piece is an open `kind:piece` issue labelled `relay:ready`, and a
// retro an open `kind:retro` one; its steps are `kind:step` sub-issues whose bodies carry a `<!-- relay {...} -->` block with the
// step's model, effort and attempts. The spec and briefs live on the piece's branch,
// `relay/<number>`. A session reports how its step ended in relay/runs/result.json, and the
// crew is the only one that closes a step. relay/README.md explains the whole loop.
//
// Environment: RELAY_APP_ID and RELAY_APP_KEY (the App, see token.mjs), CLAUDE_CODE_OAUTH_TOKEN,
// RELAY_BOARD_TOKEN (optional: a classic token with the `project` scope, since an App can't
// reach a personal account's project), and for `event` GITHUB_EVENT_PATH and DISPATCH_TOKEN
// (the workflow's own token, for starting a run).

import { execFileSync, spawn } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { appToken } from "./token.mjs";
import { readUsage } from "./usage.mjs";
import { USAGE_EVERY_MIN } from "./usage-hook.mjs";

const HERE = path.dirname(fileURLToPath(import.meta.url));
const REPO = path.dirname(HERE);
const RUNS = path.join(HERE, "runs");
const RESULT = path.join(RUNS, "result.json");
export const OWNER = "DontPanic345";
// The crew's own login: it opens the fixes piece, proposed pieces and the retro.
export const BOT = "sansfiat-relay[bot]";
const GH_REPO = `${OWNER}/sansfiat`;
const BOARD = "Sans Fiat Work";
const LOG_TITLE = "Relay log";
// The run log's issue for a milestone: one each, so the latest status is a short scroll.
export const logTitle = (milestone) => (milestone ? `${LOG_TITLE}: ${milestone.title}` : LOG_TITLE);
// A wait job sleeps at most this long; a lift further off is waited for again by the run it starts.
const WAIT_MAX_MS = 300 * 60_000;
const WORKFLOW = "relay.yml";
const MAX_ATTEMPTS = 2;
const STARTED = Date.now();
// The job is killed at 355 minutes. No step starts after the deadline, and none runs past
// its timeout, so the crew always gets to record the last one.
const DEADLINE_MS = Number(process.env.RELAY_DEADLINE_MIN ?? 250) * 60_000;
const SESSION_TIMEOUT_MS = Number(process.env.RELAY_SESSION_MIN ?? 90) * 60_000;
// A whole step's session runs as long as a 5-hour usage window.
const WHOLE_TIMEOUT_MS = Number(process.env.RELAY_WHOLE_MIN ?? 300) * 60_000;
// Written into transcripts, which become a workflow artifact anyone signed in can read.
export const SECRETS = ["CLAUDE_CODE_OAUTH_TOKEN", "RELAY_APP_KEY", "DISPATCH_TOKEN", "RELAY_BOARD_TOKEN"];

export const MODELS = ["haiku", "sonnet", "opus", "fable"];
export const EFFORTS = ["low", "medium", "high"];
// A build step's part in its piece's shape (prompts/spec.md).
export const PARTS = ["ford", "easel", "canvas", "clay", "kiln", "glaze", "magnum"];// The steps that judge, which never run at low effort.
const JUDGES = ["spec-review", "review"];

const SESSION_STOP = 75;
// A whole step starts on a 5-hour window that is all but unused, so the window lasts its session.
export const WHOLE_STOP = 10;
// How long a retro waits for the owner's notes.
export const RETRO_WAIT_MS = 72 * 3600_000;

// ---- Pure logic (covered by relay.test.mjs) ----

// Start a step only while the 5-hour window is under `stop`%, and the weekly window is under
// day-of-cycle / 7 × 95%. A stop says when it lifts, as `until`, where that's known.
// The weekly ramp: the day of the weekly cycle, and the share of the week it allows.
export const ramp = (weeklyResetsAt, now) => {
    const cycleStart = weeklyResetsAt.getTime() - 7 * 86400_000;
    const day = Math.min(7, Math.max(1, Math.floor((now.getTime() - cycleStart) / 86400_000) + 1));
    return { cycleStart, day, weeklyLimit: (day / 7) * 95 };
};
// Both windows as the run log gives them: "5-hour 55%, week 60% of the 67.9% day 5 allows".
export const usageReport = (u, now) => {
    const { day, weeklyLimit } = ramp(u.weeklyResetsAt, now);
    return `5-hour ${Math.round(u.session)}%, week ${Math.round(u.weekly)}% of the ${weeklyLimit.toFixed(1)}% day ${day} allows`;
};

export const pace = ({ session, weekly, weeklyResetsAt, sessionResetsAt }, now, stop = SESSION_STOP) => {
    const { cycleStart, day, weeklyLimit } = ramp(weeklyResetsAt, now);
    const [s, w] = [Math.round(session), Math.round(weekly)];
    if (session >= stop) {
        const resets = sessionResetsAt ? `, resets ${sessionResetsAt.toISOString()}` : "";
        return { ok: false, window: "5-hour", until: sessionResetsAt ?? null, reason: `5-hour usage ${s}% ≥ ${stop}%${resets}` };
    }
    if (weekly >= weeklyLimit) {
        const lifts = Math.floor((weekly * 7) / 95) + 1;
        const until = lifts > 7 ? weeklyResetsAt : new Date(cycleStart + (lifts - 1) * 86400_000);
        return { ok: false, window: "weekly", until, reason: `weekly usage ${w}% ≥ ${weeklyLimit.toFixed(1)}% (day ${day})` };
    }
    return { ok: true, reason: `5-hour ${s}%, weekly ${w}% < ${weeklyLimit.toFixed(1)}% (day ${day})` };
};

// The board's "Waiting on" for a budget stop, in the owner's time: "5-hour usage, until
// 14:20" or "weekly usage, until Thu 09:00".
export const waitingOn = (verdict, timeZone = "Pacific/Auckland") => {
    if (verdict.ok) return "";
    if (!verdict.until) return `${verdict.window} usage`;
    const day = verdict.window === "weekly" ? { weekday: "short" } : {};
    const when = new Intl.DateTimeFormat("en-GB", { ...day, hour: "2-digit", minute: "2-digit", hour12: false, timeZone }).format(verdict.until);
    return `${verdict.window} usage, until ${when}`;
};

// The `<!-- relay {...} -->` block in a step issue's body.
const META = /<!-- relay (\{.*?\}) -->/s;
export const readMeta = (body) => {
    const m = META.exec(body ?? "");
    return m ? JSON.parse(m[1]) : null;
};
export const writeMeta = (body, meta) => {
    // A summary holding "-->" would end the comment early, so it's escaped (still valid JSON).
    const block = `<!-- relay ${JSON.stringify(meta).replaceAll("-->", "--\\u003e")} -->`;
    return META.test(body ?? "") ? body.replace(META, () => block) : `${(body ?? "").trimEnd()}\n\n${block}\n`;
};

// A `**Name:** value` field in a piece's body (bold, list marker and colon placement vary), or
// null. A hard-wrapped value carries on to a blank line, a list item or the next field.
export const pieceField = (body, name) => {
    const lines = (body ?? "").split(/\r?\n/);
    const start = new RegExp(`^[\\s>*-]*${name}[*\\s]*:[*\\s]*(.+)$`, "i");
    const i = lines.findIndex((l) => start.test(l));
    if (i < 0) return null;
    const value = [start.exec(lines[i])[1]];
    for (const l of lines.slice(i + 1)) {
        if (!l.trim() || /^\s*([-*>#]|\d+\.)\s/.test(l) || /^\s*\*\*[^*]+:/.test(l)) break;
        value.push(l.trim());
    }
    return value.join(" ").trim();
};

// `sonnet high, 9 points …: why`: a model, effort and why, or null.
export const readChoice = (line) => {
    const words = (line ?? "").toLowerCase().split(/[^a-z]+/);
    const model = words.find((w) => MODELS.includes(w));
    const effort = words.find((w) => EFFORTS.includes(w));
    return model && effort ? { model, effort, why: line } : null;
};
export const specChoice = (body) => readChoice(pieceField(body, "Spec"));
export const specReviewChoice = (body) => readChoice(pieceField(body, "Spec-review"));

// `**Play:** sonnet, three seeds`: the play step's model, at medium unless the line names an
// effort, and how many seeds it plays; null for `none` or no line.
const COUNTS = { one: 1, two: 2, three: 3, four: 4, five: 5 };
export const playChoice = (body) => {
    const line = pieceField(body, "Play");
    const words = (line ?? "").toLowerCase().split(/[^a-z0-9]+/).filter(Boolean);
    const model = words.find((w) => MODELS.includes(w));
    if (!model || words[0] === "none") return null;
    const i = words.findIndex((w) => /^seeds?$/.test(w));
    const seeds = i > 0 ? Number(words[i - 1]) || COUNTS[words[i - 1]] || 1 : 1;
    return { model, effort: words.find((w) => EFFORTS.includes(w)) ?? "medium", seeds, why: line };
};

// `**Whole:** fable medium`: one session does the whole piece, on this model and effort.
export const wholeChoice = (body) => readChoice(pieceField(body, "Whole"));

// `**After:** #31, #32`: the pieces this one waits to see merged.
export const afterOf = (body) => [...(pieceField(body, "After") ?? "").matchAll(/#(\d+)/g)].map((m) => Number(m[1]));

// Every check on a commit finished and none failed. No checks at all isn't green.
export const checksGreen = (runs) =>
    runs.length > 0 && runs.every((c) => c.status === "completed" && ["success", "neutral", "skipped"].includes(c.conclusion));

// The `docs/design/` files a piece's body names.
export const designPaths = (body) => [...new Set((body ?? "").match(/docs\/design\/[\w./-]+\.md/g) ?? [])];

// A retro's steps, in order: the play check, the three angles that don't need the owner's
// play, the wait for the owner's notes (the crew's, no session), the game and players angles,
// the retro and the proposal.
export const RETRO_STEPS = [
    { id: "check", title: "The milestone play check", model: "sonnet", effort: "medium", why: "Plays main across the retro maps, as a play step does." },
    { id: "ladder", title: "The ladder angle", model: "opus", effort: "medium", why: "Reads every step's cost against the ladder." },
    { id: "architecture", title: "The architecture angle", model: "opus", effort: "medium", why: "Reads the code's shape against docs/architecture.md." },
    { id: "relay", title: "The relay angle", model: "opus", effort: "medium", why: "Reads the crew, the prompts and the transcripts." },
    { id: "wait", title: "The owner's notes", model: "none", effort: "none", why: "Up to 72 hours for the owner's notes, or `/relay retro go`." },
    { id: "game", title: "The game and players angles", model: "fable", effort: "medium", why: "Reads the owner's notes; one session, two files." },
    { id: "retro", title: "The retro", model: "fable", effort: "medium", why: "Reads the angles and writes the retro." },
    { id: "proposal", title: "The next proposal", model: "fable", effort: "medium", why: "Writes the next milestone's proposals from the retro." },
];
const RETRO_IDS = RETRO_STEPS.map((s) => s.id);

// Steps run spec, spec-review, build-1 … build-N, play, review; a retro's run in RETRO_STEPS.
// A whole piece has the one step, `whole`.
export const stepRank = (id) => {
    if (RETRO_IDS.includes(id)) return 2000 + RETRO_IDS.indexOf(id);
    if (id === "spec" || id === "whole") return 0;
    if (id === "spec-review") return 1;
    if (id === "play") return 999;
    if (id === "review") return 1000;
    const m = /^build-(\d+)$/.exec(id);
    return m ? 1 + Number(m[1]) : 500;
};
export const stepKind = (id) => {
    if (id === "wait") return "wait";
    if (RETRO_IDS.includes(id)) return "retro";
    return ["spec", "spec-review", "play", "review", "whole"].includes(id) ? id : "build";
};
// The steps whose `result.json` carries the merge verdict the crew acts on.
const VERDICTS = ["review", "whole"];
// How long a step's session may run, and the 5-hour usage it may start at.
export const capOf = (id) => (stepKind(id) === "whole" ? WHOLE_TIMEOUT_MS : SESSION_TIMEOUT_MS);
export const stopOf = (id) => (stepKind(id) === "whole" ? WHOLE_STOP : SESSION_STOP);
const sortSteps = (steps) => [...steps].sort((a, b) => stepRank(a.meta.id) - stepRank(b.meta.id));

export const branchOf = (piece) => `relay/${piece.number}`;
export const specDir = (piece) => `relay/specs/${piece.number}`;

// steps.json's steps as the crew makes them: the play step goes in before the review.
export const planSteps = (steps, play) => {
    if (!play) return steps;
    return [...steps.slice(0, -1), { id: "play", title: "Play the branch", model: play.model, effort: play.effort, why: `The piece's Play line: ${play.why}` }, steps.at(-1)];
};
// `M4` → `m4`, from the milestone's title; null without one.
export const milestoneTag = (milestone) => {
    const m = /^M(\d+)\b/i.exec(milestone?.title ?? "");
    return m ? `m${m[1]}` : null;
};
export const retroDir = (piece) => `relay/retro/${milestoneTag(piece.milestone) ?? piece.number}`;
export const isRetro = (piece) => (piece.labels ?? []).includes("kind:retro");
export const isFixes = (piece) => (piece.labels ?? []).includes("relay:fixes");

// The steps the spec wrote to `relay/specs/<n>/steps.json`, as spec-review left them,
// checked. Returns { steps } or { error }. A whole step that hands on may leave no builds
// (`builds` 0): the review alone.
export const checkSteps = (text, builds = 1) => {
    let steps;
    try {
        steps = JSON.parse(text);
    } catch (e) {
        return { error: `steps.json isn't JSON: ${e.message}` };
    }
    const ids = Array.isArray(steps) ? steps.map((s) => s?.id) : [];
    if (ids.at(-1) !== "review" || ids.length < builds + 1) {
        return { error: "steps.json must list the build steps, then `review`" };
    }
    for (const [i, s] of steps.slice(0, -1).entries()) {
        if (s.id !== `build-${i + 1}`) return { error: `build step ${i + 1} in steps.json must be \`build-${i + 1}\`, not \`${s.id}\`` };
    }
    for (const s of steps) {
        if (!MODELS.includes(s.model)) return { error: `${s.id}: model must be one of ${MODELS.join(", ")}` };
        if (!EFFORTS.includes(s.effort)) return { error: `${s.id}: effort must be one of ${EFFORTS.join(", ")}` };
        if (s.id !== "review" && (!Number.isInteger(s.points) || s.points < 0 || s.points > 12)) return { error: `${s.id}: points must be a whole number from 0 to 12` };
        if (JUDGES.includes(s.id) && s.effort === "low") return { error: `${s.id}: a step that judges never runs at low effort` };
        if (s.id !== "review" && !PARTS.includes(s.part)) return { error: `${s.id}: part must be one of ${PARTS.join(", ")}` };
        if (!s.title) return { error: `${s.id}: no title` };
        if (!s.why) return { error: `${s.id}: no why for its model and effort` };
    }
    return { steps };
};

// The open pieces a fixes piece waits for: its milestone's others that aren't paused.
export const fixesWaitsFor = (pieces, fixes) =>
    pieces
        .filter((p) => p !== fixes && !isFixes(p) && !isRetro(p) && p.milestone?.number === fixes.milestone?.number && !p.labels.includes("relay:paused"))
        .map((p) => p.number);

// A retro issue's body.
export const retroBody = (milestone) => {
    const m = milestoneTag(milestone);
    const next = `m${Number(m.slice(1)) + 1}`;
    return [
        `**Goal:** ${milestone.title} read back from five angles in four sessions, the retro written from them, and ${next.toUpperCase()} proposed, for the owner to correct in the proposal PR.`,
        "",
        `The angles write \`relay/retro/${m}/\`: \`play.md\` (the check), \`ladder.md\`, \`architecture.md\`, \`relay.md\`, \`owner.md\`, \`game.md\` and \`players.md\`. ` +
            `The retro is \`docs/proposals/${m}-retro.md\`, and the proposal \`docs/proposals/${next}.md\` and \`${next}-relay.md\`.`,
    ].join("\n");
};

// Why a piece can't run now, or null when it can.
export const holdReason = (piece) => {
    if (!piece.labels.includes("relay:ready")) return "not ready";
    // Anyone can open an issue, but only the owner's and the crew's words reach a session.
    if (piece.author !== OWNER && piece.author !== BOT) return "not opened by the owner";
    if (piece.labels.includes("relay:paused")) return "paused";
    if (piece.labels.includes("waiting")) return "waiting on the owner";
    const unmerged = (piece.after ?? []).filter((a) => !a.merged).map((a) => `#${a.number}`);
    if (unmerged.length) return `after ${unmerged.join(", ")}, not merged yet`;
    if (piece.before?.length) return `after the milestone's other pieces (${piece.before.map((n) => `#${n}`).join(", ")})`;
    if (piece.steps.length === 0 && !isRetro(piece) && !wholeChoice(piece.body)) {
        if (!specChoice(piece.body)) return "the body has no `Spec:` model and effort";
        if (!specReviewChoice(piece.body)) return "the body has no `Spec-review:` model and effort";
    }
    return null;
};

// Whether a retro's wait is over: the owner's notes or `/relay retro go` came, or 72 hours
// passed since the step before it closed.
export const waitOver = (steps, wait, now) => {
    if (wait.meta.notes || wait.meta.go) return true;
    const before = sortSteps(steps).filter((s) => stepRank(s.meta.id) < stepRank("wait")).at(-1);
    return Boolean(before?.closedAt) && now - new Date(before.closedAt) >= RETRO_WAIT_MS;
};

// What the crew does next for one piece: "start" (it has no steps yet), "step" (run
// `step`), "plan" (the spec or spec-review is done and the next steps aren't made), "waited"
// (a retro's wait is over), or null.
export const pieceAction = (piece, now = new Date()) => {
    if (piece.steps.length === 0) return { action: "start" };
    const steps = sortSteps(piece.steps);
    const open = steps.find((s) => s.state === "open");
    if (open?.meta.id === "wait") return waitOver(steps, open, now) ? { action: "waited", step: open } : null;
    if (open) return open.meta.status === "blocked" ? null : { action: "step", step: open };
    if (steps.every((s) => s.meta.id === "spec" || s.meta.id === "spec-review")) return { action: "plan" };
    // A whole step that handed on, whose steps aren't made yet.
    if (steps.length === 1 && steps[0].meta.id === "whole" && steps[0].meta.handed) return { action: "plan" };
    return null;
};

// The next piece and what to do with it, oldest first.
export const nextWork = (pieces, now = new Date()) => {
    for (const piece of [...pieces].sort((a, b) => a.number - b.number)) {
        if (holdReason(piece)) continue;
        const next = pieceAction(piece, now);
        if (next) return { piece, ...next };
    }
    return null;
};

// Markdown headings outside code fences move so the first sits at `level`, so quoted
// material nests under the prompt's own top-level sections.
export const atLevel = (md, level = 2) => {
    const first = /^(#{1,6})\s/m.exec(md);
    const by = first ? level - first[1].length : 0;
    let fenced = false;
    return md
        .split("\n")
        .map((line) => {
            if (/^\s*(```|~~~)/.test(line)) fenced = !fenced;
            const m = !fenced && /^(#{1,6})(\s.*)$/.exec(line);
            return m ? "#".repeat(Math.min(6, Math.max(1, m[1].length + by))) + m[2] : line;
        })
        .join("\n");
};

// A finished step's summary and decisions, as later steps see them. The spec's and
// spec-review's decisions are in spec.md, so only their summaries come here.
export const soFar = (s) => {
    const own = !["spec", "spec-review"].includes(s.meta.id);
    const decided = (own ? s.meta.decisions ?? [] : []).map((d) => `\n  - Decided: ${d.decision}${d.why ? ` (${d.why})` : ""}`).join("");
    return `- **${s.meta.id}**: ${s.meta.summary}${decided}`;
};

// The `relay/prompts/` files each kind of step follows, besides `session.md`; a name with a
// slash is under `relay/`.
export const PROMPTS_FOR = {
    spec: ["spec"],
    "spec-review": ["spec-review", "spec"],
    build: ["build"],
    play: ["play", "players/player", "players/judge"],
    review: ["review", "players/judge"],
    whole: ["whole"],
    retro: ["retro"],
};
// A step's prompt files: the retro's play check plays as a play step does.
export const promptsFor = (id) => (id === "check" ? [...PROMPTS_FOR.retro, ...PROMPTS_FOR.play] : PROMPTS_FOR[stepKind(id)]);
export const promptPath = (name) => (name.includes("/") ? `relay/${name}.md` : `relay/prompts/${name}.md`);

// A session's whole prompt: its step first, then only the material that serves it, then
// the rules for its kind of step. Throws when a build step has no brief; the crew blocks
// the step on that.
// `files` holds `prompts` (name → text), `brief` and `spec` (null when missing), `design`
// (path → text, null for a named file that doesn't exist), `designFiles` (every path in
// docs/design/), `diffStat` (review only), `lastSpec` (spec only: the folder of the latest
// other piece's spec, or null), `notes` (the retro's game step: the owner's notes, or null),
// `started` and `cap` (when the session started, and the minutes it may run), `usage` (the
// windows as the session starts, or null) and `ideas` (the milestone's open ideas, or null).
export const sessionPrompt = (piece, step, files) => {
    const kind = stepKind(step.meta.id);
    const retro = kind === "retro";
    const id = step.meta.id;
    const where = retro ? `the milestone's retro files live in \`${retroDir(piece)}/\`` : `its spec lives in \`${specDir(piece)}/\``;
    const clock = files.started
        ? ` The session started at ${files.started}, and the crew stops it after ${files.cap} minutes: check the clock (\`date -u\`) before giving up on time.`
        : "";
    const used = files.usage
        ? ` ${usageLine(files.usage)} as you start, and a hook tells you both again every ${USAGE_EVERY_MIN} minutes.`
        : "";
    const parts = [
        `You are a Relay session for Sans Fiat, in GitHub Actions with no other context: one cold session ` +
            `doing one step of ${retro ? "retro" : "piece"} #${piece.number}, "${piece.title}". You are on its branch, ` +
            `\`${branchOf(piece)}\`, and ${where}. This prompt holds how Relay works; the design and the code hold the rest.${clock}${used}`,
    ];
    if (kind === "build") {
        if (!files.brief) throw new Error(`${specDir(piece)}/${id}.md is missing`);
        parts.push(atLevel(files.brief.trim(), 1));
    } else if (retro) {
        const s = RETRO_STEPS.find((r) => r.id === step.meta.id);
        parts.push(`# Your step: #${piece.number} ${step.meta.id}\n\n${s.title}, as \`retro.md\` says for the \`${s.id}\` step. The retro:`);
        parts.push(atLevel(`# The retro\n\n${piece.body.trim()}`));
        if (step.meta.id === "game") {
            parts.push(
                files.notes
                    ? `# The owner's notes\n\nWrite them to \`${retroDir(piece)}/owner.md\` as they are, then read them as the angles' evidence.\n\n"""${files.notes}"""`
                    : `# The owner's notes\n\nNone came in 72 hours: the angles go without them, and say so.`,
            );
        }
    } else {
        const what = {
            spec: "Write the spec, the briefs and steps.json for this piece",
            "spec-review": "Review the spec, the briefs and steps.json for this piece",
            play: `Play this piece's branch and write its play note, on ${playChoice(piece.body)?.seeds ?? 1} seed${(playChoice(piece.body)?.seeds ?? 1) > 1 ? "s" : ""}`,
            review: "Review this piece as built",
            whole: "Do this whole piece in this one session, from its plan to its merge verdict",
        }[kind];
        const like = kind === "spec" && files.lastSpec ? ` Write \`spec.md\` and the briefs as \`${files.lastSpec}/\` did.` : "";
        parts.push(`# Your step: #${piece.number} ${step.meta.id}\n\n${what}, as agreed with the owner.${like} The piece:`);
        parts.push(atLevel(`# The piece\n\n${piece.body.trim()}`));
    }
    if (step.meta.note) parts.push(`# The last attempt didn't finish\n\nPick up from here.\n\n${step.meta.note}`);
    if (step.meta.reply) {
        parts.push(
            `# The owner's reply to this step's blocker\n\nThe earlier attempt's commits are on the branch. Act on the reply for this step, ` +
                `\`${id}\`, only: a later step is another session's, even if the reply says to carry on.\n\n"""${step.meta.reply}"""`,
        );
    }
    if (!["build", "play", "retro"].includes(kind)) {
        const named = Object.entries(files.design ?? {});
        const builds = named.length
            ? named.map(([f, text]) => `## \`${f}\`\n\n${text ? atLevel(text.trim(), 3) : "That file doesn't exist yet."}`).join("\n\n")
            : "The piece names no design file.";
        const others = (files.designFiles ?? []).filter((f) => !(f in (files.design ?? {})));
        const rest = others.length ? `\n\nThe other design files, to read as you need them:\n\n${others.map((f) => `- \`${f}\``).join("\n")}` : "";
        parts.push(`# The design\n\n${builds}${rest}`);
    }
    if (kind === "review") {
        parts.push(`# The diff to review\n\nThe piece's changes against \`main\`:\n\n\`\`\`\n${(files.diffStat ?? "").trim()}\n\`\`\``);
        if (files.spec) parts.push(`# The whole spec\n\n${atLevel(files.spec.trim())}`);
        parts.push(`# The steps\n\nPut this table in the PR description under the judge's block. The crew rewrites it with your own row after you finish:\n\n${stepsTable(piece.steps)}`);
    }
    if (["spec", "whole", "play"].includes(kind) && files.ideas) {
        const what =
            kind === "play"
                ? "Sessions before you noticed these. A finding that repeats one adds nothing: leave it out."
                : "Sessions before you noticed these. Take into this piece each one that is obvious to do within its goal, and name its title under `took` in `result.json`.";
        parts.push(`# The milestone's open ideas\n\n${what}\n\n${files.ideas}`);
    }
    // The player plays as a player: the piece says what is new, and how it was built isn't its to know.
    const done = kind === "play" ? [] : sortSteps(piece.steps).filter((s) => s.state === "closed" && s.meta.summary);
    if (done.length) parts.push(`# This piece so far\n\n${done.map(soFar).join("\n")}`);
    for (const name of [...promptsFor(step.meta.id), "session"]) {
        if (files.prompts[name]) parts.push(atLevel(files.prompts[name].trim(), 1));
    }
    return parts.join("\n\n") + "\n";
};

// The CLI isn't pinned, so each run records the version it got.
let cli;
const cliVersion = () => {
    if (cli === undefined) {
        try {
            cli = execFileSync("claude", ["--version"], { encoding: "utf8" }).trim();
        } catch {
            cli = "(unknown version)";
        }
    }
    return cli;
};

// The session's init frame says what it really got. A model the user's default overrode,
// or a session without auto mode, can't do the step.
export const initProblem = (frame, model) => {
    if (!frame) return `Claude Code ${cliVersion()} sent no init frame: its stream-json may have changed`;
    if (!frame.model?.includes(model)) return `model mismatch: wanted ${model}, got ${frame.model}`;
    if (frame.permissionMode !== "auto") return `permission mode is ${frame.permissionMode}, not auto`;
    return null;
};

// What a session used: its cost at list price (the CLI's own figure), the context its last
// turn held as a share of the step model's window, and how many times it compacted. A count
// above 0 says the step was too big.
export const sessionStats = (lastAssistant, result, model, compactions = 0) => {
    const usage = result?.modelUsage ?? {};
    const u = lastAssistant?.message?.usage;
    const held = u ? (u.input_tokens ?? 0) + (u.cache_creation_input_tokens ?? 0) + (u.cache_read_input_tokens ?? 0) + (u.output_tokens ?? 0) : 0;
    const window = Object.entries(usage).find(([k]) => k.includes(model))?.[1]?.contextWindow ?? 200_000;
    return { cost: result?.total_cost_usd ?? 0, context: Math.round((held / window) * 100), compactions };
};

// Whether the usage limit ended the session: its result frame is an error that says so.
export const hitLimit = (result) => Boolean(result?.is_error) && /usage[_ ]limit|rate_limit|\b429\b/i.test(JSON.stringify(result));

export const dollars = (n) => `$${(n ?? 0).toFixed(2)}`;
export const cents = (n) => Math.round((n ?? 0) * 100) / 100;

// The usage points a step took: the rise in each window from before the session to after.
// A window that reset in between counts what was used since the reset, a floor.
export const usagePoints = (before, after) => {
    const rise = (a, b) => (a == null || b == null ? null : Math.round(b < a ? b : b - a));
    return { fiveHour: rise(before?.session, after?.session), weekly: rise(before?.weekly, after?.weekly) };
};

// A finished or stopped step's one-line status, at the top of its issue's body.
export const statusLine = (meta, outcome, note) => {
    const spent = `${dollars(meta.cost)}, ${Math.round(meta.minutes ?? 0)} minutes`;
    if (outcome === "done") return `done, ${spent}`;
    if (outcome === "blocked") return `blocked: ${note}`;
    return `not finished (attempt ${meta.attempts} of ${MAX_ATTEMPTS}), ${spent}: ${note}`;
};
const STATUS = /^\*\*Status:\*\*.*$/m;
export const withStatus = (body, line) => (STATUS.test(body ?? "") ? body.replace(STATUS, () => `**Status:** ${line}`) : `**Status:** ${line}\n\n${body ?? ""}`);

// A step issue's body: its status, what it runs on and why, its links, and its brief as it
// was when the step was made.
export const stepBody = (piece, s, { brief, pr } = {}) => {
    const blob = `https://github.com/${GH_REPO}/blob/${branchOf(piece)}/${specDir(piece)}`;
    const on = `**${s.model} ${s.effort}**${s.points != null ? `, ${s.points} points` : ""}${s.part ? `, ${s.part}` : ""}`;
    const links = [`Piece #${piece.number}`, ...(pr ? [`PR #${pr}`] : []), `[spec.md](${blob}/spec.md)`, ...(brief ? [`[the brief](${blob}/${s.id}.md)`] : [])];
    const parts = [`**Status:** ready`, `Step \`${s.id}\` on ${on}. ${s.why ?? ""}`.trim(), links.join(" · ")];
    if (brief) {
        const text = brief.trim();
        const cut = text.length > 55_000 ? `${text.slice(0, 55_000)}\n\n…cut here; the rest is in the brief on the branch.` : text;
        parts.push(`---\n\nThe brief as it was when this step was made. The file on the branch is the one builds keep up.\n\n${atLevel(cut, 2)}`);
    }
    return parts.join("\n\n");
};

// A finished step's findings and ideas, sorted. A `fix` needs a repro and a milestone with a
// fixes piece to go to, and never comes from a retro; everything else is `ask`.
export const sortFindings = (result, { retro = false, milestone = null } = {}) => {
    const items = [...(Array.isArray(result?.findings) ? result.findings : []), ...(Array.isArray(result?.ideas) ? result.ideas : [])].filter((i) => i?.title);
    const fix = (i) => i.class === "fix" && i.repro && !retro && milestone;
    return { fix: items.filter(fix), ask: items.filter((i) => !fix(i)) };
};

// A finding as a line of the fixes piece or of the ideas issue.
export const findingText = (i, from) => `**${i.title}**${i.body ? `: ${i.body}` : ""}${i.repro ? ` Repro: ${i.repro}` : ""} (${from})`;

// The one issue that holds a milestone's ideas, a line each, ticked when a piece takes one.
export const ideasTitle = (milestone) => (milestone ? `Ideas from ${milestone.title}` : "Ideas");
export const IDEAS_BODY =
    "What sessions noticed and didn't build, one line each. A designer takes the obvious ones into its piece, which ticks them; the retro's proposal takes or drops the rest.\n";
const IDEA = /^- \[( |x)\] \*\*(.+?)\*\*/;
// The ideas issue's body with `ideas` added as open lines; a title already there is skipped.
export const withIdeas = (body, ideas, from) => {
    const have = new Set((body ?? "").split("\n").map((l) => IDEA.exec(l)?.[2]).filter(Boolean));
    const lines = ideas.filter((i) => !have.has(i.title) && have.add(i.title)).map((i) => `- [ ] ${findingText(i, from)}`);
    return lines.length ? `${(body ?? "").trimEnd()}\n${lines.join("\n")}\n` : body;
};
// The body with each idea a session `took` ticked.
export const tickIdeas = (body, took, from) =>
    (body ?? "")
        .split("\n")
        .map((l) => {
            const m = IDEA.exec(l);
            return m && m[1] === " " && took.includes(m[2]) ? `${l.replace("- [ ]", "- [x]")} Taken by ${from}.` : l;
        })
        .join("\n");
// The open lines, for a designer's prompt; null when there are none.
export const openIdeas = (body) => (body ?? "").split("\n").filter((l) => IDEA.exec(l)?.[1] === " ").join("\n") || null;

// `usage` as the words a session's prompt gives.
export const usageLine = (u) => `The 5-hour usage window stands at ${Math.round(u.session)}% and the week at ${Math.round(u.weekly)}%`;

// The proposed issues a merged proposal settles: each `#n` on a line of its `## Ideas`
// section that says taken or dropped.
export const settledIdeas = (md) => {
    const section = /^## Ideas\s*$([\s\S]*?)(?=^## |(?![\s\S]))/m.exec(md ?? "")?.[1] ?? "";
    return section
        .split("\n")
        .filter((l) => /\b(taken|dropped)\b/i.test(l))
        .flatMap((l) => [...l.matchAll(/#(\d+)/g)].map((m) => ({ number: Number(m[1]), line: l.replace(/^[\s|*-]+/, "").trim() })));
};

// The fixes piece a milestone's `fix` findings go to.
export const fixesBody = (milestone) =>
    [
        `**Goal:** Every \`fix\` finding of ${milestone} below works as the line it names says, and its repro is a test on the stage that fails on main and passes here.`,
        "",
        "**Spec:** sonnet medium: each fix restores a line the design already fixes",
        "",
        "**Spec-review:** sonnet medium",
        "",
        "**Play:** none",
        "",
        "**Shape:** Henry Ford: every finding is settled by the line it names.",
        "",
        "**In scope:** the findings below, and a test for each.",
        "",
        "**Out of scope:** a new or changed rule, design prose, a new lever, a band, or a `data.rs` number a design line doesn't already fix.",
        "",
        "## Findings",
        "",
    ].join("\n");

// The steps table in a PR description replaced with `table`, or put under its "Closes" line.
export const withTable = (body, table) => {
    const lines = (body ?? "").split("\n");
    const start = lines.findIndex((l) => l.startsWith("| Step | State"));
    const end = lines.findIndex((l, i) => i > start && /at list price so far\.$/.test(l));
    if (start >= 0 && end >= 0) return [...lines.slice(0, start), table, ...lines.slice(end + 1)].join("\n");
    const closes = lines.findIndex((l) => /Closes #\d+/.test(l));
    return [...lines.slice(0, closes + 1), "", table, "", ...lines.slice(closes + 1)].join("\n");
};

// A milestone is built when none of its pieces can run: every open one is paused.
export const isBuilt = (open) => open.every((p) => p.labels.includes("relay:paused"));

// The pieces in a proposal: each `### X. Title` section with a `**Goal:**` line, as
// { key, title, body }, in order. `#X` in an After line names another piece by its key.
export const parseProposal = (md) => {
    const pieces = [];
    let cur = null;
    for (const line of (md ?? "").split(/\r?\n/)) {
        const h = /^###\s+([A-Z][A-Za-z0-9]*)\.\s+(.+)$/.exec(line);
        if (h || /^##?\s/.test(line)) {
            if (cur) pieces.push(cur);
            cur = h ? { key: h[1], title: h[2].trim(), lines: [] } : null;
            continue;
        }
        if (cur) cur.lines.push(line);
    }
    if (cur) pieces.push(cur);
    return pieces
        .map(({ key, title, lines }) => ({ key, title, body: lines.join("\n").trim() }))
        .filter((p) => pieceField(p.body, "Goal"));
};

// A proposal piece's body with `#X` keys in its After line made issue numbers.
export const withNumbers = (body, numbers) =>
    body.replace(/^(\*\*After:\*\*.*)$/m, (line) => line.replace(/#([A-Z][A-Za-z0-9]*)\b/g, (m, k) => (numbers[k] ? `#${numbers[k]}` : m)));

// How a session ended: what it wrote to result.json, or null when it wrote nothing usable.
export const readResult = (text) => {
    if (text == null) return null;
    try {
        const r = JSON.parse(text);
        if (!["done", "pending", "blocked"].includes(r.status)) return null;
        if (r.status === "blocked" && !r.blocker) return null;
        return r;
    } catch {
        return null;
    }
};

// What a session's outcome does to its step's meta. Returns { meta, outcome }, where outcome
// is "done", "pending" or "blocked"; `why` says what blocked it.
export const applyResult = (meta, result, exitOk) => {
    const next = { ...meta };
    // A session that crashed may not have read the reply, so the next one gets it again.
    if (exitOk) delete next.reply;
    const status = exitOk ? result?.status ?? "pending" : "pending";
    if (status === "done") {
        delete next.note;
        next.status = "done";
        if (result.summary) next.summary = result.summary;
        const decisions = (Array.isArray(result.decisions) ? result.decisions : []).filter((d) => d?.decision);
        if (decisions.length) next.decisions = decisions.map((d) => ({ decision: String(d.decision), why: String(d.why ?? "") }));
        return { meta: next, outcome: "done" };
    }
    if (status === "blocked") {
        next.status = "blocked";
        return { meta: next, outcome: "blocked", why: result.blocker };
    }
    next.attempts = (next.attempts ?? 0) + 1;
    next.note = exitOk ? result?.note ?? "The session ended without saying where it got to." : "The session crashed, timed out or wrote no result.";
    if (next.attempts < MAX_ATTEMPTS) {
        next.status = "pending";
        return { meta: next, outcome: "pending" };
    }
    next.status = "blocked";
    return { meta: next, outcome: "blocked", why: `it ended ${next.attempts} times without finishing. Last: ${next.note}` };
};

// A `/relay …` command in a comment, or null.
export const parseCommand = (body) => {
    const m = /^\s*\/relay\s+(\S+)(.*)$/m.exec(body ?? "");
    if (!m) return null;
    return { name: m[1].toLowerCase(), args: m[2].trim().split(/\s+/).filter(Boolean) };
};

// `/relay model build-2 opus high`: applies the model and effort named after the step.
export const applyModel = (meta, args) => {
    const words = args.map((a) => a.toLowerCase());
    const model = words.findLast((w) => MODELS.includes(w));
    const effort = words.findLast((w) => EFFORTS.includes(w));
    if (!model && !effort) return null;
    return { ...meta, model: model ?? meta.model, effort: effort ?? meta.effort };
};

// The steps table for `/relay status` and the PR description, with the piece's total cost.
export const stepsTable = (steps) => {
    const sorted = sortSteps(steps);
    const total = sorted.reduce((n, s) => n + (s.meta.cost ?? 0), 0);
    const num = (n, f = String) => (n != null ? f(n) : "");
    return [
        "| Step | State | Part | Model | Effort | Points | Sessions | List $ | Minutes | 5-hour | Weekly |",
        "|---|---|---|---|---|---|---|---|---|---|---|",
        ...sorted.map((s) => {
            const m = s.meta;
            const state = s.state === "closed" ? (s.stateReason === "not_planned" ? "skipped" : "done") : m.status ?? "pending";
            return `| #${s.number} ${m.id} | ${state} | ${m.part ?? ""} | ${m.model} | ${m.effort} | ${num(m.points)} | ${m.sessions ?? m.attempts ?? 0} | ${num(m.cost, dollars)} | ${num(m.minutes, Math.round)} | ${num(m.fiveHour)} | ${num(m.weekly)} |`;
        }),
        "",
        `${dollars(total)} at list price so far.`,
    ].join("\n");
};

// GitHub tokens (an installation token's `ghs_`, a classic PAT's `ghp_`, a fine-grained
// `github_pat_`), whether or not they're in `secrets`: a session can read the token file.
const GITHUB_TOKEN = /\b(gh[pousr]_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{30,})\b/g;

export const redact = (text, secrets) =>
    secrets
        .filter((s) => s && s.length >= 8)
        .reduce((t, s) => t.split(s).join("***"), text)
        .replace(GITHUB_TOKEN, "***");

// ---- Side effects ----

const log = (msg) => {
    const line = `[${new Date().toISOString()}] ${msg}`;
    console.log(line);
    fs.mkdirSync(RUNS, { recursive: true });
    fs.appendFileSync(path.join(RUNS, "crew.log"), line + "\n");
};

const sh = (cmd, args, opts = {}) =>
    execFileSync(cmd, args, { cwd: REPO, encoding: "utf8", stdio: ["ignore", "pipe", "pipe"], ...opts }).trim();
const git = (...args) => sh("git", args);
const secrets = () => SECRETS.map((k) => process.env[k]);

const token = () => appToken(GH_REPO);

const request = async (method, url, body, auth) => {
    const res = await fetch(url, {
        method,
        headers: {
            Accept: "application/vnd.github+json",
            Authorization: `Bearer ${auth ?? (await token())}`,
            ...(body ? { "Content-Type": "application/json" } : {}),
        },
        body: body ? JSON.stringify(body) : undefined,
    });
    if (!res.ok) throw new Error(`GitHub ${method} ${url}: HTTP ${res.status} ${await res.text()}`);
    return res.status === 204 ? null : res.json();
};

// A route starting `/` is the repo's; anything else is the API root's.
const github = (method, route, body, auth) =>
    request(method, `https://api.github.com/${route.startsWith("/") ? `repos/${GH_REPO}${route}` : route}`, body, auth);

const graphql = async (query, variables, auth) => {
    const res = await request("POST", "https://api.github.com/graphql", { query, variables }, auth);
    if (res.errors) throw new Error(`GraphQL: ${res.errors.map((e) => e.message).join("; ")}`);
    return res.data;
};

const comment = (issue, body) => github("POST", `/issues/${issue}/comments`, { body });
const addLabel = (issue, label) => github("POST", `/issues/${issue}/labels`, { labels: [label] });
const removeLabel = (issue, label) =>
    github("DELETE", `/issues/${issue}/labels/${encodeURIComponent(label)}`).catch(() => null);

const allPages = async (route) => {
    const items = [];
    for (let page = 1; ; page++) {
        const batch = await github("GET", `${route}${route.includes("?") ? "&" : "?"}per_page=100&page=${page}`);
        items.push(...batch);
        if (batch.length < 100) return items;
    }
};

const toStep = (issue) => ({
    number: issue.number,
    id: issue.id,
    nodeId: issue.node_id,
    title: issue.title,
    state: issue.state,
    stateReason: issue.state_reason,
    closedAt: issue.closed_at,
    body: issue.body ?? "",
    meta: readMeta(issue.body) ?? { id: "?" },
});

// A piece done by hand has no branch, so closing it counts as merged.
const isMerged = async (number) => {
    const prs = await github("GET", `/pulls?state=all&head=${OWNER}:${encodeURIComponent(`relay/${number}`)}`);
    if (prs.length) return prs.some((p) => p.merged_at);
    return (await github("GET", `/issues/${number}`)).state === "closed";
};

const toPiece = async (issue) => ({
    number: issue.number,
    nodeId: issue.node_id,
    title: issue.title,
    body: issue.body ?? "",
    author: issue.user.login,
    labels: issue.labels.map((l) => l.name),
    milestone: issue.milestone,
    after: await Promise.all(afterOf(issue.body).map(async (number) => ({ number, merged: await isMerged(number) }))),
    steps: (await allPages(`/issues/${issue.number}/sub_issues`)).filter((s) => readMeta(s.body)).map(toStep),
});

const loadPieces = async () => {
    const issues = [...(await allPages("/issues?state=open&labels=kind:piece")), ...(await allPages("/issues?state=open&labels=kind:retro"))];
    const pieces = await Promise.all(issues.filter((i) => !i.pull_request).map(toPiece));
    for (const p of pieces.filter(isFixes)) p.before = fixesWaitsFor(pieces, p);
    return pieces;
};

const loadPiece = async (number) => toPiece(await github("GET", `/issues/${number}`));

const saveMeta = async (step, meta) => {
    step.body = writeMeta(step.body, meta);
    step.meta = meta;
    await github("PATCH", `/issues/${step.number}`, { body: step.body });
};

const closeStep = (step, reason = "completed") => github("PATCH", `/issues/${step.number}`, { state: "closed", state_reason: reason });

// Step issues carry no comments: how a step stands is the line at the top of its body.
const setStepStatus = async (step, line) => {
    step.body = withStatus(step.body, line);
    await github("PATCH", `/issues/${step.number}`, { body: step.body });
};

// ---- The board (best effort: the crew works without it) ----

let board;
const BOARD_TOKEN = process.env.RELAY_BOARD_TOKEN || undefined;

const loadBoard = async () => {
    if (board !== undefined) return board;
    if (!BOARD_TOKEN) {
        log("the board won't be updated this run: RELAY_BOARD_TOKEN isn't set, and the App can't reach a personal account's project");
        return (board = null);
    }
    try {
        const data = await graphql(
            `query($login: String!, $q: String!) { user(login: $login) { projectsV2(first: 5, query: $q) { nodes {
                id title fields(first: 50) { nodes {
                    ... on ProjectV2Field { id name dataType }
                    ... on ProjectV2SingleSelectField { id name dataType options { id name } } } } } } } }`,
            { login: OWNER, q: BOARD },
            BOARD_TOKEN,
        );
        const project = data.user.projectsV2.nodes.find((p) => p.title === BOARD);
        if (!project) throw new Error(`no project called "${BOARD}"`);
        board = { id: project.id, fields: Object.fromEntries(project.fields.nodes.filter((f) => f.name).map((f) => [f.name, f])) };
    } catch (e) {
        log(`the board isn't reachable, so it won't be updated this run: ${e.message}`);
        board = null;
    }
    return board;
};

// Sets fields on an issue's board item, adding the item first. `values` maps a field name to
// an option name, a number or text; "" clears the field. Each field is set on its own, Status
// first, so one that fails doesn't stop the rest.
const setBoard = async (nodeId, values) => {
    const b = await loadBoard();
    if (!b) return;
    let item;
    try {
        const added = await graphql(
            `mutation($p: ID!, $c: ID!) { addProjectV2ItemById(input: { projectId: $p, contentId: $c }) { item { id } } }`,
            { p: b.id, c: nodeId },
            BOARD_TOKEN,
        );
        item = added.addProjectV2ItemById.item.id;
    } catch (e) {
        return log(`couldn't add an item to the board: ${e.message}`);
    }
    const names = Object.keys(values).sort((x, y) => (y === "Status") - (x === "Status"));
    for (const name of names) {
        const field = b.fields[name];
        const value = values[name];
        if (!field || value == null) continue;
        try {
            if (value === "") {
                await graphql(
                    `mutation($p: ID!, $i: ID!, $f: ID!) { clearProjectV2ItemFieldValue(input: { projectId: $p, itemId: $i, fieldId: $f }) { projectV2Item { id } } }`,
                    { p: b.id, i: item, f: field.id },
                    BOARD_TOKEN,
                );
                continue;
            }
            let v;
            if (field.options) {
                const option = field.options.find((o) => o.name.toLowerCase() === String(value).toLowerCase());
                if (!option) continue;
                v = { singleSelectOptionId: option.id };
            } else if (field.dataType === "NUMBER") {
                v = { number: Number(value) };
            } else if (field.dataType === "TEXT") {
                v = { text: String(value) };
            } else continue;
            await graphql(
                `mutation($p: ID!, $i: ID!, $f: ID!, $v: ProjectV2FieldValue!) {
                    updateProjectV2ItemFieldValue(input: { projectId: $p, itemId: $i, fieldId: $f, value: $v }) { projectV2Item { id } } }`,
                { p: b.id, i: item, f: field.id, v },
                BOARD_TOKEN,
            );
        } catch (e) {
            log(`couldn't set ${name} on the board: ${e.message}`);
        }
    }
};

const setStatus = (piece, status, waiting = "") => setBoard(piece.nodeId, { Status: status, "Waiting on": waiting });

// ---- Git ----

// The owner may push to a piece's branch while a run is going, so every push rebases first.
const push = (branch) => {
    for (let i = 1; ; i++) {
        try {
            git("pull", "--rebase", "origin", branch);
            git("push", "origin", `HEAD:${branch}`);
            return;
        } catch (e) {
            try {
                git("rebase", "--abort");
            } catch {
                // no rebase in progress
            }
            if (i >= 3) throw e;
        }
    }
};

// Checks out the piece's branch, making it from main if it doesn't exist yet.
const checkoutPiece = (piece) => {
    const branch = branchOf(piece);
    git("fetch", "-q", "origin", "main");
    let exists = true;
    try {
        git("fetch", "-q", "origin", branch);
    } catch {
        exists = false;
    }
    git("checkout", "-q", "-B", branch, exists ? `origin/${branch}` : "origin/main");
    if (!exists) {
        git("push", "-q", "origin", `HEAD:${branch}`);
        // So a session's plain `git push` goes to the piece's branch, never main.
        git("branch", "-q", "-u", `origin/${branch}`);
    }
    return branch;
};

// Whatever the session left behind: unpushed commits are pushed if they rebase cleanly,
// anything uncommitted is dropped. The runner is thrown away at the end of the job.
const tidyUp = (branch) => {
    if (git("status", "--porcelain")) {
        log("session left uncommitted changes; dropping them");
        git("reset", "--hard");
        git("clean", "-fdq", "-e", "relay/runs/");
    }
    if (git("rev-list", `origin/${branch}..HEAD`)) {
        try {
            push(branch);
        } catch (e) {
            log(`could not push the session's commits, dropping them: ${e.message}`);
            git("reset", "--hard", `origin/${branch}`);
        }
    }
};

const readRepo = (f) => (fs.existsSync(path.join(REPO, f)) ? fs.readFileSync(path.join(REPO, f), "utf8") : null);

// ---- The session ----

// The token sessions use for git and `gh`, rewritten as the crew's token is renewed, so a
// session never holds the App's key and never outlives its token.
const TOKEN_FILE = path.join(process.env.RUNNER_TEMP ?? os.tmpdir(), "relay-token");
const BIN = path.join(process.env.RUNNER_TEMP ?? os.tmpdir(), "relay-bin");

const writeTokenFile = async () => fs.writeFileSync(TOKEN_FILE, await token(), { mode: 0o600 });

// Points git and `gh` at the token file, and commits as the App's bot.
const setUpIdentity = async () => {
    await writeTokenFile();
    const ghPath = sh("bash", ["-c", "command -v gh"]);
    fs.mkdirSync(BIN, { recursive: true });
    fs.writeFileSync(path.join(BIN, "gh"), `#!/bin/sh\nGH_TOKEN="$(cat '${TOKEN_FILE}')" exec '${ghPath}' "$@"\n`, { mode: 0o755 });
    git("config", "credential.helper", `!f() { echo username=x-access-token; echo "password=$(cat '${TOKEN_FILE}')"; }; f`);
    const bot = await github("GET", "users/sansfiat-relay%5Bbot%5D");
    git("config", "user.name", "sansfiat-relay[bot]");
    git("config", "user.email", `${bot.id}+sansfiat-relay[bot]@users.noreply.github.com`);
};

// The prompt goes on stdin and is saved beside the transcript.
const runSession = (piece, step, prompt) =>
    new Promise((resolve) => {
        fs.mkdirSync(RUNS, { recursive: true });
        fs.rmSync(RESULT, { force: true });
        const stamp = new Date().toISOString().replace(/[:.]/g, "-");
        const name = `${stamp}-${piece.number}-${step.meta.id}`;
        fs.writeFileSync(path.join(RUNS, `${name}.prompt.md`), prompt);
        const out = fs.createWriteStream(path.join(RUNS, `${name}.jsonl`));
        const hidden = secrets();
        const env = { ...process.env, PATH: `${BIN}:${process.env.PATH}` };
        delete env.RELAY_APP_KEY;
        delete env.DISPATCH_TOKEN;
        delete env.RELAY_BOARD_TOKEN;
        const child = spawn(
            "claude",
            [
                "-p",
                "--model", step.meta.model,
                "--effort", step.meta.effort,
                "--permission-mode", "auto",
                "--output-format", "stream-json",
                "--verbose",
                // The hook tells the session its usage windows as it works (usage-hook.mjs).
                "--settings", JSON.stringify({ hooks: { PostToolUse: [{ hooks: [{ type: "command", command: `node "${path.join(HERE, "usage-hook.mjs")}"`, timeout: 20 }] }] } }),
            ],
            { cwd: REPO, env, shell: false, stdio: ["pipe", "pipe", "pipe"] },
        );
        child.stdin.end(prompt);
        const began = Date.now();
        let model = null;
        let refused = null;
        let timedOut = false;
        let lastAssistant = null;
        let result = null;
        let compactions = 0;
        const renew = setInterval(() => writeTokenFile().catch((e) => log(`couldn't renew the token: ${e.message}`)), 5 * 60_000);
        const cap = capOf(step.meta.id);
        const timer = setTimeout(() => {
            timedOut = true;
            log(`#${piece.number} ${step.meta.id} ran past ${cap / 60_000} minutes; stopping it`);
            child.kill();
        }, cap);
        let buf = "";
        child.stdout.on("data", (chunk) => {
            buf += chunk;
            let nl;
            while ((nl = buf.indexOf("\n")) >= 0) {
                const line = buf.slice(0, nl);
                buf = buf.slice(nl + 1);
                out.write(redact(line, hidden) + "\n");
                try {
                    const frame = JSON.parse(line);
                    if (frame.type === "system" && frame.subtype === "init" && !model) {
                        model = frame.model;
                        refused = initProblem(frame, step.meta.model);
                        if (refused) {
                            log(`${refused}; stopping the session`);
                            child.kill();
                        }
                    }
                    if (frame.type === "assistant") lastAssistant = frame;
                    if (frame.type === "result") result = frame;
                    if (frame.type === "system" && frame.subtype === "compact_boundary") compactions++;
                } catch {
                    // non-JSON line
                }
            }
        });
        child.stderr.on("data", (chunk) => out.write(redact(String(chunk), hidden)));
        child.on("close", (code) => {
            clearTimeout(timer);
            clearInterval(renew);
            out.end(redact(buf, hidden));
            // A session that never said how it started costs no attempt: the CLI changed, not the step.
            if (!model && !refused && !timedOut) refused = initProblem(null, step.meta.model);
            const stats = { ...sessionStats(lastAssistant, result, step.meta.model, compactions), minutes: (Date.now() - began) / 60_000 };
            resolve({ code, model, refused, timedOut, stats, limited: hitLimit(result) });
        });
    });

// The latest other piece's spec folder, for the designer to write theirs like it.
const lastSpec = (piece) => {
    const dir = path.join(REPO, "relay/specs");
    const pieces = fs.existsSync(dir) ? fs.readdirSync(dir).map(Number).filter((n) => n && n !== piece.number) : [];
    const n = pieces.filter((n) => fs.existsSync(path.join(dir, String(n), "spec.md"))).sort((a, b) => b - a)[0];
    return n ? `relay/specs/${n}` : null;
};

// The files a step's prompt draws on, read from the piece's branch. `extra` holds what the
// crew read elsewhere: `usage` and `ideas`.
const stepPrompt = (piece, step, extra = {}) => {
    const kind = stepKind(step.meta.id);
    const prompts = Object.fromEntries([...promptsFor(step.meta.id), "session"].map((n) => [n, readRepo(promptPath(n))]));
    const designDir = path.join(REPO, "docs/design");
    const files = {
        prompts,
        ...extra,
        brief: readRepo(`${specDir(piece)}/${step.meta.id}.md`),
        spec: readRepo(`${specDir(piece)}/spec.md`),
        design: Object.fromEntries(designPaths(piece.body).map((f) => [f, readRepo(f)])),
        designFiles: fs.existsSync(designDir) ? fs.readdirSync(designDir).map((f) => `docs/design/${f}`) : [],
        diffStat: kind === "review" ? git("diff", "--stat=120", "origin/main...HEAD") : "",
        lastSpec: kind === "spec" ? lastSpec(piece) : null,
        notes: piece.steps.find((s) => s.meta.id === "wait")?.meta.notes ?? null,
        started: new Date().toISOString(),
        cap: capOf(step.meta.id) / 60_000,
    };
    return sessionPrompt(piece, step, files);
};

// ---- Piece transitions ----

const findPr = async (piece) => {
    const prs = await github("GET", `/pulls?state=open&head=${OWNER}:${encodeURIComponent(branchOf(piece))}`);
    return prs[0] ?? null;
};

// A step's issue, with its brief as it is on the branch now.
const createStep = async (piece, s) => {
    const meta = { id: s.id, model: s.model, effort: s.effort, why: s.why ?? "", attempts: 0, status: "pending" };
    for (const k of ["points", "part"]) if (s[k] != null) meta[k] = s[k];
    const brief = stepKind(s.id) === "build" ? readRepo(`${specDir(piece)}/${s.id}.md`) : null;
    const pr = (await findPr(piece))?.number;
    const issue = await github("POST", "/issues", {
        title: `#${piece.number} ${s.id}: ${s.title}`,
        body: writeMeta(stepBody(piece, s, { brief, pr }), meta),
        labels: ["kind:step"],
    });
    await github("POST", `/issues/${piece.number}/sub_issues`, { sub_issue_id: issue.id });
    await setBoard(issue.node_id, { Status: "Ready", Step: stepKind(s.id), Model: s.model, Effort: s.effort, Attempts: 0, Points: s.points });
    log(`#${piece.number}: made step ${s.id} (#${issue.number})`);
};

// A new piece gets its spec step, on the model and effort the piece's body chose, or its whole
// step when the body has a `Whole:` line. A retro gets all its steps at once.
const startPiece = async (piece) => {
    if (isRetro(piece)) {
        for (const s of RETRO_STEPS) await createStep(piece, s);
        await setStatus(piece, "Building");
        return;
    }
    const whole = wholeChoice(piece.body);
    if (whole) {
        await createStep(piece, { id: "whole", title: "The whole piece", ...whole });
        await setStatus(piece, "Building");
        return;
    }
    const { model, effort, why } = specChoice(piece.body);
    await createStep(piece, { id: "spec", title: "Spec and briefs", model, effort, why });
    await setStatus(piece, "Spec");
};

const readSteps = (piece, builds) => checkSteps(readRepo(`${specDir(piece)}/steps.json`) ?? "", builds);

// After the spec, spec-review on the model the piece's body chose. After spec-review, or a
// whole step that handed on, every later step from steps.json as that step left it. It was
// checked before the step closed, so an error here means the branch changed since; the step
// runs again to fix it.
const plan = async (piece) => {
    checkoutPiece(piece);
    const whole = piece.steps.find((s) => s.meta.id === "whole");
    const review = whole ?? piece.steps.find((s) => s.meta.id === "spec-review");
    if (!review) {
        const choice = specReviewChoice(piece.body) ?? { model: "opus", effort: "medium", why: "The piece names no spec-review, so the default." };
        await createStep(piece, { id: "spec-review", title: "Review the spec", ...choice });
        await setStatus(piece, "Spec");
        return;
    }
    const { steps, error } = readSteps(piece, whole ? 0 : 1);
    if (error) {
        await github("PATCH", `/issues/${review.number}`, { state: "open" });
        await saveMeta(review, { ...review.meta, status: "pending", note: `steps.json can't be read: ${error}` });
        log(`#${piece.number}: ${error}; ${review.meta.id} runs again`);
        return;
    }
    for (const s of planSteps(steps, playChoice(piece.body))) await createStep(piece, s);
    await setStatus(piece, "Building");
};

const blockStep = async (piece, step, why) => {
    await addLabel(piece.number, "waiting");
    await comment(
        piece.number,
        `⛔ **${step.meta.id}** is blocked: ${why}\n\nReply here to unblock it (your reply goes to the next session), or \`/relay retry\`, or \`/relay skip ${step.meta.id}\`.`,
    );
    await setStatus(piece, "Waiting", "your answer");
};

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

// Merges the PR when the review says merge and CI on its head commit is green, waiting up to
// ten minutes for CI to finish and the merge to go through. Returns why it didn't, or null.
const autoMerge = async (piece, pr, result) => {
    if (!pr) return "there's no PR";
    if (result.merge !== true) return "its verdict isn't merge";
    let why = "CI didn't finish in ten minutes";
    for (let i = 0; i < 20; i++, await sleep(30_000)) {
        const head = (await github("GET", `/pulls/${pr.number}`)).head.sha;
        const { check_runs } = await github("GET", `/commits/${head}/check-runs?per_page=100`);
        if (check_runs.some((c) => c.status === "completed" && !["success", "neutral", "skipped"].includes(c.conclusion))) return "CI failed";
        if (!checksGreen(check_runs)) continue;
        try {
            await github("PUT", `/pulls/${pr.number}/merge`, { sha: head, merge_method: "squash", commit_title: `${piece.title} (#${pr.number})` });
            why = null;
            break;
        } catch (e) {
            why = `merging failed: ${e.message}`;
        }
    }
    if (why) return why;
    log(`#${piece.number}: merged PR #${pr.number}`);
    await comment(piece.number, `🔀 The review's verdict is merge and CI is green, so ${pr.html_url} is merged.`);
    return null;
};

// A piece's PR is a draft until its verdict, so nothing asks for the owner's eyes before then.
const openPr = async (piece) => {
    const pr = { title: piece.title, head: branchOf(piece), base: "main", body: `Closes #${piece.number}.` };
    try {
        return await github("POST", "/pulls", { ...pr, draft: true });
    } catch (e) {
        log(`couldn't open the PR as a draft: ${e.message}`);
        return github("POST", "/pulls", pr);
    }
};
const markReady = async (pr) => {
    if (!pr?.draft) return;
    try {
        await graphql(`mutation($id: ID!) { markPullRequestReadyForReview(input: { pullRequestId: $id }) { pullRequest { id } } }`, { id: pr.node_id });
    } catch (e) {
        log(`couldn't mark PR #${pr.number} ready: ${e.message}`);
    }
};

// What follows a finished step: the draft PR once the branch has commits, the next steps after
// the spec, spec-review or a whole step that hands on, and the owner's turn after the verdict.
// A step's summary and decisions stay in its issue's block for the sessions after it: the PR's
// description is what the owner reads.
const afterStep = async (piece, step, result) => {
    const kind = stepKind(step.meta.id);
    let pr = await findPr(piece);
    if (!pr && git("rev-list", `origin/main..origin/${branchOf(piece)}`)) {
        pr = await openPr(piece);
        log(`#${piece.number}: PR #${pr.number}`);
        const tree = `https://github.com/${GH_REPO}/tree/${branchOf(piece)}/${specDir(piece)}`;
        await comment(piece.number, `📎 The PR is #${pr.number}, a draft until its verdict, and the spec is in [\`${specDir(piece)}/\`](${tree}).`);
    }
    await fileFindings(piece, step, result);
    if (kind === "spec" || kind === "spec-review" || step.meta.handed) await plan(await loadPiece(piece.number));
    else if (step.meta.id === "proposal") {
        await markReady(pr);
        await comment(piece.number, `📝 The proposal is on ${pr?.html_url ?? "the branch"}. Correct it there and merge it: the crew then opens its pieces.`);
        await setStatus(piece, "Review", "your merge");
    } else if (!VERDICTS.includes(kind)) await setStatus(piece, "Building");
    else {
        if (pr) {
            const fresh = await loadPiece(piece.number);
            const { body } = await github("GET", `/pulls/${pr.number}`);
            await github("PATCH", `/pulls/${pr.number}`, { body: withTable(body ?? `Closes #${piece.number}.`, stepsTable(fresh.steps)) });
            await markReady(pr);
        }
        const why = await autoMerge(piece, pr, result);
        if (why) {
            await comment(
                piece.number,
                `🔍 The ${kind === "whole" ? "piece" : "review"} is done on ${pr?.html_url ?? "the PR"}, but ${why}. Its description ends with the verdict. Merge it, which closes this piece, or close this piece to keep the branch as \`proof/${piece.number}\`.`,
            );
            await setStatus(piece, "Review", "your merge");
        }
    }
};

// The milestone's fixes piece, opened if it isn't yet.
const fixesPiece = async (milestone) => {
    const open = await allPages(`/issues?state=open&labels=relay:fixes&milestone=${milestone.number}`);
    if (open.length) return open[0];
    const issue = await github("POST", "/issues", {
        title: `Fixes for ${milestone.title}`,
        body: fixesBody(milestone.title),
        labels: ["kind:piece", "relay:ready", "relay:fixes"],
        milestone: milestone.number,
    });
    log(`opened the fixes piece for ${milestone.title}: #${issue.number}`);
    return issue;
};

// The issue that holds a milestone's ideas, opened if it isn't yet.
const ideasIssue = async (milestone, { open = true } = {}) => {
    const title = ideasTitle(milestone);
    const found = (await allPages("/issues?state=open&labels=kind:idea")).find((i) => i.title === title);
    if (found || !open) return found ?? null;
    return github("POST", "/issues", { title, body: IDEAS_BODY, labels: ["kind:idea"], ...(milestone ? { milestone: milestone.number } : {}) });
};

// A step's `fix` findings go on its milestone's fixes piece while that hasn't started; the rest
// are lines on the milestone's ideas issue, and the ideas the step took are ticked there. A
// retro's ideas stay in its angle files, which its proposal reads.
const fileFindings = async (piece, step, result) => {
    const { fix, ask } = sortFindings(result, { retro: isRetro(piece), milestone: piece.milestone });
    const took = (Array.isArray(result?.took) ? result.took : []).map(String);
    if (isRetro(piece) || (!fix.length && !ask.length && !took.length)) return;
    const from = `#${piece.number} ${step.meta.id}`;
    if (fix.length) {
        const fixes = await fixesPiece(piece.milestone);
        const started = (await allPages(`/issues/${fixes.number}/sub_issues`)).length > 0;
        if (started) ask.push(...fix);
        else {
            const body = fixes.body ?? "";
            const lines = fix.filter((i) => !body.includes(`**${i.title}**`)).map((i) => `- ${findingText(i, from)}`);
            if (lines.length) await github("PATCH", `/issues/${fixes.number}`, { body: `${body.trimEnd()}\n${lines.join("\n")}\n` });
        }
    }
    const ideas = await ideasIssue(piece.milestone, { open: ask.length > 0 });
    if (ideas) {
        const body = withIdeas(tickIdeas(ideas.body, took, from), ask, from);
        if (body !== ideas.body) await github("PATCH", `/issues/${ideas.number}`, { body });
    }
    log(`#${piece.number} ${step.meta.id}: ${fix.length} fix findings, ${ask.length} ideas, ${took.length} taken`);
};

// When none of a milestone's pieces can run, main is tagged `m<n>` and its retro opens.
const milestoneCheck = async (milestone, { early = false } = {}) => {
    const tag = milestoneTag(milestone);
    if (!tag) return null;
    if (!early) {
        const open = (await allPages(`/issues?state=open&labels=kind:piece&milestone=${milestone.number}`)).filter((i) => !i.pull_request);
        if (!isBuilt(open.map((i) => ({ labels: i.labels.map((l) => l.name) })))) return null;
        try {
            const main = await github("GET", "/git/ref/heads/main");
            await github("POST", "/git/refs", { ref: `refs/tags/${tag}`, sha: main.object.sha });
            log(`${milestone.title} is built: tagged main ${tag}`);
        } catch (e) {
            log(`couldn't tag ${tag}: ${e.message}`);
        }
    }
    if ((await allPages(`/issues?state=all&labels=kind:retro&milestone=${milestone.number}`)).length) return null;
    const issue = await github("POST", "/issues", { title: `${milestone.title} retro`, body: retroBody(milestone), labels: ["kind:retro", "relay:ready"], milestone: milestone.number });
    await comment(
        issue.number,
        `🔁 The retro starts with the play check and three angles. Play main and comment your notes here within 72 hours, or \`/relay retro go\` to go on without them.`,
    );
    log(`opened the ${milestone.title} retro: #${issue.number}`);
    return issue;
};

// A file on main, read through the API, or null.
const mainFile = async (f) => {
    try {
        const { content } = await github("GET", `/contents/${f}?ref=main`);
        return Buffer.from(content, "base64").toString("utf8");
    } catch {
        return null;
    }
};

// A merged retro opens the next milestone's pieces from its proposal, `relay:ready`.
const openProposal = async (retro) => {
    const m = milestoneTag(retro.milestone);
    if (!m) return;
    const next = `M${Number(m.slice(1)) + 1}`;
    const pieces = parseProposal(await mainFile(`docs/proposals/${next.toLowerCase()}.md`));
    if (!pieces.length) return comment(retro.number, `No pieces in \`docs/proposals/${next.toLowerCase()}.md\` on main, so none were opened.`);
    const milestones = await allPages("/milestones?state=all");
    const milestone = milestones.find((x) => milestoneTag(x) === next.toLowerCase()) ?? (await github("POST", "/milestones", { title: next }));
    const numbers = {};
    for (const p of pieces) {
        const issue = await github("POST", "/issues", { title: p.title, body: withNumbers(p.body, numbers), labels: ["kind:piece", "relay:ready"], milestone: milestone.number });
        numbers[p.key] = issue.number;
    }
    await comment(retro.number, `🚀 Opened ${next}'s pieces: ${pieces.map((p) => `${p.key} #${numbers[p.key]}`).join(", ")}.`);
    await settleIdeas(retro, `docs/proposals/${next.toLowerCase()}.md`);
};

// A merged proposal ends the ideas it took or dropped: each proposed issue its `## Ideas`
// section names is closed with its line, and so are the milestone's ideas issue and its log.
const settleIdeas = async (retro, file) => {
    const blob = `https://github.com/${GH_REPO}/blob/main/${file}`;
    for (const { number, line } of settledIdeas(await mainFile(file))) {
        try {
            const issue = await github("GET", `/issues/${number}`);
            if (issue.state !== "open" || !issue.labels.some((l) => l.name === "relay:proposed")) continue;
            await comment(number, `[The proposal](${blob}) settles this: ${line}`);
            await github("PATCH", `/issues/${number}`, { state: "closed", state_reason: /\bdropped\b/i.test(line) ? "not_planned" : "completed" });
        } catch (e) {
            log(`couldn't settle #${number}: ${e.message}`);
        }
    }
    const done = [await ideasIssue(retro.milestone, { open: false }), (await allPages("/issues?state=open&labels=kind:chore")).find((i) => i.title === logTitle(retro.milestone))];
    for (const issue of done.filter(Boolean)) {
        await comment(issue.number, `[The proposal](${blob}) is merged, so this closes with ${retro.milestone.title}.`);
        await github("PATCH", `/issues/${issue.number}`, { state: "closed" });
    }
};

// Ends a retro's wait, with or without the owner's notes.
const endWait = async (piece, step) => {
    const line = step.meta.notes ? "done: the owner's notes came" : step.meta.go ? "done: the owner said go" : "done: no notes in 72 hours";
    step.body = withStatus(step.body, line);
    await saveMeta(step, { ...step.meta, status: "done" });
    await closeStep(step);
    await setBoard(step.nodeId, { Status: "Done" });
};

// Runs one step and records how it went. Returns a line for the run report, or throws
// `refused` when the session couldn't start as asked.
// `before` is the usage the crew paced on, so the step's points are the rise from it.
const runStep = async (piece, step, before) => {
    const branch = checkoutPiece(piece);
    const tracked = git("ls-files", "relay/runs");
    if (tracked) {
        throw Object.assign(new Error(`#${piece.number}'s branch tracks ${tracked.split("\n").join(", ")}; \`git rm --cached\` it before a session runs`), { refused: true });
    }
    const kind = stepKind(step.meta.id);
    await setStatus(piece, { review: "Review", build: "Building", play: "Building", retro: "Building", whole: "Building" }[kind] ?? "Spec");
    let prompt;
    try {
        const ideas = ["spec", "whole", "play"].includes(kind) ? openIdeas((await ideasIssue(piece.milestone, { open: false }))?.body) : null;
        prompt = stepPrompt(piece, step, { usage: before, ideas });
    } catch (e) {
        // The spec can't brief the session; only a spec edit fixes that.
        step.body = withStatus(step.body, `blocked: ${e.message} (see #${piece.number})`);
        await saveMeta(step, { ...step.meta, status: "blocked" });
        await blockStep(piece, step, e.message);
        return { line: `#${piece.number} ${step.meta.id} (#${step.number}): blocked (${e.message})`, after: before };
    }
    await writeTokenFile();
    log(`start #${piece.number} ${step.meta.id} on ${step.meta.model}/${step.meta.effort} with Claude Code ${cliVersion()}`);
    const { code, model, refused, timedOut, stats, limited } = await runSession(piece, step, prompt);
    if (refused) {
        tidyUp(branch);
        throw Object.assign(new Error(`stopped #${piece.number} ${step.meta.id} before it began: ${refused}`), { refused: true });
    }
    const exitOk = code === 0 && !timedOut && model !== null;
    let result = readResult(fs.existsSync(RESULT) ? fs.readFileSync(RESULT, "utf8") : null);
    log(`end #${piece.number} ${step.meta.id}: exit ${code}, model ${model}, ${dollars(stats.cost)}, ${stats.minutes.toFixed(1)} minutes, ${stats.context}% context, ${stats.compactions} compactions, result ${JSON.stringify(result)}`);
    tidyUp(branch);
    const head = git("log", "-1", "--format=%h %s");
    // Spec-review isn't done until the crew can make the steps.
    if (step.meta.id === "spec-review" && result?.status === "done") {
        const { error } = readSteps(piece);
        if (error) result = { status: "pending", note: `You reported done, but steps.json can't be read: ${error}` };
    }
    // A whole step ends with its verdict, or hands on to the steps it wrote in steps.json.
    const handed = kind === "whole" && result?.status === "done" && typeof result.merge !== "boolean" && !readSteps(piece, 0).error;
    if (VERDICTS.includes(kind) && result?.status === "done" && typeof result.merge !== "boolean" && !handed) {
        const hand = kind === "whole" ? `, or a \`steps.json\` the crew can read to hand on (${readSteps(piece, 0).error})` : "";
        result = { status: "pending", note: `You reported done without \`merge\`: result.json needs \`"merge": true\` or \`false\`${hand}.` };
    }
    let after = null;
    try {
        after = await usage();
    } catch (e) {
        log(`couldn't read usage after the step: ${e.message}`);
    }

    const applied = applyResult(step.meta, result, exitOk);
    let { outcome, why } = applied;
    const { meta } = applied;
    // The usage limit ending a session isn't the step's doing, so it costs no attempt.
    if (limited && outcome !== "done") {
        outcome = "pending";
        meta.status = "pending";
        meta.attempts = step.meta.attempts ?? 0;
        meta.note = `The usage limit ended the last session, not anything it did. What it hadn't committed is gone; the branch's last commit is \`${head}\`.`;
    }
    if (handed) meta.handed = true;
    const points = usagePoints(before, after);
    const add = (was, n) => (n == null ? was ?? null : (was ?? 0) + n);
    meta.cost = cents((step.meta.cost ?? 0) + stats.cost);
    meta.minutes = Math.round(((step.meta.minutes ?? 0) + stats.minutes) * 10) / 10;
    meta.fiveHour = add(step.meta.fiveHour, points.fiveHour);
    meta.weekly = add(step.meta.weekly, points.weekly);
    const rounded = (u) => u && { session: Math.round(u.session), weekly: Math.round(u.weekly) };
    meta.sessions = (step.meta.sessions ?? 0) + 1;
    meta.usage = [...(step.meta.usage ?? []), { before: rounded(before), after: rounded(after) }];
    meta.context = stats.context;
    meta.compactions = (step.meta.compactions ?? 0) + stats.compactions;
    meta.cli = cliVersion();
    step.body = withStatus(step.body, statusLine(meta, outcome, outcome === "blocked" ? `${why} (see #${piece.number})` : meta.note));
    await saveMeta(step, meta);
    await setBoard(step.nodeId, {
        Status: outcome === "done" ? "Done" : outcome === "blocked" ? "Waiting" : "Building",
        Attempts: meta.sessions,
        "List $": meta.cost,
        Minutes: Math.round(meta.minutes),
        "5-hour points": meta.fiveHour,
        "Weekly points": meta.weekly,
    });
    if (outcome === "done") {
        await closeStep(step);
        await afterStep(piece, step, result);
    } else if (outcome === "blocked") {
        await blockStep(piece, step, why);
    }
    return { line: `#${piece.number} ${step.meta.id} (#${step.number}): ${outcome}${handed ? ", handed on" : ""}`, after };
};

// ---- Commands ----

// Pacing from the subscription's usage windows, saying which route answered.
const usage = async () => {
    const { route, usage: u } = await readUsage();
    log(`usage from the ${route} route`);
    return u;
};

// Starts a crew run, as if from Actions → relay → Run workflow.
const dispatchRun = () =>
    github("POST", `/actions/workflows/${WORKFLOW}/dispatches`, { ref: "main", inputs: { command: "run" } }, process.env.DISPATCH_TOKEN);

// The log issue of `milestone`, opened if it isn't yet; without one, the newest log there is.
const logIssue = async (milestone) => {
    const logs = (await allPages("/issues?state=open&labels=kind:chore")).filter((i) => i.title.startsWith(LOG_TITLE));
    const found = milestone ? logs.find((i) => i.title === logTitle(milestone)) : logs[0];
    if (found) return found.number;
    const issue = await github("POST", "/issues", {
        title: logTitle(milestone),
        body: `Each Relay run${milestone ? ` on ${milestone.title}` : ""} reports here, each line linking its step. Pieces report on their own issues.`,
        labels: ["kind:chore"],
        ...(milestone ? { milestone: milestone.number } : {}),
    });
    return issue.number;
};

// Starts a run that waits for a window to lift, then starts a crew run.
const dispatchWait = (until) =>
    github(
        "POST",
        `/actions/workflows/${WORKFLOW}/dispatches`,
        { ref: "main", inputs: { command: "wait", until: new Date(Math.min(until.getTime(), Date.now() + WAIT_MAX_MS)).toISOString() } },
        process.env.DISPATCH_TOKEN,
    );

// The crew's own code at a ref: the blob of each module it loads.
const crewCode = (ref) =>
    git("ls-tree", ref, "relay/")
        .split("\n")
        .filter((l) => /\.mjs$/.test(l) && !/\.test\.mjs$/.test(l))
        .map((l) => l.split(/\s+/)[2])
        .join(" ");

// Runs steps until something stops it, and says why. The usage read after one step paces the
// next, so each step boundary costs one probe. A crew whose code main has moved past hands
// over to a fresh run on main.
const runSteps = async (ran) => {
    const code = crewCode("HEAD");
    let reading = null;
    for (;;) {
        if (Date.now() - STARTED > DEADLINE_MS) return "deadline: no time for another step in this job";
        git("fetch", "-q", "origin", "main");
        if (crewCode("origin/main") !== code) {
            await dispatchRun();
            return "the crew changed on main: a fresh run carries on";
        }
        const pieces = await loadPieces();
        const next = nextWork(pieces);
        if (!next) return "nothing to do: every ready piece is waiting, paused, held or finished";
        ran.milestone = next.piece.milestone ?? ran.milestone;
        if (next.action === "start") {
            await startPiece(next.piece);
            ran.push(`#${next.piece.number}: started`);
            continue;
        }
        if (next.action === "plan") {
            await plan(next.piece);
            ran.push(`#${next.piece.number}: steps made`);
            continue;
        }
        if (next.action === "waited") {
            await endWait(next.piece, next.step);
            ran.push(`#${next.piece.number}: the wait is over`);
            continue;
        }
        const cap = capOf(next.step.meta.id);
        if (Date.now() - STARTED + cap > DEADLINE_MS + SESSION_TIMEOUT_MS) {
            await dispatchRun().catch((e) => log(`couldn't start a run: ${e.message}`));
            return `deadline: #${next.piece.number} ${next.step.meta.id} needs ${cap / 60_000} minutes, so it runs in a job of its own`;
        }
        const before = reading ?? (await usage());
        reading = null;
        ran.usage = before;
        const verdict = pace(before, new Date(), stopOf(next.step.meta.id));
        if (!verdict.ok) {
            await setBoard(next.piece.nodeId, { "Waiting on": waitingOn(verdict) });
            if (verdict.until) {
                try {
                    await dispatchWait(verdict.until);
                } catch (e) {
                    log(`couldn't start a run for the reset: ${e.message}`);
                }
            }
            return `budget stop: ${verdict.reason}`;
        }
        log(verdict.reason);
        const { line, after } = await runStep(next.piece, next.step, before);
        ran.push(line);
        reading = after;
        ran.usage = after ?? ran.usage;
    }
};

const run = async () => {
    await setUpIdentity();
    const ran = [];
    let reason;
    try {
        reason = await runSteps(ran);
    } catch (e) {
        if (!e.refused) throw e;
        reason = e.message;
    }
    log(reason);
    // A quiet stop with nothing run isn't worth a notification.
    if (ran.length > 0 || !/^(budget stop|nothing to do)/.test(reason)) {
        const steps = ran.length ? ran.map((r) => `- ${r}`).join("\n") : "- (no steps)";
        const url = process.env.GITHUB_RUN_ID ? `\n\n[The run](https://github.com/${GH_REPO}/actions/runs/${process.env.GITHUB_RUN_ID})` : "";
        const windows = ran.usage ? `\n\n${usageReport(ran.usage, new Date())}.` : "";
        await comment(await logIssue(ran.milestone), `🏃 Run finished: ${reason}\n\n${steps}${windows}${url}`);
    }
};

const status = async () => {
    const pieces = await loadPieces();
    for (const p of pieces) console.log(`#${p.number} ${p.title}: ${holdReason(p) ?? JSON.stringify(pieceAction(p)?.action ?? "finished")}`);
    const next = nextWork(pieces);
    console.log(next ? `next: #${next.piece.number} ${next.action}${next.step ? ` ${next.step.meta.id}` : ""}` : "next: nothing");
    console.log(`pacing: ${JSON.stringify(pace(await usage(), new Date(), next?.step ? stopOf(next.step.meta.id) : undefined))}`);
};

// For the workflow to report a failure the crew couldn't, e.g. the job timing out.
const report = async () => comment(await logIssue(), `⚠️ ${process.argv.slice(3).join(" ")}`);

// ---- Events: the owner's comments on a piece ----

const findStep = (piece, id) => piece.steps.find((s) => s.meta.id === id && s.state === "open");

// The owner's reply on a piece with a blocked step goes to that step's next session.
const unblock = async (piece, step, reply) => {
    const meta = { ...step.meta, status: "pending", attempts: 0 };
    if (reply) meta.reply = reply;
    delete meta.note;
    step.body = withStatus(step.body, reply ? "ready, with your reply" : "ready, retrying");
    await saveMeta(step, meta);
    if (!piece.steps.some((s) => s !== step && s.state === "open" && s.meta.status === "blocked")) await removeLabel(piece.number, "waiting");
    await setStatus(piece, "Building");
};

const COMMANDS = {
    model: async (piece, [id, ...rest]) => {
        const step = findStep(piece, id);
        if (!step) return `No open step \`${id}\`.`;
        const meta = applyModel(step.meta, rest);
        if (!meta) return `Name a model (${MODELS.join(", ")}) or an effort (${EFFORTS.join(", ")}).`;
        const was = `${step.meta.model}/${step.meta.effort}`;
        await saveMeta(step, { ...meta, why: `The owner's choice (the spec had ${was}).${step.meta.why ? ` ${step.meta.why}` : ""}` });
        await setBoard(step.nodeId, { Model: meta.model, Effort: meta.effort });
        return `${id}: ${was} → ${meta.model}/${meta.effort}.`;
    },
    retry: async (piece, [id]) => {
        const step = id ? findStep(piece, id) : piece.steps.find((s) => s.state === "open" && s.meta.status === "blocked");
        if (!step) return id ? `No open step \`${id}\`.` : "No step is blocked.";
        await unblock(piece, step, null);
        await dispatchRun();
        return `${step.meta.id} gets two fresh attempts, starting now.`;
    },
    skip: async (piece, [id]) => {
        const step = findStep(piece, id);
        if (!step) return `No open step \`${id}\`.`;
        await setStepStatus(step, "skipped by the owner");
        await closeStep(step, "not_planned");
        step.state = "closed";
        if (!piece.steps.some((s) => s.state === "open" && s.meta.status === "blocked")) await removeLabel(piece.number, "waiting");
        return `${id} skipped.`;
    },
    pause: async (piece) => {
        await addLabel(piece.number, "relay:paused");
        if (piece.milestone) await milestoneCheck(piece.milestone);
        return "Paused. A step that's running finishes first.";
    },
    ready: async (piece) => {
        if (piece.labels.includes("relay:proposed")) await removeLabel(piece.number, "relay:proposed");
        if (!piece.labels.includes("relay:ready")) await addLabel(piece.number, "relay:ready");
        const hold = holdReason({ ...piece, labels: ["relay:ready"] });
        return `Labelled \`relay:ready\`.${hold ? ` It's held: ${hold}.` : " The next run picks it up."}`;
    },
    retro: async (piece, [word]) => {
        if (isRetro(piece)) {
            const wait = findStep(piece, "wait");
            if (word !== "go") return "On a retro, `/relay retro go` ends the wait for your notes.";
            if (!wait) return "This retro isn't waiting.";
            await saveMeta(wait, { ...wait.meta, go: true });
            await dispatchRun();
            return "Going on without notes, starting now.";
        }
        if (!piece.milestone) return "This piece has no milestone, so there's no retro to start.";
        const issue = await milestoneCheck(piece.milestone, { early: true });
        if (!issue) return `${piece.milestone.title}'s retro is already open.`;
        await dispatchRun();
        return `Opened #${issue.number}, starting now.`;
    },
    resume: async (piece) => {
        await removeLabel(piece.number, "relay:paused");
        return "Resumed from the next run.";
    },
    status: async (piece) => (piece.steps.length ? stepsTable(piece.steps) : "No steps yet: the next run starts the spec."),
    run: async (piece) => {
        // Asking for a run on a piece says it's ready, so the run has it to work on.
        const readied = !piece.labels.includes("relay:ready");
        if (readied) await addLabel(piece.number, "relay:ready");
        await dispatchRun();
        const hold = holdReason({ ...piece, labels: [...new Set([...piece.labels, "relay:ready"])] });
        return `${readied ? "Labelled `relay:ready` and started" : "Started"} a run.${hold ? ` This piece is held (${hold}), so the run works on others.` : ""}`;
    },
};

export const HELP =
    "Commands: `run`, `retry [step]`, `skip <step>`, `model <step> <model> [effort]`, `pause`, `resume`, `status`, `ready`, `retro` (on a piece: start its milestone's retro now; on a retro: `retro go` ends the wait).";

// A closed piece closes its open steps. Closed without merging, its branch head is tagged
// `proof/<n>` and its PR closed.
const closePiece = async (number) => {
    const piece = await loadPiece(number);
    for (const s of piece.steps.filter((s) => s.state === "open")) {
        await setStepStatus(s, `closed with #${number}`);
        await closeStep(s, "not_planned");
    }
    const pr = await findPr(piece);
    if (pr) {
        try {
            await github("POST", "/git/refs", { ref: `refs/tags/proof/${number}`, sha: pr.head.sha });
            await github("PATCH", `/pulls/${pr.number}`, { state: "closed" });
            log(`#${number}: tagged proof/${number} and closed PR #${pr.number}`);
        } catch (e) {
            log(`#${number}: couldn't tag the proof: ${e.message}`);
        }
    }
    await setStatus(piece, "Done");
    if (isRetro(piece)) {
        if (await isMerged(number)) await openProposal(piece);
    } else if (piece.milestone) await milestoneCheck(piece.milestone);
    // A close frees the pieces waiting on it, so the next run needn't wait for the schedule.
    await dispatchRun().catch((e) => log(`couldn't start a run: ${e.message}`));
};

const event = async () => {
    const e = JSON.parse(fs.readFileSync(process.env.GITHUB_EVENT_PATH, "utf8"));
    if (e.action === "closed" && !e.comment) {
        if (e.issue.labels.some((l) => ["kind:piece", "kind:retro"].includes(l.name))) await closePiece(e.issue.number);
        return;
    }
    if (e.comment?.user?.login !== OWNER || e.issue?.pull_request) return log("not the owner's comment on an issue; ignoring");
    const labels = e.issue.labels.map((l) => l.name);
    const cmd = parseCommand(e.comment.body);
    if (!labels.includes("kind:piece") && !labels.includes("kind:retro")) {
        if (cmd) await comment(e.issue.number, "`/relay` commands go on the piece's issue.");
        return;
    }
    const piece = await loadPiece(e.issue.number);
    const wait = isRetro(piece) ? findStep(piece, "wait") : null;
    if (!cmd && wait) {
        const notes = [wait.meta.notes, e.comment.body].filter(Boolean).join("\n\n");
        await saveMeta(wait, { ...wait.meta, notes });
        await dispatchRun();
        await comment(piece.number, "Got your notes. The game and players angles read them, starting now.");
        return;
    }
    if (!cmd) {
        const blocked = piece.steps.find((s) => s.state === "open" && s.meta.status === "blocked");
        if (!blocked) return log("a comment, not a command, and nothing is blocked");
        await unblock(piece, blocked, e.comment.body);
        await dispatchRun();
        await comment(piece.number, `Picked up. ${blocked.meta.id} carries on with your reply, starting now.`);
        return;
    }
    const handler = COMMANDS[cmd.name];
    const reply = handler ? await handler(piece, cmd.args) : `Unknown command \`${cmd.name}\`. ${HELP}`;
    await comment(piece.number, reply);
};

if (process.argv[1] === fileURLToPath(import.meta.url)) {
    const cmd = process.argv[2];
    const main = { run, status, event, report }[cmd];
    if (!main) {
        console.error("Usage: node relay/relay.mjs <run|status|event|report <message>>");
        process.exit(2);
    }
    Promise.resolve(main()).catch(async (e) => {
        log(`crash: ${e.stack ?? e}`);
        if (cmd === "run") {
            try {
                await comment(await logIssue(), `💥 Relay crashed:\n\n\`\`\`\n${redact(e.message ?? String(e), secrets())}\n\`\`\``);
            } catch {
                // nothing left to report through
            }
        }
        process.exit(1);
    });
}
