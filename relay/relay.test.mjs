import assert from "node:assert/strict";
import { test } from "node:test";
import {
    applyModel,
    applyResult,
    atLevel,
    checkSteps,
    designPaths,
    dollars,
    endsOnMain,
    holdReason,
    initProblem,
    ideaList,
    nextWork,
    pace,
    parseCommand,
    pieceAction,
    pieceField,
    readMeta,
    readResult,
    redact,
    sessionPrompt,
    sessionStats,
    specChoice,
    stepsTable,
    writeMeta,
} from "./relay.mjs";

const resets = new Date("2026-10-17T09:00:00Z");
const day = (n) => new Date(resets.getTime() - 7 * 86400_000 + (n - 1) * 86400_000 + 3600_000);

test("pace stops at half the 5-hour window", () => {
    assert.equal(pace({ session: 49, weekly: 0, weeklyResetsAt: resets }, day(1)).ok, true);
    assert.equal(pace({ session: 50, weekly: 0, weeklyResetsAt: resets }, day(1)).ok, false);
});

test("pace spreads the weekly budget across the cycle", () => {
    assert.equal(pace({ session: 0, weekly: 12, weeklyResetsAt: resets }, day(1)).ok, true);
    assert.equal(pace({ session: 0, weekly: 14, weeklyResetsAt: resets }, day(1)).ok, false);
    assert.equal(pace({ session: 0, weekly: 94, weeklyResetsAt: resets }, day(7)).ok, true);
    assert.equal(pace({ session: 0, weekly: 95, weeklyResetsAt: resets }, day(7)).ok, false);
});

test("a step's meta block round-trips and replaces itself", () => {
    const body = writeMeta("Step `build-1` of #3.", { id: "build-1", model: "sonnet" });
    assert.deepEqual(readMeta(body), { id: "build-1", model: "sonnet" });
    const again = writeMeta(body, { id: "build-1", model: "opus" });
    assert.equal(readMeta(again).model, "opus");
    assert.equal(again.match(/<!-- relay/g).length, 1);
    assert.equal(readMeta("no block"), null);
    const tricky = writeMeta("", { summary: "a --> b } -->" });
    assert.equal(readMeta(tricky).summary, "a --> b } -->");
});

test("a piece's body says how it ends, the spec's model and its design files", () => {
    const body = "**Goal:** rivers.\n\n- **Ends**: on main\n**Spec:** Opus high, because rivers touch every mechanic.\n\nBuilds docs/design/map.md and docs/design/map.md.";
    assert.equal(pieceField(body, "Goal"), "rivers.");
    assert.equal(endsOnMain(body), true);
    assert.equal(endsOnMain("**Ends:** on its branch"), false);
    assert.equal(endsOnMain("no line"), null);
    assert.deepEqual(specChoice(body), { model: "opus", effort: "high", why: "Opus high, because rivers touch every mechanic." });
    assert.equal(specChoice("**Spec:** opus"), null);
    assert.deepEqual(designPaths(body), ["docs/design/map.md"]);
});

test("checkSteps wants the spec-review, builds in order, then the review, each with a why", () => {
    const s = (id, extra = {}) => ({ id, title: "t", model: "sonnet", effort: "medium", why: "w", ...extra });
    const ok = (...ids) => JSON.stringify(ids.map((id) => (typeof id === "string" ? s(id) : id)));
    assert.ok(checkSteps(ok("spec-review", "build-1", "review")).steps);
    assert.match(checkSteps("nope").error, /JSON/);
    assert.match(checkSteps(ok("build-1", "review")).error, /spec-review/);
    assert.match(checkSteps(ok("spec-review", "review")).error, /spec-review/);
    assert.match(checkSteps(ok("spec-review", "build-1", "build-3", "review")).error, /build-2/);
    assert.match(checkSteps(ok("spec-review", "build-1", "build-2")).error, /review/);
    assert.match(checkSteps(ok("spec-review", s("build-1", { model: "gpt" }), "review")).error, /model/);
    assert.match(checkSteps(ok("spec-review", s("build-1", { effort: "max" }), "review")).error, /effort/);
    assert.match(checkSteps(ok("spec-review", s("build-1", { why: "" }), "review")).error, /no why/);
});

const step = (id, state = "open", meta = {}) => ({ number: 100, state, meta: { id, model: "sonnet", effort: "medium", ...meta } });
const piece = (number, labels, steps = [], extra = {}) => ({
    number,
    title: `Piece ${number}`,
    body: "**Goal:** rivers.\n**Ends:** on its branch\n**Spec:** sonnet high, because it's one module.\n**Design:** docs/design/map.md\n\n**In scope**\n\n- rivers",
    labels: ["kind:piece", "relay:ready", ...labels],
    author: "DontPanic345",
    milestone: null,
    steps,
    ...extra,
});

test("pieceAction starts, plans after the spec, then runs each open step in order", () => {
    assert.deepEqual(pieceAction(piece(1, [])), { action: "start" });
    assert.equal(pieceAction(piece(1, [], [step("spec")])).step.meta.id, "spec");
    assert.deepEqual(pieceAction(piece(1, [], [step("spec", "closed")])), { action: "plan" });
    const planned = piece(1, [], [step("spec", "closed"), step("review"), step("build-1"), step("spec-review")]);
    assert.equal(pieceAction(planned).step.meta.id, "spec-review");
    const building = piece(1, [], [step("spec", "closed"), step("spec-review", "closed"), step("review"), step("build-2"), step("build-1", "closed")]);
    assert.equal(pieceAction(building).step.meta.id, "build-2");
    const blocked = piece(1, [], [step("spec", "closed"), step("spec-review", "closed"), step("build-1", "open", { status: "blocked" })]);
    assert.equal(pieceAction(blocked), null);
    const finished = piece(1, [], [step("spec", "closed"), step("spec-review", "closed"), step("build-1", "closed"), step("review", "closed")]);
    assert.equal(pieceAction(finished), null);
});

test("holdReason keeps pieces that end on main one at a time, and needs the body's lines", () => {
    const main = (n) => piece(n, [], [], { body: "**Ends:** main\n**Spec:** opus medium, because." });
    const [a, b] = [main(5), main(7)];
    const proof = piece(8, []);
    assert.equal(holdReason(a, [a, b]), null);
    assert.match(holdReason(b, [a, b]), /#5 goes to main first/);
    assert.equal(holdReason(proof, [a, b, proof]), null);
    assert.equal(holdReason(piece(9, ["waiting"]), []), "waiting on the owner");
    assert.equal(holdReason(piece(9, ["relay:paused"]), []), "paused");
    assert.equal(holdReason({ ...piece(9, []), labels: ["kind:piece"] }, []), "not ready");
    assert.equal(holdReason({ ...piece(9, []), author: "someone" }, []), "not opened by the owner");
    assert.match(holdReason(piece(9, [], [], { body: "**Spec:** opus high" }), []), /Ends/);
    assert.match(holdReason(piece(9, [], [], { body: "**Ends:** branch" }), []), /Spec/);
    // Once the spec step exists, its model is on the step, not the body.
    assert.equal(holdReason(piece(9, [], [step("spec")], { body: "**Ends:** branch" }), []), null);
});

test("nextWork takes the oldest piece that can run", () => {
    const [older, newer] = [piece(2, []), piece(9, [])];
    assert.equal(nextWork([newer, older]).piece.number, 2);
    const waiting = piece(1, ["waiting"]);
    assert.equal(nextWork([waiting, newer]).piece.number, 9);
    assert.equal(nextWork([waiting]), null);
});

test("applyResult closes, retries once, then blocks", () => {
    const meta = { id: "build-1", attempts: 0, reply: "use the river map" };
    const done = applyResult(meta, { status: "done", summary: "Built it.", decisions: [{ decision: "Rivers flow east", why: "the map tilts" }, { why: "no decision" }] }, true);
    assert.equal(done.outcome, "done");
    assert.equal(done.meta.summary, "Built it.");
    assert.deepEqual(done.meta.decisions, [{ decision: "Rivers flow east", why: "the map tilts" }]);
    assert.equal(done.meta.reply, undefined);
    const first = applyResult(meta, { status: "pending", note: "half way" }, true);
    assert.deepEqual([first.outcome, first.meta.attempts, first.meta.note], ["pending", 1, "half way"]);
    const second = applyResult(first.meta, null, false);
    assert.equal(second.outcome, "blocked");
    assert.match(second.why, /2 times/);
    // A crashed session may not have read the owner's reply, so it's kept for the next one.
    assert.equal(applyResult(meta, null, false).meta.reply, "use the river map");
    const blocked = applyResult(meta, { status: "blocked", blocker: "Needs a Steam account" }, true);
    assert.deepEqual([blocked.outcome, blocked.why], ["blocked", "Needs a Steam account"]);
});

test("readResult accepts only a known status, and a blocker with its reason", () => {
    assert.equal(readResult('{"status":"done"}').status, "done");
    assert.equal(readResult('{"status":"blocked"}'), null);
    assert.equal(readResult('{"status":"great"}'), null);
    assert.equal(readResult("not json"), null);
    assert.equal(readResult(null), null);
});

test("sessionStats sums every model's tokens and measures the last turn's context", () => {
    const last = { message: { usage: { input_tokens: 10, cache_creation_input_tokens: 990, cache_read_input_tokens: 49_000, output_tokens: 0 } } };
    const result = {
        total_cost_usd: 1.25,
        modelUsage: {
            "claude-sonnet-x": { inputTokens: 100, outputTokens: 200, cacheReadInputTokens: 300, cacheCreationInputTokens: 400, contextWindow: 200_000 },
            "claude-haiku-x": { inputTokens: 5, outputTokens: 5, contextWindow: 200_000 },
        },
    };
    assert.deepEqual(sessionStats(last, result, "sonnet", 1), { tokens: 1010, cost: 1.25, context: 25, compactions: 1 });
    assert.deepEqual(sessionStats(null, null, "sonnet"), { tokens: 0, cost: 0, context: 0, compactions: 0 });
    assert.equal(dollars(2.143), "$2.14");
});

test("initProblem refuses the wrong model and anything but auto mode", () => {
    assert.equal(initProblem({ model: "claude-sonnet-x", permissionMode: "auto" }, "sonnet"), null);
    assert.match(initProblem({ model: "claude-opus-x", permissionMode: "auto" }, "sonnet"), /mismatch/);
    assert.match(initProblem({ model: "claude-sonnet-x", permissionMode: "default" }, "sonnet"), /not auto/);
    assert.match(initProblem(null, "sonnet"), /no init frame/);
});

test("parseCommand and applyModel read the owner's commands", () => {
    assert.deepEqual(parseCommand("Thanks.\n/relay model build-2 Opus high"), { name: "model", args: ["build-2", "Opus", "high"] });
    assert.deepEqual(parseCommand("/relay run"), { name: "run", args: [] });
    assert.equal(parseCommand("no command here"), null);
    const meta = { id: "build-2", model: "sonnet", effort: "medium" };
    assert.deepEqual(applyModel(meta, ["Opus", "high"]), { ...meta, model: "opus", effort: "high" });
    assert.deepEqual(applyModel(meta, ["low"]), { ...meta, effort: "low" });
    assert.equal(applyModel(meta, ["fast"]), null);
});

test("atLevel moves headings outside code fences", () => {
    assert.equal(atLevel("# A\n## B\n```\n# c\n```", 2), "## A\n### B\n```\n# c\n```");
});

const files = (extra = {}) => ({
    prompts: { spec: "# Spec step", build: "# Build step", review: "# Review step", session: "# Every session" },
    brief: "# build-1: Rivers\n\nBuild rivers.",
    spec: "# Spec\n\n## Decisions",
    design: { "docs/design/map.md": "# Map\n\nRivers run downhill.", "docs/design/sea.md": null },
    designFiles: ["docs/design/map.md", "docs/design/clans.md"],
    diffStat: " engine/core/src/world.rs | 10 +",
    ...extra,
});

test("a spec session gets the piece, its design files and the spec prompt", () => {
    const text = sessionPrompt(piece(4, []), step("spec"), files());
    assert.match(text, /piece #4/);
    assert.match(text, /relay\/4/);
    assert.match(text, /- rivers/);
    assert.match(text, /Rivers run downhill/);
    assert.match(text, /docs\/design\/sea\.md`\n\nThat file doesn't exist yet/);
    assert.match(text, /- `docs\/design\/clans\.md`/);
    assert.doesNotMatch(text, /- `docs\/design\/map\.md`/);
    assert.match(text, /# Spec step/);
    assert.ok(text.indexOf("# Spec step") < text.indexOf("# Every session"));
});

test("a build session gets its brief, not the design, and the owner's reply", () => {
    const p = piece(4, [], [step("spec", "closed", { summary: "Wrote the spec.", decisions: [{ decision: "Rivers flow east", why: "the map tilts" }] })]);
    const text = sessionPrompt(p, step("build-1", "open", { reply: "Use 0.5", note: "tests half done" }), files());
    assert.match(text, /^# build-1: Rivers/m);
    assert.doesNotMatch(text, /Rivers run downhill/);
    assert.match(text, /Use 0\.5/);
    assert.match(text, /tests half done/);
    assert.match(text, /\*\*spec\*\*: Wrote the spec\.\n  - Decided: Rivers flow east \(the map tilts\)/);
    assert.throws(() => sessionPrompt(p, step("build-2"), files({ brief: null })), /build-2\.md is missing/);
});

test("a review session gets the diff and the whole spec", () => {
    const text = sessionPrompt(piece(4, []), step("review"), files());
    assert.match(text, /world\.rs \| 10/);
    assert.match(text, /# The whole spec/);
});

test("stepsTable shows each step's state and spend", () => {
    const table = stepsTable([
        step("review"),
        { ...step("build-1", "closed", { tokens: 12345, cost: 1.5, context: 40, compactions: 0, attempts: 1 }), stateReason: "completed" },
        { ...step("spec", "closed", { cost: 2 }), stateReason: "completed" },
    ]);
    const rows = table.split("\n");
    assert.match(rows[3], /\| build-1 \| done \| sonnet \| medium \| 1 \| 12,345 \| \$1\.50 \| 40% \| 0 \|/);
    assert.match(rows[4], /\| review \| pending \|/);
    assert.equal(rows.at(-1), "$3.50 at list price so far.");
});

test("redact hides secrets of eight characters or more, and any GitHub token", () => {
    assert.equal(redact("token abcdefgh and xy", ["abcdefgh", "xy"]), "token *** and xy");
    const ghs = "ghs_" + "a1B2".repeat(9);
    assert.equal(redact(`password=${ghs}\n`, []), "password=***\n");
    assert.equal(redact("ghost_writer ghs_short", []), "ghost_writer ghs_short");
});

test("ideaList lists a session's ideas and skips any without a title", () => {
    assert.equal(ideaList([{ title: "Rivers", body: "carry goods" }, { title: "" }, null, { title: "Ports" }]), "- **Rivers**: carry goods\n- **Ports**");
    assert.equal(ideaList(undefined), "");
    assert.equal(ideaList("rivers"), "");
});
