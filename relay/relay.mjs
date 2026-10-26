// The Relay crew: builds Sans Fiat one piece at a time, one `claude -p` session per step,
// on GitHub Actions (.github/workflows/relay.yml). Ported from Tortoise Spelling's Jogging.
//
//   node relay/relay.mjs run       # run steps until the budget, the deadline or the work runs out
//   node relay/relay.mjs status    # print the next step and the pacing verdict, run nothing
//   node relay/relay.mjs event     # act on the owner's comment (relay-events.yml)
//   node relay/relay.mjs report <message>   # comment on the Relay log issue
//
// GitHub holds the state. A piece is an open `kind:piece` issue labelled `relay:ready`; its
// steps are `kind:step` sub-issues whose bodies carry a `<!-- relay {...} -->` block with the
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

const HERE = path.dirname(fileURLToPath(import.meta.url));
const REPO = path.dirname(HERE);
const RUNS = path.join(HERE, "runs");
const RESULT = path.join(RUNS, "result.json");
export const OWNER = "DontPanic345";
const GH_REPO = `${OWNER}/sansfiat`;
const BOARD = "Sans Fiat Work";
const LOG_TITLE = "Relay log";
const WORKFLOW = "relay.yml";
const MAX_ATTEMPTS = 2;
const STARTED = Date.now();
// The job is killed at 345 minutes. No step starts after the deadline, and none runs past
// its timeout, so the crew always gets to record the last one.
const DEADLINE_MS = Number(process.env.RELAY_DEADLINE_MIN ?? 250) * 60_000;
const SESSION_TIMEOUT_MS = Number(process.env.RELAY_SESSION_MIN ?? 90) * 60_000;
// Written into transcripts, which become a workflow artifact anyone signed in can read.
export const SECRETS = ["CLAUDE_CODE_OAUTH_TOKEN", "RELAY_APP_KEY", "DISPATCH_TOKEN", "RELAY_BOARD_TOKEN"];

export const MODELS = ["haiku", "sonnet", "opus"];
export const EFFORTS = ["low", "medium", "high"];

// ---- Pure logic (covered by relay.test.mjs) ----

// Continue while the 5-hour window is under 50% and the weekly window is under
// day-of-cycle / 7 × 95%. A stop says when it lifts, as `until`, where that's known.
export const pace = ({ session, weekly, weeklyResetsAt, sessionResetsAt }, now) => {
    const cycleStart = weeklyResetsAt.getTime() - 7 * 86400_000;
    const day = Math.min(7, Math.max(1, Math.floor((now.getTime() - cycleStart) / 86400_000) + 1));
    const weeklyLimit = (day / 7) * 95;
    if (session >= 50) return { ok: false, window: "5-hour", until: sessionResetsAt ?? null, reason: `5-hour usage ${session}% ≥ 50%` };
    if (weekly >= weeklyLimit) {
        const lifts = Math.floor((weekly * 7) / 95) + 1;
        const until = lifts > 7 ? weeklyResetsAt : new Date(cycleStart + (lifts - 1) * 86400_000);
        return { ok: false, window: "weekly", until, reason: `weekly usage ${weekly}% ≥ ${weeklyLimit.toFixed(1)}% (day ${day})` };
    }
    return { ok: true, reason: `5-hour ${session}%, weekly ${weekly}% < ${weeklyLimit.toFixed(1)}% (day ${day})` };
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

// `**After:** #31, #32`: the pieces this one waits to see merged.
export const afterOf = (body) => [...(pieceField(body, "After") ?? "").matchAll(/#(\d+)/g)].map((m) => Number(m[1]));

// Every check on a commit finished and none failed. No checks at all isn't green.
export const checksGreen = (runs) =>
    runs.length > 0 && runs.every((c) => c.status === "completed" && ["success", "neutral", "skipped"].includes(c.conclusion));

// The `docs/design/` files a piece's body names.
export const designPaths = (body) => [...new Set((body ?? "").match(/docs\/design\/[\w./-]+\.md/g) ?? [])];

// Steps run spec, spec-review, build-1 … build-N, review.
export const stepRank = (id) => {
    if (id === "spec") return 0;
    if (id === "spec-review") return 1;
    if (id === "review") return 1000;
    const m = /^build-(\d+)$/.exec(id);
    return m ? 1 + Number(m[1]) : 500;
};
export const stepKind = (id) => (["spec", "spec-review", "review"].includes(id) ? id : "build");
const sortSteps = (steps) => [...steps].sort((a, b) => stepRank(a.meta.id) - stepRank(b.meta.id));

export const branchOf = (piece) => `relay/${piece.number}`;
export const specDir = (piece) => `relay/specs/${piece.number}`;

// The steps the spec wrote to `relay/specs/<n>/steps.json`, as spec-review left them,
// checked. Returns { steps } or { error }.
export const checkSteps = (text) => {
    let steps;
    try {
        steps = JSON.parse(text);
    } catch (e) {
        return { error: `steps.json isn't JSON: ${e.message}` };
    }
    const ids = Array.isArray(steps) ? steps.map((s) => s?.id) : [];
    if (ids.at(-1) !== "review" || ids.length < 2) {
        return { error: "steps.json must list the build steps, then `review`" };
    }
    for (const [i, s] of steps.slice(0, -1).entries()) {
        if (s.id !== `build-${i + 1}`) return { error: `build step ${i + 1} in steps.json must be \`build-${i + 1}\`, not \`${s.id}\`` };
    }
    for (const s of steps) {
        if (!MODELS.includes(s.model)) return { error: `${s.id}: model must be one of ${MODELS.join(", ")}` };
        if (!EFFORTS.includes(s.effort)) return { error: `${s.id}: effort must be one of ${EFFORTS.join(", ")}` };
        if (!Number.isInteger(s.points) || s.points < 0 || s.points > 15) return { error: `${s.id}: points must be a whole number from 0 to 15` };
        if (!s.title) return { error: `${s.id}: no title` };
        if (!s.why) return { error: `${s.id}: no why for its model and effort` };
    }
    return { steps };
};

// Why a piece can't run now, or null when it can.
export const holdReason = (piece) => {
    if (!piece.labels.includes("relay:ready")) return "not ready";
    // Anyone can open an issue, but only the owner's words reach a session.
    if (piece.author !== OWNER) return "not opened by the owner";
    if (piece.labels.includes("relay:paused")) return "paused";
    if (piece.labels.includes("waiting")) return "waiting on the owner";
    const unmerged = (piece.after ?? []).filter((a) => !a.merged).map((a) => `#${a.number}`);
    if (unmerged.length) return `after ${unmerged.join(", ")}, not merged yet`;
    if (piece.steps.length === 0) {
        if (!specChoice(piece.body)) return "the body has no `Spec:` model and effort";
        if (!specReviewChoice(piece.body)) return "the body has no `Spec-review:` model and effort";
    }
    return null;
};

// What the crew does next for one piece: "start" (it has no steps yet), "step" (run
// `step`), "plan" (the spec or spec-review is done and the next steps aren't made), or null.
export const pieceAction = (piece) => {
    if (piece.steps.length === 0) return { action: "start" };
    const steps = sortSteps(piece.steps);
    const open = steps.find((s) => s.state === "open");
    if (open) return open.meta.status === "blocked" ? null : { action: "step", step: open };
    if (steps.every((s) => s.meta.id === "spec" || s.meta.id === "spec-review")) return { action: "plan" };
    return null;
};

// The next piece and what to do with it, oldest first.
export const nextWork = (pieces) => {
    for (const piece of [...pieces].sort((a, b) => a.number - b.number)) {
        if (holdReason(piece)) continue;
        const next = pieceAction(piece);
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

// A finished step's summary and decisions, as later steps see them.
export const soFar = (s) => {
    const decided = (s.meta.decisions ?? []).map((d) => `\n  - Decided: ${d.decision}${d.why ? ` (${d.why})` : ""}`).join("");
    return `- **${s.meta.id}**: ${s.meta.summary}${decided}`;
};

// The `relay/prompts/` files each kind of step follows, besides `session.md`.
export const PROMPTS_FOR = {
    spec: ["spec"],
    "spec-review": ["spec-review", "spec"],
    build: ["build"],
    review: ["review"],
};

// A session's whole prompt: its step first, then only the material that serves it, then
// the rules for its kind of step. Throws when a build step has no brief; the crew blocks
// the step on that.
// `files` holds `prompts` (name → text), `brief` and `spec` (null when missing), `design`
// (path → text, null for a named file that doesn't exist), `designFiles` (every path in
// docs/design/), and `diffStat` (review only).
export const sessionPrompt = (piece, step, files) => {
    const kind = stepKind(step.meta.id);
    const parts = [
        `You are a Relay session for Sans Fiat, in GitHub Actions with no other context: one cold session ` +
            `doing one step of piece #${piece.number}, "${piece.title}". You are on its branch, ` +
            `\`${branchOf(piece)}\`, and its spec lives in \`${specDir(piece)}/\`. This prompt holds how Relay ` +
            `works; the design and the code hold the rest.`,
    ];
    if (kind === "build") {
        if (!files.brief) throw new Error(`${specDir(piece)}/${step.meta.id}.md is missing`);
        parts.push(atLevel(files.brief.trim(), 1));
    } else {
        const what = {
            spec: "Write the spec, the briefs and steps.json for this piece",
            "spec-review": "Review the spec, the briefs and steps.json for this piece",
            review: "Review this piece as built",
        }[kind];
        parts.push(`# Your step: #${piece.number} ${step.meta.id}\n\n${what}, as agreed with the owner:`);
        parts.push(atLevel(`# The piece\n\n${piece.body.trim()}`));
    }
    if (step.meta.note) parts.push(`# The last attempt didn't finish\n\nPick up from here.\n\n${step.meta.note}`);
    if (step.meta.reply) parts.push(`# The owner's reply to this step's blocker\n\nAct on it.\n\n"""${step.meta.reply}"""`);
    if (kind !== "build") {
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
        parts.push(`# The steps\n\nPut this table at the top of the PR description, under "Closes #${piece.number}", with your own row added:\n\n${stepsTable(piece.steps)}`);
    }
    const done = sortSteps(piece.steps).filter((s) => s.state === "closed" && s.meta.summary);
    if (done.length) parts.push(`# This piece so far\n\n${done.map(soFar).join("\n")}`);
    for (const name of [...PROMPTS_FOR[kind], "session"]) {
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

export const dollars = (n) => `$${(n ?? 0).toFixed(2)}`;
export const cents = (n) => Math.round((n ?? 0) * 100) / 100;

// The usage points a step took: the rise in each window from before the session to after.
// A window that reset in between gives null, since its rise can't be known.
export const usagePoints = (before, after) => {
    const rise = (a, b) => (a == null || b == null || b < a ? null : Math.round(b - a));
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
    const on = `**${s.model} ${s.effort}**${s.points != null ? `, ${s.points} points` : ""}${s.coin ? ", by coin" : ""}`;
    const links = [`Piece #${piece.number}`, ...(pr ? [`PR #${pr}`] : []), `[spec.md](${blob}/spec.md)`, ...(brief ? [`[the brief](${blob}/${s.id}.md)`] : [])];
    const parts = [`**Status:** ready`, `Step \`${s.id}\` on ${on}. ${s.why ?? ""}`.trim(), links.join(" · ")];
    if (brief) {
        const text = brief.trim();
        const cut = text.length > 55_000 ? `${text.slice(0, 55_000)}\n\n…cut here; the rest is in the brief on the branch.` : text;
        parts.push(`---\n\nThe brief as it was when this step was made. The file on the branch is the one builds keep up.\n\n${atLevel(cut, 2)}`);
    }
    return parts.join("\n\n");
};

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

// The ideas a session noticed, as a list for its PR comment.
export const ideaList = (ideas) =>
    (Array.isArray(ideas) ? ideas : [])
        .filter((i) => i?.title)
        .map((i) => `- **${i.title}**${i.body ? `: ${i.body}` : ""}`)
        .join("\n");

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
        "| Step | State | Model | Effort | Points | Estimate | Attempts | List $ | Minutes | 5-hour | Weekly |",
        "|---|---|---|---|---|---|---|---|---|---|---|",
        ...sorted.map((s) => {
            const m = s.meta;
            const state = s.state === "closed" ? (s.stateReason === "not_planned" ? "skipped" : "done") : m.status ?? "pending";
            const model = `${m.model}${m.coin ? " (coin)" : ""}`;
            return `| #${s.number} ${m.id} | ${state} | ${model} | ${m.effort} | ${num(m.points)} | ${num(m.estimate, dollars)} | ${m.attempts ?? 0} | ${num(m.cost, dollars)} | ${num(m.minutes, Math.round)} | ${num(m.fiveHour)} | ${num(m.weekly)} |`;
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
    const issues = await allPages("/issues?state=open&labels=kind:piece");
    return Promise.all(issues.filter((i) => !i.pull_request).map(toPiece));
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
        const timer = setTimeout(() => {
            timedOut = true;
            log(`#${piece.number} ${step.meta.id} ran past ${SESSION_TIMEOUT_MS / 60_000} minutes; stopping it`);
            child.kill();
        }, SESSION_TIMEOUT_MS);
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
            resolve({ code, model, refused, timedOut, stats });
        });
    });

// The files a step's prompt draws on, read from the piece's branch.
const stepPrompt = (piece, step) => {
    const kind = stepKind(step.meta.id);
    const prompts = Object.fromEntries([...PROMPTS_FOR[kind], "session"].map((n) => [n, readRepo(`relay/prompts/${n}.md`)]));
    const designDir = path.join(REPO, "docs/design");
    const files = {
        prompts,
        brief: readRepo(`${specDir(piece)}/${step.meta.id}.md`),
        spec: readRepo(`${specDir(piece)}/spec.md`),
        design: Object.fromEntries(designPaths(piece.body).map((f) => [f, readRepo(f)])),
        designFiles: fs.existsSync(designDir) ? fs.readdirSync(designDir).map((f) => `docs/design/${f}`) : [],
        diffStat: kind === "review" ? git("diff", "--stat=120", "origin/main...HEAD") : "",
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
    for (const k of ["points", "estimate", "coin"]) if (s[k] != null) meta[k] = s[k];
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

// A new piece gets its spec step, on the model and effort the piece's body chose.
const startPiece = async (piece) => {
    const { model, effort, why } = specChoice(piece.body);
    await createStep(piece, { id: "spec", title: "Spec and briefs", model, effort, why });
    await setStatus(piece, "Spec");
};

const readSteps = (piece) => checkSteps(readRepo(`${specDir(piece)}/steps.json`) ?? "");

// After the spec, spec-review on the model the piece's body chose. After spec-review, every
// later step from steps.json as spec-review left it. Spec-review checked it before it closed,
// so an error here means the branch changed since; spec-review runs again to fix it.
const plan = async (piece) => {
    checkoutPiece(piece);
    const review = piece.steps.find((s) => s.meta.id === "spec-review");
    if (!review) {
        const choice = specReviewChoice(piece.body) ?? { model: "sonnet", effort: "medium", why: "The piece names no spec-review, so the datum." };
        await createStep(piece, { id: "spec-review", title: "Review the spec", ...choice });
        await setStatus(piece, "Spec");
        return;
    }
    const { steps, error } = readSteps(piece);
    if (error) {
        await github("PATCH", `/issues/${review.number}`, { state: "open" });
        await saveMeta(review, { ...review.meta, status: "pending", note: `steps.json can't be read: ${error}` });
        log(`#${piece.number}: ${error}; spec-review runs again`);
        return;
    }
    for (const s of steps) await createStep(piece, s);
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

// Merges the PR when the review says merge and CI is green. Returns why it didn't, or null.
const autoMerge = async (piece, pr, result) => {
    if (!pr) return "there's no PR";
    if (result.merge !== true) return "its verdict isn't merge";
    const { check_runs } = await github("GET", `/commits/${pr.head.sha}/check-runs?per_page=100`);
    if (!checksGreen(check_runs)) return "CI isn't green";
    try {
        await github("PUT", `/pulls/${pr.number}/merge`, { sha: pr.head.sha, merge_method: "squash", commit_title: `${piece.title} (#${pr.number})` });
    } catch (e) {
        return `merging failed: ${e.message}`;
    }
    log(`#${piece.number}: merged PR #${pr.number}`);
    await comment(piece.number, `🔀 The review's verdict is merge and CI is green, so ${pr.html_url} is merged.`);
    return null;
};

// The PR comment for a finished step: its summary, decisions and ideas, with what it took.
export const stepComment = (step, result) => {
    const m = step.meta;
    const took = [`${m.model} ${m.effort}${m.coin ? " (coin)" : ""}`, ...(m.points != null ? [`${m.points} points`] : []), dollars(m.cost), `${Math.round(m.minutes ?? 0)} min`, `#${step.number}`];
    const decided = m.decisions?.length ? `\n\n🧭 Decided:\n${m.decisions.map((d) => `- ${d.decision}${d.why ? `: ${d.why}` : ""}`).join("\n")}` : "";
    const list = ideaList(result.ideas);
    const ideas = list ? `\n\n💡 Ideas, not built:\n${list}` : "";
    return `**${m.id}** (${took.join(", ")}): ${result.summary}${decided}${ideas}`;
};

// What follows a finished step: the PR once the branch has commits, the next steps after the
// spec and spec-review, and the owner's turn after the piece's review.
const afterStep = async (piece, step, result) => {
    const kind = stepKind(step.meta.id);
    let pr = await findPr(piece);
    if (!pr && git("rev-list", `origin/main..origin/${branchOf(piece)}`)) {
        pr = await github("POST", "/pulls", { title: piece.title, head: branchOf(piece), base: "main", body: `Closes #${piece.number}.` });
        log(`#${piece.number}: PR #${pr.number}`);
        const tree = `https://github.com/${GH_REPO}/tree/${branchOf(piece)}/${specDir(piece)}`;
        await comment(piece.number, `📎 The PR is #${pr.number}, and the spec is in [\`${specDir(piece)}/\`](${tree}).`);
    }
    if (pr && result.summary) await comment(pr.number, stepComment(step, result));
    if (kind === "spec" || kind === "spec-review") await plan(await loadPiece(piece.number));
    else if (kind === "build") await setStatus(piece, "Building");
    else if (kind === "review") {
        const why = await autoMerge(piece, pr, result);
        if (why) {
            await comment(
                piece.number,
                `🔍 The review is done on ${pr?.html_url ?? "the PR"}, but ${why}. Its description ends with the review's verdict. Merge it, which closes this piece, or close this piece to keep the branch as \`proof/${piece.number}\`.`,
            );
            await setStatus(piece, "Review", "your merge");
        }
    }
};

// Runs one step and records how it went. Returns a line for the run report, or throws
// `refused` when the session couldn't start as asked.
// `before` is the usage the crew paced on, so the step's points are the rise from it.
const runStep = async (piece, step, before) => {
    const branch = checkoutPiece(piece);
    const kind = stepKind(step.meta.id);
    await setStatus(piece, { review: "Review", build: "Building" }[kind] ?? "Spec");
    let prompt;
    try {
        prompt = stepPrompt(piece, step);
    } catch (e) {
        // The spec can't brief the session; only a spec edit fixes that.
        step.body = withStatus(step.body, `blocked: ${e.message} (see #${piece.number})`);
        await saveMeta(step, { ...step.meta, status: "blocked" });
        await blockStep(piece, step, e.message);
        return `#${piece.number} ${step.meta.id}: blocked (${e.message})`;
    }
    await writeTokenFile();
    log(`start #${piece.number} ${step.meta.id} on ${step.meta.model}/${step.meta.effort} with Claude Code ${cliVersion()}`);
    const { code, model, refused, timedOut, stats } = await runSession(piece, step, prompt);
    if (refused) {
        tidyUp(branch);
        throw Object.assign(new Error(`stopped #${piece.number} ${step.meta.id} before it began: ${refused}`), { refused: true });
    }
    const exitOk = code === 0 && !timedOut && model !== null;
    let result = readResult(fs.existsSync(RESULT) ? fs.readFileSync(RESULT, "utf8") : null);
    log(`end #${piece.number} ${step.meta.id}: exit ${code}, model ${model}, ${dollars(stats.cost)}, ${stats.minutes.toFixed(1)} minutes, ${stats.context}% context, ${stats.compactions} compactions, result ${JSON.stringify(result)}`);
    tidyUp(branch);
    // Spec-review isn't done until the crew can make the steps.
    if (step.meta.id === "spec-review" && result?.status === "done") {
        const { error } = readSteps(piece);
        if (error) result = { status: "pending", note: `You reported done, but steps.json can't be read: ${error}` };
    }
    let after = null;
    try {
        after = await usage();
    } catch (e) {
        log(`couldn't read usage after the step: ${e.message}`);
    }

    const applied = applyResult(step.meta, result, exitOk);
    const { outcome, why } = applied;
    const { meta } = applied;
    const points = usagePoints(before, after);
    const add = (was, n) => (n == null ? was ?? null : (was ?? 0) + n);
    meta.cost = cents((step.meta.cost ?? 0) + stats.cost);
    meta.minutes = Math.round(((step.meta.minutes ?? 0) + stats.minutes) * 10) / 10;
    meta.fiveHour = add(step.meta.fiveHour, points.fiveHour);
    meta.weekly = add(step.meta.weekly, points.weekly);
    meta.usage = [...(step.meta.usage ?? []), { before: before && { session: before.session, weekly: before.weekly }, after: after && { session: after.session, weekly: after.weekly } }];
    meta.context = stats.context;
    meta.compactions = (step.meta.compactions ?? 0) + stats.compactions;
    meta.cli = cliVersion();
    step.body = withStatus(step.body, statusLine(meta, outcome, outcome === "blocked" ? `${why} (see #${piece.number})` : meta.note));
    await saveMeta(step, meta);
    await setBoard(step.nodeId, {
        Status: outcome === "done" ? "Done" : outcome === "blocked" ? "Waiting" : "Building",
        Attempts: meta.attempts ?? 0,
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
    return `#${piece.number} ${step.meta.id}: ${outcome}`;
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

const logIssue = async () => {
    const found = (await allPages("/issues?state=open&labels=kind:chore")).find((i) => i.title === LOG_TITLE);
    if (found) return found.number;
    const issue = await github("POST", "/issues", {
        title: LOG_TITLE,
        body: "Each Relay run reports here. Pieces report on their own issues.",
        labels: ["kind:chore"],
    });
    return issue.number;
};

// Runs steps until something stops it, and says why.
const runSteps = async (ran) => {
    for (;;) {
        if (Date.now() - STARTED > DEADLINE_MS) return "deadline: no time for another step in this job";
        const pieces = await loadPieces();
        const next = nextWork(pieces);
        if (!next) return "nothing to do: every ready piece is waiting, paused, held or finished";
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
        const before = await usage();
        const verdict = pace(before, new Date());
        if (!verdict.ok) {
            await setBoard(next.piece.nodeId, { "Waiting on": waitingOn(verdict) });
            return `budget stop: ${verdict.reason}`;
        }
        log(verdict.reason);
        ran.push(await runStep(next.piece, next.step, before));
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
        await comment(await logIssue(), `🏃 Run finished: ${reason}\n\n${steps}${url}`);
    }
};

const status = async () => {
    const pieces = await loadPieces();
    for (const p of pieces) console.log(`#${p.number} ${p.title}: ${holdReason(p) ?? JSON.stringify(pieceAction(p)?.action ?? "finished")}`);
    const next = nextWork(pieces);
    console.log(next ? `next: #${next.piece.number} ${next.action}${next.step ? ` ${next.step.meta.id}` : ""}` : "next: nothing");
    console.log(`pacing: ${JSON.stringify(pace(await usage(), new Date()))}`);
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
        return "Paused. A step that's running finishes first.";
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

export const HELP = "Commands: `run`, `retry [step]`, `skip <step>`, `model <step> <model> [effort]`, `pause`, `resume`, `status`.";

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
};

const event = async () => {
    const e = JSON.parse(fs.readFileSync(process.env.GITHUB_EVENT_PATH, "utf8"));
    if (e.action === "closed" && !e.comment) {
        if (e.issue.labels.some((l) => l.name === "kind:piece")) await closePiece(e.issue.number);
        return;
    }
    if (e.comment?.user?.login !== OWNER || e.issue?.pull_request) return log("not the owner's comment on an issue; ignoring");
    const labels = e.issue.labels.map((l) => l.name);
    const cmd = parseCommand(e.comment.body);
    if (!labels.includes("kind:piece")) {
        if (cmd) await comment(e.issue.number, "`/relay` commands go on the piece's issue.");
        return;
    }
    const piece = await loadPiece(e.issue.number);
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
