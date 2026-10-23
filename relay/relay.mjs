// The Relay crew: builds Sans Fiat one piece at a time, one `claude -p` session per step,
// on GitHub Actions (.github/workflows/relay.yml). Ported from Tortoise Spelling's Jogging.
//
//   node relay/relay.mjs run       # run steps until the budget, the deadline or the work runs out
//   node relay/relay.mjs status    # print the next step and the pacing verdict, run nothing
//   node relay/relay.mjs event     # act on Alan's comment (relay-events.yml)
//   node relay/relay.mjs report <message>   # comment on the Relay log issue
//
// GitHub holds the state. A piece is an open `kind:piece` issue labelled `relay:ready`; its
// steps are `kind:step` sub-issues whose bodies carry a `<!-- relay {...} -->` block with the
// step's model, effort and attempts. The spec and briefs live on the piece's branch,
// `relay/<number>`. A session reports how its step ended in relay/runs/result.json, and the
// crew is the only one that closes a step. relay/README.md explains the whole loop.
//
// Environment: RELAY_APP_ID and RELAY_APP_KEY (the App, see token.mjs), CLAUDE_CODE_OAUTH_TOKEN,
// and for `event` GITHUB_EVENT_PATH and DISPATCH_TOKEN (the workflow's own token, for `/relay run`).

import { execFileSync, spawn } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { appToken } from "./token.mjs";

const HERE = path.dirname(fileURLToPath(import.meta.url));
const REPO = path.dirname(HERE);
const RUNS = path.join(HERE, "runs");
const RESULT = path.join(RUNS, "result.json");
export const ALAN = "DontPanic345";
const GH_REPO = `${ALAN}/sansfiat`;
const BOARD = "Sans Fiat Work";
const LOG_TITLE = "Relay log";
const WORKFLOW = "relay.yml";
const MAX_ATTEMPTS = 2;
export const MAX_BUILDS = 4;
const STARTED = Date.now();
// The job is killed at 345 minutes. No step starts after the deadline, and none runs past
// its timeout, so the crew always gets to record the last one.
const DEADLINE_MS = Number(process.env.RELAY_DEADLINE_MIN ?? 250) * 60_000;
const SESSION_TIMEOUT_MS = Number(process.env.RELAY_SESSION_MIN ?? 90) * 60_000;
// Written into transcripts, which become a workflow artifact anyone signed in can read.
export const SECRETS = ["CLAUDE_CODE_OAUTH_TOKEN", "RELAY_APP_KEY", "DISPATCH_TOKEN"];

export const MODELS = ["haiku", "sonnet", "opus"];
export const EFFORTS = ["low", "medium", "high"];
export const TYPES = ["proof", "merge", "polish"];

// ---- Pure logic (covered by relay.test.mjs) ----

// Continue while the 5-hour window is under 50% and the weekly window is under
// day-of-cycle / 7 × 95%.
export const pace = ({ session, weekly, weeklyResetsAt }, now) => {
    const cycleStart = weeklyResetsAt.getTime() - 7 * 86400_000;
    const day = Math.min(7, Math.max(1, Math.floor((now.getTime() - cycleStart) / 86400_000) + 1));
    const weeklyLimit = (day / 7) * 95;
    if (session >= 50) return { ok: false, reason: `5-hour usage ${session}% ≥ 50%` };
    if (weekly >= weeklyLimit) {
        return { ok: false, reason: `weekly usage ${weekly}% ≥ ${weeklyLimit.toFixed(1)}% (day ${day})` };
    }
    return { ok: true, reason: `5-hour ${session}%, weekly ${weekly}% < ${weeklyLimit.toFixed(1)}% (day ${day})` };
};

export const parseUsage = (raw) => {
    const limits = raw.limits ?? [];
    const session = limits.find((l) => l.kind === "session");
    const weekly = limits.find((l) => l.kind === "weekly_all");
    if (session && weekly) {
        return { session: session.percent, weekly: weekly.percent, weeklyResetsAt: new Date(weekly.resets_at) };
    }
    if (raw.five_hour && raw.seven_day) {
        return {
            session: raw.five_hour.utilization,
            weekly: raw.seven_day.utilization,
            weeklyResetsAt: new Date(raw.seven_day.resets_at),
        };
    }
    const uw = raw.unifiedWindows;
    if (uw?.five_hour && uw?.seven_day) {
        return {
            session: uw.five_hour.utilization * 100,
            weekly: uw.seven_day.utilization * 100,
            weeklyResetsAt: new Date(uw.seven_day.resetsAt * 1000),
        };
    }
    throw new Error("no session/weekly windows in the usage payload");
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

// An issue form's answers, by heading: `### In scope` → `form["in scope"]`.
export const parseForm = (body) => {
    const form = {};
    for (const part of (body ?? "").split(/^### /m).slice(1)) {
        const nl = part.indexOf("\n");
        const value = part.slice(nl + 1).trim();
        form[part.slice(0, nl).trim().toLowerCase()] = value === "_No response_" ? "" : value;
    }
    return form;
};

export const pieceType = (labels) => TYPES.find((t) => labels.includes(`type:${t}`)) ?? null;

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

// The steps a spec session wrote to `relay/specs/<n>/steps.json`, checked. Returns
// { steps } or { error }.
export const checkSteps = (text) => {
    let steps;
    try {
        steps = JSON.parse(text);
    } catch (e) {
        return { error: `steps.json isn't JSON: ${e.message}` };
    }
    if (!Array.isArray(steps) || steps.length < 2) return { error: "steps.json must list the build steps and the review" };
    const builds = steps.slice(0, -1);
    if (steps.at(-1).id !== "review") return { error: "the last step in steps.json must be `review`" };
    if (builds.length > MAX_BUILDS) return { error: `steps.json has ${builds.length} build steps; at most ${MAX_BUILDS}` };
    for (const [i, s] of builds.entries()) {
        if (s.id !== `build-${i + 1}`) return { error: `step ${i + 1} in steps.json must be \`build-${i + 1}\`, not \`${s.id}\`` };
    }
    for (const s of steps) {
        if (!MODELS.includes(s.model)) return { error: `${s.id}: model must be one of ${MODELS.join(", ")}` };
        if (!EFFORTS.includes(s.effort)) return { error: `${s.id}: effort must be one of ${EFFORTS.join(", ")}` };
        if (!s.title) return { error: `${s.id}: no title` };
    }
    return { steps };
};

// The milestone number in a title such as "M2 Pre-game", for ordering gates.
const milestoneRank = (m) => (m ? Number(/M(\d+)/i.exec(m.title)?.[1] ?? 999) : 9999);

// Why a piece can't run now, or null when it can. `pieces` are every open piece, ready or
// not, and `openMilestones` the repo's open milestones.
export const holdReason = (piece, pieces, openMilestones) => {
    if (!piece.labels.includes("relay:ready")) return "not ready";
    // Anyone can open an issue, but only Alan's words reach a session.
    if (piece.author !== ALAN) return "not opened by Alan";
    if (piece.labels.includes("relay:paused")) return "paused";
    if (piece.labels.includes("waiting")) return "waiting on Alan";
    if (!pieceType(piece.labels)) return "no type label";
    if (pieceType(piece.labels) === "merge") {
        // Merges never overlap, and a milestone's merges wait for the gates before it.
        const earlier = pieces.find((p) => p.number < piece.number && pieceType(p.labels) === "merge");
        if (earlier) return `merge #${earlier.number} goes first`;
        const gate = openMilestones.find((m) => milestoneRank(m) < milestoneRank(piece.milestone));
        if (gate) return `waiting for the ${gate.title} gate`;
    }
    return null;
};

// What the crew does next for one piece: "start" (it has no steps yet), "step" (run
// `step`), "signoff" (the spec is reviewed but its build steps aren't made), or null.
export const pieceAction = (piece) => {
    if (piece.steps.length === 0) return { action: "start" };
    const steps = sortSteps(piece.steps);
    const open = steps.find((s) => s.state === "open");
    if (open) return open.meta.status === "blocked" ? null : { action: "step", step: open };
    const reviewed = steps.some((s) => s.meta.id === "spec-review");
    const built = steps.some((s) => stepKind(s.meta.id) === "build" || s.meta.id === "review");
    if (reviewed && !built) return { action: "signoff" };
    return null;
};

// The next piece and what to do with it: pieces in milestone order, then oldest first.
export const nextWork = (pieces, openMilestones) => {
    const order = [...pieces].sort(
        (a, b) => milestoneRank(a.milestone) - milestoneRank(b.milestone) || a.number - b.number,
    );
    for (const piece of order) {
        if (holdReason(piece, pieces, openMilestones)) continue;
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

// The text under a heading, e.g. "Scope map", or null. Matches a heading that starts with it.
export const section = (md, name) => {
    const lines = (md ?? "").split("\n");
    let fenced = false;
    let start = -1;
    let level = 0;
    for (const [i, line] of lines.entries()) {
        if (/^\s*(```|~~~)/.test(line)) fenced = !fenced;
        const m = !fenced && /^(#{1,6})\s+(.+?)\s*$/.exec(line);
        if (!m) continue;
        if (start >= 0 && m[1].length <= level) return lines.slice(start, i).join("\n").trim();
        if (start < 0 && m[2].toLowerCase().startsWith(name.toLowerCase())) {
            start = i;
            level = m[1].length;
        }
    }
    return start >= 0 ? lines.slice(start).join("\n").trim() : null;
};

// The `relay/prompts/` files each kind of step follows, besides `session.md`.
export const PROMPTS_FOR = {
    spec: ["spec"],
    "spec-review": ["spec-review", "spec"],
    build: ["build"],
    review: ["review"],
};

// A session's whole prompt: its step first, then only the material that serves it, then
// the rules for its kind of step. Throws when a build or review step has no brief; the
// crew blocks the step on that.
// `files` holds `prompts` (name → text), `brief`, `spec` and `design` (null when missing),
// `designFiles` (paths in docs/design/), and `diffStat` (review only).
export const sessionPrompt = (piece, step, files) => {
    const kind = stepKind(step.meta.id);
    const type = pieceType(piece.labels);
    const form = parseForm(piece.body);
    const designPath = form["design file"] || "(none given)";
    const parts = [
        `You are a Relay session for Sans Fiat, in GitHub Actions with no other context: one cold session ` +
            `doing one step of ${type} piece #${piece.number}, "${piece.title}". You are on its branch, ` +
            `\`${branchOf(piece)}\`, and its spec lives in \`${specDir(piece)}/\`. This prompt holds Relay's ` +
            `rules; the design and the code hold the rest.`,
    ];
    if (kind === "build" || kind === "review") {
        if (!files.brief) throw new Error(`${specDir(piece)}/${step.meta.id}.md is missing`);
        parts.push(atLevel(files.brief.trim(), 1));
    } else {
        const what = kind === "spec" ? "Write the spec, the briefs and steps.json" : "Review the spec, the briefs and steps.json";
        parts.push(`# Your step: #${piece.number} ${step.meta.id}\n\n${what} for this piece, as Alan wrote it:`);
        parts.push(atLevel(`# The piece\n\n${piece.body.trim()}`));
    }
    if (step.meta.note) parts.push(`# The last attempt didn't finish\n\nPick up from here.\n\n${step.meta.note}`);
    if (step.meta.reply) parts.push(`# Alan's reply to this step's blocker\n\nAct on it.\n\n"""${step.meta.reply}"""`);
    if (kind !== "build") {
        const design = files.design
            ? atLevel(files.design.trim(), 3)
            : "That file doesn't exist yet. Every rule the piece needs is then a design question for Alan.";
        const others = (files.designFiles ?? []).map((f) => `- \`${f}\``).join("\n") || "- (none yet)";
        parts.push(`# The design\n\nThe piece builds \`${designPath}\`:\n\n${design}\n\nThe other design files, to read as you need them:\n\n${others}`);
    }
    if (kind === "review") {
        parts.push(`# The diff to review\n\nThe piece's changes against \`main\`:\n\n\`\`\`\n${(files.diffStat ?? "").trim()}\n\`\`\``);
        if (files.spec) parts.push(`# The whole spec\n\n${atLevel(files.spec.trim())}`);
    }
    const done = sortSteps(piece.steps).filter((s) => s.state === "closed" && s.meta.summary);
    if (done.length) {
        parts.push(`# This piece so far\n\n${done.map((s) => `- **${s.meta.id}**: ${s.meta.summary}`).join("\n")}`);
    }
    for (const name of [...PROMPTS_FOR[kind], `type-${type}`, "session"]) {
        if (files.prompts[name]) parts.push(atLevel(files.prompts[name].trim(), 1));
    }
    parts.push(`Do this one step, then finish as "Finishing" says.`);
    return parts.join("\n\n") + "\n";
};

// The session's init frame says what it really got. A model the user's default overrode,
// or a session without auto mode, can't do the step.
export const initProblem = (frame, model) => {
    if (!frame.model?.includes(model)) return `model mismatch: wanted ${model}, got ${frame.model}`;
    if (frame.permissionMode !== "auto") return `permission mode is ${frame.permissionMode}, not auto`;
    return null;
};

// What a session used: every token across its models, the context its last turn held as a
// share of the step model's window, and how many times it compacted. Pieces are sized so a
// session shouldn't need to compact; a count above 0 says the step was too big.
export const sessionStats = (lastAssistant, result, model, compactions = 0) => {
    const usage = result?.modelUsage ?? {};
    const tokens = Object.values(usage).reduce(
        (n, u) => n + (u.inputTokens ?? 0) + (u.outputTokens ?? 0) + (u.cacheReadInputTokens ?? 0) + (u.cacheCreationInputTokens ?? 0),
        0,
    );
    const u = lastAssistant?.message?.usage;
    const held = u ? (u.input_tokens ?? 0) + (u.cache_creation_input_tokens ?? 0) + (u.cache_read_input_tokens ?? 0) + (u.output_tokens ?? 0) : 0;
    const window = Object.entries(usage).find(([k]) => k.includes(model))?.[1]?.contextWindow ?? 200_000;
    return { tokens, context: Math.round((held / window) * 100), compactions };
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

// The steps table for the sign-off and `/relay status`.
export const stepsTable = (steps) =>
    [
        "| Step | State | Model | Effort | Attempts | Tokens | Context | Compactions | Why |",
        "|---|---|---|---|---|---|---|---|---|",
        ...sortSteps(steps).map((s) => {
            const state = s.state === "closed" ? (s.stateReason === "not_planned" ? "skipped" : "done") : s.meta.status ?? "pending";
            const why = (s.meta.why ?? "").replaceAll("|", "\\|");
            const tokens = s.meta.tokens ? s.meta.tokens.toLocaleString("en-NZ") : "";
            const context = s.meta.context != null ? `${s.meta.context}%` : "";
            return `| ${s.meta.id} | ${state} | ${s.meta.model} | ${s.meta.effort} | ${s.meta.attempts ?? 0} | ${tokens} | ${context} | ${s.meta.compactions ?? ""} | ${why} |`;
        }),
    ].join("\n");

export const redact = (text, secrets) => secrets.filter((s) => s && s.length >= 8).reduce((t, s) => t.split(s).join("***"), text);

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

const graphql = async (query, variables) => {
    const res = await request("POST", "https://api.github.com/graphql", { query, variables });
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

const toPiece = async (issue) => ({
    number: issue.number,
    nodeId: issue.node_id,
    title: issue.title,
    body: issue.body ?? "",
    author: issue.user.login,
    labels: issue.labels.map((l) => l.name),
    milestone: issue.milestone,
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

// ---- The board (best effort: the crew works without it) ----

let board;

const loadBoard = async () => {
    if (board !== undefined) return board;
    try {
        const data = await graphql(
            `query($login: String!, $q: String!) { user(login: $login) { projectsV2(first: 5, query: $q) { nodes {
                id title fields(first: 50) { nodes {
                    ... on ProjectV2Field { id name dataType }
                    ... on ProjectV2SingleSelectField { id name dataType options { id name } } } } } } } }`,
            { login: ALAN, q: BOARD },
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
// an option name or a number.
const setBoard = async (nodeId, values) => {
    const b = await loadBoard();
    if (!b) return;
    try {
        const added = await graphql(
            `mutation($p: ID!, $c: ID!) { addProjectV2ItemById(input: { projectId: $p, contentId: $c }) { item { id } } }`,
            { p: b.id, c: nodeId },
        );
        const item = added.addProjectV2ItemById.item.id;
        for (const [name, value] of Object.entries(values)) {
            const field = b.fields[name];
            if (!field || value == null) continue;
            let v;
            if (field.options) {
                const option = field.options.find((o) => o.name.toLowerCase() === String(value).toLowerCase());
                if (!option) continue;
                v = { singleSelectOptionId: option.id };
            } else if (field.dataType === "NUMBER") {
                v = { number: Number(value) };
            } else continue;
            await graphql(
                `mutation($p: ID!, $i: ID!, $f: ID!, $v: ProjectV2FieldValue!) {
                    updateProjectV2ItemFieldValue(input: { projectId: $p, itemId: $i, fieldId: $f, value: $v }) { projectV2Item { id } } }`,
                { p: b.id, i: item, f: field.id, v },
            );
        }
    } catch (e) {
        log(`couldn't update the board: ${e.message}`);
    }
};

const setStatus = (piece, status) => setBoard(piece.nodeId, { Status: status, "Piece type": pieceType(piece.labels) });

// ---- Git ----

// Alan may push to a piece's branch while a run is going, so every push rebases first.
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
            resolve({ code, model, refused, timedOut, stats: sessionStats(lastAssistant, result, step.meta.model, compactions) });
        });
    });

// The files a step's prompt draws on, read from the piece's branch.
const stepPrompt = (piece, step) => {
    const kind = stepKind(step.meta.id);
    const type = pieceType(piece.labels);
    const prompts = Object.fromEntries(
        [...PROMPTS_FOR[kind], `type-${type}`, "session"].map((n) => [n, readRepo(`relay/prompts/${n}.md`)]),
    );
    const designPath = parseForm(piece.body)["design file"];
    const designDir = path.join(REPO, "docs/design");
    const files = {
        prompts,
        brief: readRepo(`${specDir(piece)}/${step.meta.id}.md`),
        spec: readRepo(`${specDir(piece)}/spec.md`),
        design: designPath && /^docs\/design\/[\w./-]+$/.test(designPath) ? readRepo(designPath) : null,
        designFiles: fs.existsSync(designDir) ? fs.readdirSync(designDir).map((f) => `docs/design/${f}`) : [],
        diffStat: kind === "review" ? git("diff", "--stat=120", "origin/main...HEAD") : "",
    };
    return sessionPrompt(piece, step, files);
};

// ---- Piece transitions ----

const findPr = async (piece) => {
    const prs = await github("GET", `/pulls?state=open&head=${ALAN}:${encodeURIComponent(branchOf(piece))}`);
    return prs[0] ?? null;
};

const createStep = async (piece, s) => {
    const meta = { id: s.id, model: s.model, effort: s.effort, why: s.why ?? "", attempts: 0, status: "pending" };
    const issue = await github("POST", "/issues", {
        title: `#${piece.number} ${s.id}: ${s.title}`,
        body: writeMeta(`Step \`${s.id}\` of #${piece.number}.${s.why ? `\n\n${s.why}` : ""}`, meta),
        labels: ["kind:step"],
    });
    await github("POST", `/issues/${piece.number}/sub_issues`, { sub_issue_id: issue.id });
    await setBoard(issue.node_id, { Step: stepKind(s.id), Model: s.model, Effort: s.effort, Status: "Ready", Attempts: 0 });
    log(`#${piece.number}: made step ${s.id} (#${issue.number})`);
};

// A new piece gets its spec and spec-review steps. Alan can change their models on the
// step issues, or with `/relay model`, before they run.
const startPiece = async (piece) => {
    await createStep(piece, { id: "spec", title: "Spec and briefs", model: "opus", effort: "high", why: "Writes the spec, briefs and steps" });
    await createStep(piece, { id: "spec-review", title: "Review the spec", model: "sonnet", effort: "high", why: "Checks the briefs and cuts work outside the scope" });
    await setStatus(piece, "Spec");
};

// After the spec review: the build and review steps are made from steps.json, and the
// piece waits for Alan's `/relay go`.
const signOff = async (piece) => {
    checkoutPiece(piece);
    const { steps, error } = checkSteps(readRepo(`${specDir(piece)}/steps.json`) ?? "");
    if (error) {
        await addLabel(piece.number, "waiting");
        await comment(piece.number, `⛔ The spec's steps can't be read: ${error}\n\nFix \`${specDir(piece)}/steps.json\` on \`${branchOf(piece)}\`, then \`/relay go\`.`);
        await setStatus(piece, "Waiting");
        return;
    }
    // Waiting first: if the crew stops partway, no build starts before Alan signs off.
    await addLabel(piece.number, "waiting");
    for (const s of steps) await createStep(piece, s);
    const fresh = await loadPiece(piece.number);
    const pr = await findPr(piece);
    await comment(
        piece.number,
        [
            `📝 The spec is reviewed and ready for your sign-off: \`${specDir(piece)}/spec.md\`${pr ? ` in ${pr.html_url}` : ""}.`,
            "",
            stepsTable(fresh.steps.filter((s) => s.state === "open")),
            "",
            "Answer any design questions in `docs/design/` first. Then:",
            "- `/relay model <step> <model> [effort]` to change a step",
            "- `/relay go` to start building",
        ].join("\n"),
    );
    await setStatus(piece, "Sign-off");
};

const blockStep = async (piece, step, why) => {
    await addLabel(piece.number, "waiting");
    await comment(
        piece.number,
        `⛔ **${step.meta.id}** is blocked: ${why}\n\nReply here to unblock it (your reply goes to the next session), or \`/relay retry\`, or \`/relay skip ${step.meta.id}\`.`,
    );
    await setStatus(piece, "Waiting");
};

// Ideas a session noticed become issues for Alan to triage, never more code in the piece.
const raiseIdeas = async (piece, ideas) => {
    for (const idea of (ideas ?? []).filter((i) => i?.title)) {
        const issue = await github("POST", "/issues", {
            title: idea.title,
            body: `${idea.body ?? ""}\n\nRaised by #${piece.number} (Relay).`.trim(),
            labels: ["kind:idea"],
        });
        log(`#${piece.number}: idea -> #${issue.number}`);
    }
};

// What follows a finished step: a draft PR after the spec, the sign-off after its review,
// and Alan's turn after the piece's review.
const afterStep = async (piece, step, result) => {
    const kind = stepKind(step.meta.id);
    const type = pieceType(piece.labels);
    let pr = await findPr(piece);
    if (!pr && git("rev-list", `origin/main..origin/${branchOf(piece)}`)) {
        pr = await github("POST", "/pulls", {
            title: piece.title,
            head: branchOf(piece),
            base: "main",
            draft: true,
            body: type === "proof" ? `Proof for #${piece.number}. Not to be merged: the branch is the proof.` : `Closes #${piece.number}.`,
        });
        log(`#${piece.number}: draft PR #${pr.number}`);
    }
    if (pr && result.summary) await comment(pr.number, `**${step.meta.id}** (${step.meta.model}/${step.meta.effort}): ${result.summary}`);
    if (kind === "spec-review" && result.scopeCut) {
        await addLabel(piece.number, "scope-cut");
        await comment(piece.number, `✂️ The spec review cut work outside the scope: ${result.scopeCut}`);
    }
    if (kind === "spec-review") await signOff(piece);
    else if (kind === "spec") await setStatus(piece, "Spec");
    else if (kind === "build") await setStatus(piece, "Building");
    else if (kind === "review") {
        await comment(
            piece.number,
            type === "proof"
                ? `🔍 The review is done. Read the findings in ${pr?.html_url ?? "the PR"}, then close this piece. The PR isn't merged: the branch is the proof.`
                : `🔍 The review is done and CI has run on ${pr?.html_url ?? "the PR"}. Approve and merge it; that closes this piece.`,
        );
        await setStatus(piece, "Review");
    }
};

// Runs one step and records how it went. Returns a line for the run report, or throws
// `refused` when the session couldn't start as asked.
const runStep = async (piece, step) => {
    const branch = checkoutPiece(piece);
    if (step.meta.id === "review") {
        const pr = await findPr(piece);
        // CI runs on the whole piece once the PR is ready.
        if (pr?.draft) {
            await graphql(`mutation($id: ID!) { markPullRequestReadyForReview(input: { pullRequestId: $id }) { clientMutationId } }`, { id: pr.node_id });
        }
        await setStatus(piece, "Review");
    } else if (stepKind(step.meta.id) === "build") {
        await setStatus(piece, "Building");
    }
    let prompt;
    try {
        prompt = stepPrompt(piece, step);
    } catch (e) {
        // The spec can't brief the session; only a spec edit fixes that.
        await saveMeta(step, { ...step.meta, status: "blocked" });
        await blockStep(piece, step, e.message);
        return `#${piece.number} ${step.meta.id}: blocked (${e.message})`;
    }
    await writeTokenFile();
    log(`start #${piece.number} ${step.meta.id} on ${step.meta.model}/${step.meta.effort}`);
    const { code, model, refused, timedOut, stats } = await runSession(piece, step, prompt);
    if (refused) {
        tidyUp(branch);
        throw Object.assign(new Error(`stopped #${piece.number} ${step.meta.id} before it began: ${refused}`), { refused: true });
    }
    const exitOk = code === 0 && !timedOut && model !== null;
    const result = readResult(fs.existsSync(RESULT) ? fs.readFileSync(RESULT, "utf8") : null);
    log(`end #${piece.number} ${step.meta.id}: exit ${code}, model ${model}, ${stats.tokens} tokens, ${stats.context}% context, ${stats.compactions} compactions, result ${JSON.stringify(result)}`);
    tidyUp(branch);

    const { meta, outcome, why } = applyResult(step.meta, result, exitOk);
    meta.tokens = (step.meta.tokens ?? 0) + stats.tokens;
    meta.context = stats.context;
    meta.compactions = (step.meta.compactions ?? 0) + stats.compactions;
    await saveMeta(step, meta);
    await setBoard(step.nodeId, { Attempts: meta.attempts ?? 0, Tokens: meta.tokens, Context: meta.context, Compactions: meta.compactions, Status: outcome === "done" ? "Done" : outcome === "blocked" ? "Waiting" : "Building" });
    if (exitOk) await raiseIdeas(piece, result?.ideas);
    if (outcome === "done") {
        await comment(step.number, `✅ ${result.summary ?? "Done."}\n\n${stats.tokens.toLocaleString("en-NZ")} tokens, ${stats.context}% context, ${stats.compactions} compactions.`);
        await closeStep(step);
        await afterStep(piece, step, result);
    } else if (outcome === "blocked") {
        await blockStep(piece, step, why);
    } else {
        await comment(step.number, `↩️ Not finished (attempt ${meta.attempts} of ${MAX_ATTEMPTS}): ${meta.note}`);
    }
    return `#${piece.number} ${step.meta.id}: ${outcome}`;
};

// ---- Commands ----

const readUsage = () => {
    const out = sh("python3", [path.join(HERE, "usage.py"), "--json"], { timeout: 240_000 });
    return parseUsage(JSON.parse(out).raw);
};

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
        const milestones = await github("GET", "/milestones?state=open&per_page=100");
        const next = nextWork(pieces, milestones);
        if (!next) return "nothing to do: every ready piece is waiting, paused, held or finished";
        if (next.action === "start") {
            await startPiece(next.piece);
            ran.push(`#${next.piece.number}: started`);
            continue;
        }
        if (next.action === "signoff") {
            await signOff(next.piece);
            ran.push(`#${next.piece.number}: waiting for sign-off`);
            continue;
        }
        const verdict = pace(readUsage(), new Date());
        if (!verdict.ok) return `budget stop: ${verdict.reason}`;
        log(verdict.reason);
        ran.push(await runStep(next.piece, next.step));
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
    const milestones = await github("GET", "/milestones?state=open&per_page=100");
    for (const p of pieces) console.log(`#${p.number} ${p.title}: ${holdReason(p, pieces, milestones) ?? JSON.stringify(pieceAction(p)?.action ?? "finished")}`);
    const next = nextWork(pieces, milestones);
    console.log(next ? `next: #${next.piece.number} ${next.action}${next.step ? ` ${next.step.meta.id}` : ""}` : "next: nothing");
    console.log(`pacing: ${JSON.stringify(pace(readUsage(), new Date()))}`);
};

// For the workflow to report a failure the crew couldn't, e.g. the job timing out.
const report = async () => comment(await logIssue(), `⚠️ ${process.argv.slice(3).join(" ")}`);

// ---- Events: Alan's comments on a piece ----

const findStep = (piece, id) => piece.steps.find((s) => s.meta.id === id && s.state === "open");

// Alan's reply on a piece with a blocked step goes to that step's next session.
const unblock = async (piece, step, reply) => {
    const meta = { ...step.meta, status: "pending", attempts: 0 };
    if (reply) meta.reply = reply;
    delete meta.note;
    await saveMeta(step, meta);
    if (!piece.steps.some((s) => s !== step && s.state === "open" && s.meta.status === "blocked")) await removeLabel(piece.number, "waiting");
    await setStatus(piece, "Building");
};

const COMMANDS = {
    go: async (piece) => {
        const built = piece.steps.some((s) => stepKind(s.meta.id) === "build");
        const reviewed = piece.steps.some((s) => s.meta.id === "spec-review" && s.state === "closed");
        if (!built && !reviewed) return "There's nothing to sign off yet: the spec isn't reviewed.";
        if (!built) {
            // steps.json couldn't be read; the next run reads it again.
            await removeLabel(piece.number, "waiting");
            return "The next run reads `steps.json` again and makes the build steps.";
        }
        if (piece.steps.some((s) => s.state === "open" && s.meta.status === "blocked")) return "A step is blocked: reply, `/relay retry` or `/relay skip <step>` instead.";
        await removeLabel(piece.number, "waiting");
        await setStatus(piece, "Building");
        return "Signed off. Building starts on the next run.";
    },
    model: async (piece, [id, ...rest]) => {
        const step = findStep(piece, id);
        if (!step) return `No open step \`${id}\`.`;
        const meta = applyModel(step.meta, rest);
        if (!meta) return `Name a model (${MODELS.join(", ")}) or an effort (${EFFORTS.join(", ")}).`;
        const was = `${step.meta.model}/${step.meta.effort}`;
        await saveMeta(step, { ...meta, why: `Alan's choice (the spec had ${was}).${step.meta.why ? ` ${step.meta.why}` : ""}` });
        await setBoard(step.nodeId, { Model: meta.model, Effort: meta.effort });
        return `${id}: ${was} → ${meta.model}/${meta.effort}.`;
    },
    retry: async (piece, [id]) => {
        const step = id ? findStep(piece, id) : piece.steps.find((s) => s.state === "open" && s.meta.status === "blocked");
        if (!step) return id ? `No open step \`${id}\`.` : "No step is blocked.";
        await unblock(piece, step, null);
        return `${step.meta.id} gets two fresh attempts from the next run.`;
    },
    skip: async (piece, [id]) => {
        const step = findStep(piece, id);
        if (!step) return `No open step \`${id}\`.`;
        await comment(step.number, "Skipped by Alan.");
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
    run: async () => {
        await github("POST", `/actions/workflows/${WORKFLOW}/dispatches`, { ref: "main", inputs: { command: "run" } }, process.env.DISPATCH_TOKEN);
        return "Started a run.";
    },
    scope: async (piece) => {
        let map = null;
        try {
            const file = await github("GET", `/contents/${specDir(piece)}/spec.md?ref=${encodeURIComponent(branchOf(piece))}`);
            map = section(Buffer.from(file.content, "base64").toString("utf8"), "Scope map");
        } catch {
            // no spec yet
        }
        const ideas = (await allPages("/issues?state=all&labels=kind:idea")).filter((i) => (i.body ?? "").includes(`Raised by #${piece.number} `));
        return [
            map ?? "No scope map yet: the spec isn't written.",
            piece.labels.includes("scope-cut") ? "\nThe spec review cut work from this piece; see its ✂️ comment." : "",
            ideas.length ? `\nIdeas raised so far:\n${ideas.map((i) => `- #${i.number} ${i.title} (${i.state})`).join("\n")}` : "\nNo ideas raised so far.",
        ].join("\n");
    },
};

export const HELP = "Commands: `go`, `model <step> <model> [effort]`, `retry [step]`, `skip <step>`, `pause`, `resume`, `status`, `run`, `scope`.";

const event = async () => {
    const e = JSON.parse(fs.readFileSync(process.env.GITHUB_EVENT_PATH, "utf8"));
    if (e.comment?.user?.login !== ALAN || e.issue?.pull_request) return log("not Alan's comment on an issue; ignoring");
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
        await comment(piece.number, `Picked up. ${blocked.meta.id} carries on from the next run with your reply.`);
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
