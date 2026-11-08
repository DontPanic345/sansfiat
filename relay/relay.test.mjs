import assert from "node:assert/strict";
import { test } from "node:test";
import {
    afterOf,
    applyModel,
    applyResult,
    atLevel,
    checkSteps,
    checksGreen,
    designPaths,
    dollars,
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
    specReviewChoice,
    statusLine,
    stepBody,
    stepComment,
    stepsTable,
    usagePoints,
    waitingOn,
    playChoice,
    stepRank,
    stepKind,
    waitOver,
    RETRO_STEPS,
    RETRO_WAIT_MS,
    BOT,
    sortFindings,
    findingText,
    withTable,
    isBuilt,
    parseProposal,
    withNumbers,
    fixesWaitsFor,
    retroBody,
    milestoneTag,
    promptPath,
    withStatus,
    writeMeta,
    pairBranches,
    pairSteps,
    pickOutcome,
    planSteps,
    sideOf,
} from "./relay.mjs";

const resets = new Date("2026-10-17T09:00:00Z");
const day = (n) => new Date(resets.getTime() - 7 * 86400_000 + (n - 1) * 86400_000 + 3600_000);

test("pace starts a step only while the 5-hour window is under 75%", () => {
    assert.equal(pace({ session: 74.6, weekly: 0, weeklyResetsAt: resets }, day(1)).ok, true);
    assert.equal(pace({ session: 75, weekly: 0, weeklyResetsAt: resets }, day(1)).ok, false);
    assert.equal(pace({ session: 33.333, weekly: 4.0001, weeklyResetsAt: resets }, day(1)).reason, "5-hour 33%, weekly 4% < 13.6% (day 1)");
});

test("pace spreads the weekly budget across the cycle", () => {
    assert.equal(pace({ session: 0, weekly: 12, weeklyResetsAt: resets }, day(1)).ok, true);
    assert.equal(pace({ session: 0, weekly: 14, weeklyResetsAt: resets }, day(1)).ok, false);
    assert.equal(pace({ session: 0, weekly: 94, weeklyResetsAt: resets }, day(7)).ok, true);
    assert.equal(pace({ session: 0, weekly: 95, weeklyResetsAt: resets }, day(7)).ok, false);
});

test("a stop says when it lifts", () => {
    const at = new Date("2026-10-11T02:20:00Z");
    const five = pace({ session: 80, weekly: 0, weeklyResetsAt: resets, sessionResetsAt: at }, day(1));
    assert.equal(waitingOn(five), "5-hour usage, until 15:20");
    assert.match(five.reason, /resets 2026-10-11T02:20:00\.000Z/);
    // 14% on day 1 lifts on day 2, when the ramp reaches 27.1%.
    const weekly = pace({ session: 0, weekly: 14, weeklyResetsAt: resets }, day(1));
    assert.equal(weekly.until.getTime(), resets.getTime() - 6 * 86400_000);
    assert.equal(waitingOn(weekly), "weekly usage, until Sun 22:00");
    assert.equal(pace({ session: 0, weekly: 96, weeklyResetsAt: resets }, day(7)).until, resets);
    assert.equal(waitingOn(pace({ session: 0, weekly: 0, weeklyResetsAt: resets }, day(1))), "");
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

test("a piece's body says the spec's model and its design files", () => {
    const body = "**Goal:** rivers.\n\n- **Spec**: Opus high, because rivers touch every mechanic.\n\nBuilds docs/design/map.md and docs/design/map.md.";
    assert.equal(pieceField(body, "Goal"), "rivers.");
    assert.deepEqual(specChoice(body), { model: "opus", effort: "high", why: "Opus high, because rivers touch every mechanic." });
    assert.equal(specChoice("**Spec:** opus"), null);
    assert.deepEqual(designPaths(body), ["docs/design/map.md"]);
});

test("a hard-wrapped field reads to the next field, and spec-review has its own", () => {
    const body = "**Spec:** sonnet high, 9 points (Open 2,\nNew 1): the store meets the clans.\n\n**Spec-review:** haiku high: it reads cold.\n**Design:** docs/design/founding.md\n- **Decided:** x";
    assert.equal(pieceField(body, "Spec"), "sonnet high, 9 points (Open 2, New 1): the store meets the clans.");
    assert.deepEqual(specReviewChoice(body), { model: "haiku", effort: "high", why: "haiku high: it reads cold." });
    assert.equal(pieceField("**Spec-review:** sonnet low\n**Design:** a", "Spec-review"), "sonnet low");
    assert.equal(specReviewChoice("**Spec:** opus low"), null);
});

test("checkSteps wants builds in order, then the review, each with points and a why", () => {
    const s = (id, extra = {}) => ({ id, title: "t", model: "sonnet", effort: "medium", points: 7, why: "w", ...(id === "review" ? {} : { part: "ford" }), ...extra });
    const ok = (...ids) => JSON.stringify(ids.map((id) => (typeof id === "string" ? s(id) : id)));
    assert.ok(checkSteps(ok("build-1", "review")).steps);
    assert.ok(checkSteps(ok(s("build-1", { model: "opus", effort: "low", part: "clay" }), "review")).steps);
    assert.match(checkSteps(ok(s("build-1", { part: "freestyle" }), "review")).error, /part must be/);
    assert.match(checkSteps("nope").error, /JSON/);
    assert.match(checkSteps(ok("review")).error, /build steps/);
    assert.match(checkSteps(ok("spec-review", "build-1", "review")).error, /build-1/);
    assert.match(checkSteps(ok("build-1", "build-3", "review")).error, /build-2/);
    assert.match(checkSteps(ok("build-1", "build-2")).error, /review/);
    assert.match(checkSteps(ok(s("build-1", { model: "gpt" }), "review")).error, /model/);
    assert.match(checkSteps(ok(s("build-1", { effort: "max" }), "review")).error, /effort/);
    assert.match(checkSteps(ok(s("build-1", { points: 13 }), "review")).error, /points/);
    assert.match(checkSteps(ok(s("build-1", { points: undefined }), "review")).error, /points/);
    assert.ok(checkSteps(ok("build-1", s("review", { points: undefined, model: "sonnet", effort: "high" }))).steps);
    assert.match(checkSteps(ok("build-1", s("review", { effort: "low" }))).error, /low effort/);
    assert.ok(checkSteps(ok(s("build-1", { part: "glaze", model: "opus", effort: "low" }), "review")).steps);
    assert.match(checkSteps(ok(s("build-1", { why: "" }), "review")).error, /no why/);
    const twin = { model: "sonnet", effort: "high", why: "w" };
    assert.ok(checkSteps(ok(s("build-1", { model: "opus", effort: "low", twin }), "review")).steps);
    assert.match(checkSteps(ok(s("build-1", { twin: { ...twin, model: "gpt" } }), "review")).error, /twin needs/);
    assert.match(checkSteps(ok(s("build-1", { effort: "high", twin }), "review")).error, /own rung/);
    assert.match(checkSteps(ok(s("build-1", { model: "opus", twin: { ...twin, why: "" } }), "review")).error, /why for its twin/);
    assert.match(checkSteps(ok(s("build-1", { model: "opus", twin }), s("build-2", { model: "opus", twin }), "review")).error, /only one/);
});

const step = (id, state = "open", meta = {}) => ({ number: 100, state, meta: { id, model: "sonnet", effort: "medium", ...meta } });
const piece = (number, labels, steps = [], extra = {}) => ({
    number,
    title: `Piece ${number}`,
    body: "**Goal:** rivers.\n**Spec:** sonnet high, because it's one module.\n**Spec-review:** sonnet medium, to read it cold.\n**Design:** docs/design/map.md\n\n**In scope**\n\n- rivers",
    labels: ["kind:piece", "relay:ready", ...labels],
    author: "DontPanic345",
    milestone: null,
    steps,
    ...extra,
});

test("pieceAction starts, plans after the spec and after spec-review, then runs each open step in order", () => {
    assert.deepEqual(pieceAction(piece(1, [])), { action: "start" });
    assert.equal(pieceAction(piece(1, [], [step("spec")])).step.meta.id, "spec");
    assert.deepEqual(pieceAction(piece(1, [], [step("spec", "closed")])), { action: "plan" });
    assert.equal(pieceAction(piece(1, [], [step("spec", "closed"), step("spec-review")])).step.meta.id, "spec-review");
    assert.deepEqual(pieceAction(piece(1, [], [step("spec", "closed"), step("spec-review", "closed")])), { action: "plan" });
    const planned = piece(1, [], [step("spec", "closed"), step("review"), step("build-1"), step("spec-review", "closed")]);
    assert.equal(pieceAction(planned).step.meta.id, "build-1");
    const building = piece(1, [], [step("spec", "closed"), step("spec-review", "closed"), step("review"), step("build-2"), step("build-1", "closed")]);
    assert.equal(pieceAction(building).step.meta.id, "build-2");
    const blocked = piece(1, [], [step("spec", "closed"), step("spec-review", "closed"), step("build-1", "open", { status: "blocked" })]);
    assert.equal(pieceAction(blocked), null);
    const finished = piece(1, [], [step("spec", "closed"), step("spec-review", "closed"), step("build-1", "closed"), step("review", "closed")]);
    assert.equal(pieceAction(finished), null);
});

test("the Play line names the play step's model, its effort and its seeds", () => {
    assert.deepEqual(playChoice("**Play:** sonnet"), { model: "sonnet", effort: "medium", seeds: 1, why: "sonnet" });
    assert.equal(playChoice("**Play:** sonnet, three seeds").seeds, 3);
    assert.equal(playChoice("**Play:** opus high, 2 seeds").effort, "high");
    assert.equal(playChoice("**Play:** none"), null);
    assert.equal(playChoice("**Goal:** x"), null);
});

test("a piece's play step runs after its builds and before its review; a retro's steps in order", () => {
    const ids = ["review", "play", "build-2", "spec", "build-1", "spec-review"];
    assert.deepEqual([...ids].sort((a, b) => stepRank(a) - stepRank(b)), ["spec", "spec-review", "build-1", "build-2", "play", "review"]);
    const retro = RETRO_STEPS.map((s) => s.id);
    assert.deepEqual([...retro].reverse().sort((a, b) => stepRank(a) - stepRank(b)), retro);
    assert.deepEqual(retro, ["check", "ladder", "architecture", "relay", "wait", "game", "retro", "proposal"]);
    assert.deepEqual(["play", "wait", "ladder", "build-3"].map(stepKind), ["play", "wait", "retro", "build"]);
    assert.equal(promptPath("players/player"), "relay/players/player.md");
    assert.equal(promptPath("play"), "relay/prompts/play.md");
});

test("a retro waits for the owner's notes, `go`, or 72 hours after the step before", () => {
    const closed = new Date("2026-11-01T00:00:00Z");
    const steps = [step("relay", "closed", {}), step("wait")];
    steps[0].closedAt = closed.toISOString();
    const at = (ms) => new Date(closed.getTime() + ms);
    assert.equal(waitOver(steps, steps[1], at(RETRO_WAIT_MS - 1)), false);
    assert.equal(waitOver(steps, steps[1], at(RETRO_WAIT_MS)), true);
    assert.equal(waitOver(steps, step("wait", "open", { notes: "the scout starves" }), at(0)), true);
    assert.equal(waitOver(steps, step("wait", "open", { go: true }), at(0)), true);
    const retro = { ...piece(5, ["kind:retro"]), steps };
    assert.equal(pieceAction(retro, at(0)), null);
    assert.deepEqual(pieceAction(retro, at(RETRO_WAIT_MS)).action, "waited");
});

test("a retro or a fixes piece the crew opened can run; a retro needs no Spec line", () => {
    const retro = piece(5, ["kind:retro"], [], { author: BOT, body: "**Goal:** M4 read back." });
    assert.equal(holdReason(retro), null);
    assert.deepEqual(pieceAction(retro), { action: "start" });
    const m4 = { number: 7, title: "M4" };
    const fixes = piece(8, ["relay:fixes"], [], { author: BOT, milestone: m4 });
    const others = [piece(3, [], [], { milestone: m4 }), piece(4, ["relay:paused"], [], { milestone: m4 }), piece(6, [], [], { milestone: { number: 2 } }), fixes];
    assert.deepEqual(fixesWaitsFor(others, fixes), [3]);
    assert.match(holdReason({ ...fixes, before: [3] }), /other pieces \(#3\)/);
    assert.equal(holdReason({ ...fixes, before: [] }), null);
});

test("findings sort into fix and ask", () => {
    const result = {
        findings: [
            { title: "Disband all sends one", class: "fix", repro: "play/disband.lines" },
            { title: "No repro", class: "fix" },
            { title: "Less fun", class: "ask" },
        ],
        ideas: [{ title: "Herds" }, { body: "no title" }],
    };
    const m = { number: 7, title: "M4" };
    const { fix, ask } = sortFindings(result, { milestone: m });
    assert.deepEqual(fix.map((i) => i.title), ["Disband all sends one"]);
    assert.deepEqual(ask.map((i) => i.title), ["No repro", "Less fun", "Herds"]);
    assert.equal(sortFindings(result, { milestone: m, retro: true }).fix.length, 0);
    assert.equal(sortFindings(result).fix.length, 0);
    assert.equal(findingText(fix[0], "#9 play"), "**Disband all sends one** Repro: play/disband.lines (#9 play)");
});

test("the crew rewrites the PR's steps table, or puts it under Closes", () => {
    const table = "| Step | State |\n|---|---|\n| #1 review | done |\n\n$1.00 at list price so far.";
    const old = "Closes #9.\n\n| Step | State |\n|---|---|\n| #1 review | pending |\n\n$0.50 at list price so far.\n\n## Verdict\n\nmerge";
    assert.equal(withTable(old, table), `Closes #9.\n\n${table}\n\n## Verdict\n\nmerge`);
    assert.equal(withTable("Closes #9.\n\n## Verdict", table), `Closes #9.\n\n${table}\n\n\n## Verdict`);
});

test("a milestone is built when every open piece is paused", () => {
    assert.equal(isBuilt([]), true);
    assert.equal(isBuilt([{ labels: ["relay:paused"] }]), true);
    assert.equal(isBuilt([{ labels: ["relay:paused"] }, { labels: ["relay:ready"] }]), false);
    assert.equal(milestoneTag({ title: "M4: Settling" }), "m4");
    assert.equal(milestoneTag({ title: "Later" }), null);
    assert.match(retroBody({ title: "M4" }), /docs\/proposals\/m5\.md/);
});

test("a proposal's pieces are its lettered sections with a Goal, After keys made numbers", () => {
    const md = [
        "# M4", "", "## 1. Foundation", "", "### S. The game proves itself", "", "**Goal:** a stage.", "", "**Spec:** sonnet high", "",
        "### E. A game that ends", "", "**Goal:** an end.", "", "**After:** #S", "",
        "## 3. Order and cost", "", "### Not a piece", "", "text",
    ].join("\n");
    const pieces = parseProposal(md);
    assert.deepEqual(pieces.map((p) => [p.key, p.title]), [["S", "The game proves itself"], ["E", "A game that ends"]]);
    assert.equal(pieceField(pieces[0].body, "Spec"), "sonnet high");
    assert.equal(withNumbers(pieces[1].body, { S: 150 }), "**Goal:** an end.\n\n**After:** #150");
});

test("holdReason needs a ready piece from the owner with the spec's model", () => {
    assert.equal(holdReason(piece(9, [])), null);
    assert.equal(holdReason(piece(9, ["waiting"])), "waiting on the owner");
    assert.equal(holdReason(piece(9, ["relay:paused"])), "paused");
    assert.equal(holdReason({ ...piece(9, []), labels: ["kind:piece"] }), "not ready");
    assert.equal(holdReason({ ...piece(9, []), author: "someone" }), "not opened by the owner");
    assert.match(holdReason(piece(9, [], [], { body: "**Goal:** rivers" })), /Spec/);
    assert.match(holdReason(piece(9, [], [], { body: "**Spec:** opus low" })), /Spec-review/);
    // Once the spec step exists, its model is on the step, not the body.
    assert.equal(holdReason(piece(9, [], [step("spec")], { body: "**Goal:** rivers" })), null);
});

test("a piece waits for the pieces its After line names to be merged", () => {
    assert.deepEqual(afterOf("**Goal:** units.\n**After:** #31 and #32, for the map"), [31, 32]);
    assert.deepEqual(afterOf("**Goal:** units."), []);
    const after = [{ number: 31, merged: true }, { number: 32, merged: false }];
    assert.equal(holdReason(piece(9, [], [], { after })), "after #32, not merged yet");
    assert.equal(holdReason(piece(9, [], [], { after: [after[0]] })), null);
});

test("CI is green when every check finished and none failed", () => {
    const run = (status, conclusion) => ({ status, conclusion });
    assert.equal(checksGreen([run("completed", "success"), run("completed", "skipped")]), true);
    assert.equal(checksGreen([run("completed", "success"), run("in_progress", null)]), false);
    assert.equal(checksGreen([run("completed", "failure")]), false);
    assert.equal(checksGreen([]), false);
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

test("sessionStats measures the last turn's context", () => {
    const last = { message: { usage: { input_tokens: 10, cache_creation_input_tokens: 990, cache_read_input_tokens: 49_000, output_tokens: 0 } } };
    const result = {
        total_cost_usd: 1.25,
        modelUsage: {
            "claude-sonnet-x": { inputTokens: 100, outputTokens: 200, cacheReadInputTokens: 300, cacheCreationInputTokens: 400, contextWindow: 200_000 },
            "claude-haiku-x": { inputTokens: 5, outputTokens: 5, contextWindow: 200_000 },
        },
    };
    assert.deepEqual(sessionStats(last, result, "sonnet", 1), { cost: 1.25, context: 25, compactions: 1 });
    assert.deepEqual(sessionStats(null, null, "sonnet"), { cost: 0, context: 0, compactions: 0 });
    assert.equal(dollars(2.143), "$2.14");
});

test("usagePoints is each window's rise, and what was used since a reset", () => {
    assert.deepEqual(usagePoints({ session: 20, weekly: 9 }, { session: 38.4, weekly: 11 }), { fiveHour: 18, weekly: 2 });
    assert.deepEqual(usagePoints({ session: 45, weekly: 9 }, { session: 3, weekly: 10 }), { fiveHour: 3, weekly: 1 });
    assert.deepEqual(usagePoints(null, { session: 3, weekly: 10 }), { fiveHour: null, weekly: null });
});

test("a step's body is its status, its choice, its links and its brief as made", () => {
    const s = { id: "build-1", model: "opus", effort: "low", points: 10, part: "canvas", why: "Rivers are open." };
    const body = stepBody(piece(4, []), s, { brief: "# build-1: Rivers\n\nBuild rivers.", pr: 7 });
    assert.match(body, /^\*\*Status:\*\* ready\n\nStep `build-1` on \*\*opus low\*\*, 10 points, canvas\. Rivers are open\./);
    assert.match(body, /Piece #4 · PR #7 · \[spec\.md\]\(https:\/\/github\.com\/DontPanic345\/sansfiat\/blob\/relay\/4\/relay\/specs\/4\/spec\.md\)/);
    assert.match(body, /^## build-1: Rivers/m);
    assert.doesNotMatch(stepBody(piece(4, []), { id: "review", model: "sonnet", effort: "high", why: "w" }), /brief/);
    const done = withStatus(body, statusLine({ cost: 1.3, minutes: 12.6, attempts: 0 }, "done"));
    assert.match(done, /^\*\*Status:\*\* done, \$1\.30, 13 minutes\n/);
    assert.equal(done.match(/\*\*Status:\*\*/g).length, 1);
    assert.equal(statusLine({ cost: 0.5, minutes: 3, attempts: 1 }, "pending", "half way"), "not finished (attempt 1 of 2), $0.50, 3 minutes: half way");
    assert.equal(withStatus("old body", "skipped"), "**Status:** skipped\n\nold body");
});

test("a step's PR comment carries what it took", () => {
    const s = { number: 50, meta: { id: "build-2", model: "sonnet", effort: "high", points: 9, cost: 1.3, minutes: 12.4, decisions: [{ decision: "Rivers flow east", why: "" }] } };
    assert.equal(
        stepComment(s, { summary: "Rivers.", ideas: [{ title: "Ports" }] }),
        "**build-2** (sonnet high, 9 points, $1.30, 12 min, #50): Rivers.\n\n🧭 Decided:\n- Rivers flow east\n\n💡 Ideas, not built:\n- **Ports**",
    );
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
    const p = piece(4, [], [
        step("spec", "closed", { summary: "Wrote the spec.", decisions: [{ decision: "Rivers flow south", why: "" }] }),
        step("build-1", "closed", { summary: "Built rivers.", decisions: [{ decision: "Rivers flow east", why: "the map tilts" }] }),
    ]);
    const text = sessionPrompt(p, step("build-1", "open", { reply: "Use 0.5", note: "tests half done" }), files());
    assert.match(text, /^# build-1: Rivers/m);
    assert.doesNotMatch(text, /Rivers run downhill/);
    assert.match(text, /Use 0\.5/);
    assert.match(text, /commits are on the branch/);
    assert.match(text, /tests half done/);
    assert.match(text, /\*\*spec\*\*: Wrote the spec\.\n- \*\*build-1\*\*/);
    assert.doesNotMatch(text, /Rivers flow south/);
    assert.match(text, /\*\*build-1\*\*: Built rivers\.\n  - Decided: Rivers flow east \(the map tilts\)/);
    assert.throws(() => sessionPrompt(p, step("build-2"), files({ brief: null })), /build-2\.md is missing/);
});

test("a review session gets the piece, the diff, the whole spec and the steps, with no brief", () => {
    const text = sessionPrompt(piece(4, [], [step("build-1", "closed", { cost: 1 })]), step("review"), files({ brief: null }));
    assert.match(text, /Review this piece as built/);
    assert.match(text, /- rivers/);
    assert.match(text, /world\.rs \| 10/);
    assert.match(text, /# The whole spec/);
    assert.match(text, /# The steps\n\nPut this table .*\n\n\| Step \|/);
});

test("a play session gets the piece, its seeds, the player and the judge, and no design", () => {
    const p = piece(4, [], [], { body: "**Goal:** rivers.\n**Play:** sonnet, three seeds" });
    const prompts = { ...files().prompts, play: "# Play step", "players/player": "# The player", "players/judge": "# The judge" };
    const text = sessionPrompt(p, step("play"), files({ prompts, started: "2026-11-07T01:00:00Z", cap: 90 }));
    assert.match(text, /Play this piece's branch and write its play note, on 3 seeds/);
    assert.match(text, /# Play step[\s\S]*# The player[\s\S]*# The judge/);
    assert.doesNotMatch(text, /Rivers run downhill/);
    assert.match(text, /started at 2026-11-07T01:00:00Z, and the crew stops it after 90 minutes/);
});

test("a retro session works in the milestone's retro folder, and the game step gets the owner's notes", () => {
    const retro = piece(5, ["kind:retro"], [], { body: "**Goal:** M4 read back.", milestone: { number: 7, title: "M4" } });
    const prompts = { ...files().prompts, retro: "# The retro" };
    const ladder = sessionPrompt(retro, step("ladder"), files({ prompts }));
    assert.match(ladder, /retro #5/);
    assert.match(ladder, /relay\/retro\/m4\//);
    assert.match(ladder, /The ladder angle, as `retro.md` says for the `ladder` step/);
    assert.doesNotMatch(ladder, /owner's notes/);
    const game = sessionPrompt(retro, step("game"), files({ prompts, notes: "The scout starves." }));
    assert.match(game, /relay\/retro\/m4\/owner\.md[\s\S]*The scout starves\./);
    assert.match(sessionPrompt(retro, step("game"), files({ prompts, notes: null })), /None came in 72 hours/);
});

test("stepsTable shows each step's state and spend", () => {
    const table = stepsTable([
        step("review"),
        { ...step("build-1", "closed", { model: "opus", effort: "low", part: "canvas", points: 10, cost: 1.5, minutes: 12.6, fiveHour: 8, weekly: 1, attempts: 0, sessions: 1 }), stateReason: "completed" },
        { ...step("spec", "closed", { cost: 2 }), stateReason: "completed" },
    ]);
    const rows = table.split("\n");
    assert.equal(rows[3], "| #100 build-1 | done | canvas | opus | low | 10 | 1 | $1.50 | 13 | 8 | 1 |");
    assert.match(rows[4], /\| #100 review \| pending \|/);
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

const twinned = [
    { id: "build-1", title: "Easel", part: "easel", model: "haiku", effort: "high", points: 5, why: "w1" },
    { id: "build-2", title: "Canvas", part: "canvas", model: "opus", effort: "low", points: 10, why: "w2", twin: { model: "sonnet", effort: "high", why: "does Sonnet paint it?" } },
    { id: "review", title: "Review the piece", model: "sonnet", effort: "high", why: "w3" },
];

test("the crew twins one step from steps.json and adds its pick, then play before the review", () => {
    const p = piece(9, []);
    const made = planSteps(p, twinned, { model: "sonnet", effort: "medium", why: "sonnet" }, () => 0.9);
    assert.deepEqual(made.map((s) => s.id), ["build-1", "build-2", "build-2-twin", "build-2-pick", "play", "review"]);
    const [, ladder, twin, pick] = made;
    assert.deepEqual(pairBranches(p, "build-2"), ["relay/9-build-2-a", "relay/9-build-2-b"]);
    assert.equal(ladder.branch, "relay/9-build-2-b");
    assert.equal(ladder.twin, undefined);
    assert.deepEqual([twin.model, twin.effort, twin.branch, twin.of, twin.part, twin.points], ["sonnet", "high", "relay/9-build-2-a", "build-2", "canvas", 10]);
    assert.match(twin.why, /does Sonnet paint it/);
    assert.deepEqual([pick.model, pick.effort, pick.of, pick.branch], ["opus", "medium", "build-2", undefined]);
    assert.equal(planSteps(p, twinned, null, () => 0.1)[1].branch, "relay/9-build-2-a");
    assert.deepEqual(made.map((s) => s.id).sort((a, b) => stepRank(a) - stepRank(b)), made.map((s) => s.id));
    assert.deepEqual(["build-2-twin", "build-2-pick"].map(stepKind), ["build", "pick"]);
});

test("the pick's choice reads back as which run won and how much of each landed", () => {
    const steps = [step("build-2", "closed", { branch: "relay/9-build-2-b" }), step("build-2-twin", "closed", { branch: "relay/9-build-2-a", of: "build-2" }), step("build-2-pick")];
    const pair = pairSteps(steps, "build-2-pick");
    assert.equal(pair.ladder.meta.id, "build-2");
    assert.equal(pairSteps(steps, "build-2").twin.meta.id, "build-2-twin");
    assert.equal(pairSteps(steps, "build-1"), null);
    assert.equal(sideOf("relay/9-build-2-a"), "A");
    assert.deepEqual(pickOutcome(pair, "A"), { chose: "twin", ladder: "none", twin: "all" });
    assert.deepEqual(pickOutcome(pair, "B"), { chose: "ladder", ladder: "all", twin: "none" });
    assert.deepEqual(pickOutcome(pair, "both"), { chose: "both", ladder: "part", twin: "part" });
});

test("a twin is told it's the step it twins, on its own branch, and sees neither run's summary", () => {
    const p = piece(9, [], [
        step("build-1", "closed", { summary: "Built the easel." }),
        step("build-2", "closed", { summary: "Painted on Opus.", branch: "relay/9-build-2-b" }),
    ]);
    const twin = step("build-2-twin", "open", { of: "build-2", branch: "relay/9-build-2-a", reply: "Use 0.5" });
    const text = sessionPrompt(p, twin, files({ brief: "# build-2: Canvas\n\nPaint." }));
    assert.match(text, /on its branch, `relay\/9-build-2-a`/);
    assert.match(text, /# build-2: Canvas/);
    assert.match(text, /this step, `build-2`, only/);
    assert.doesNotMatch(text, /twin/);
    assert.match(text, /Built the easel/);
    assert.doesNotMatch(text, /Painted on Opus/);
});

test("a pick session gets the brief and both versions as A and B, and no design", () => {
    const p = piece(9, [], [
        step("build-1", "closed", { summary: "Built the easel." }),
        step("build-2", "closed", { model: "opus", summary: "Painted it one way.", branch: "relay/9-build-2-b", decisions: [{ decision: "Rivers flow east", why: "" }] }),
        step("build-2-twin", "closed", { summary: "Painted it another.", branch: "relay/9-build-2-a", of: "build-2", ideas: [{ title: "Ports" }] }),
    ]);
    const prompts = { ...files().prompts, pick: "# Pick step" };
    const text = sessionPrompt(p, step("build-2-pick", "open", { of: "build-2" }), files({ prompts, brief: "# build-2: Canvas\n\nPaint.", sides: { A: " a.rs | 3 +", B: " b.rs | 9 +" } }));
    assert.match(text, /version A on `relay\/9-build-2-a` and version B on `relay\/9-build-2-b`/);
    assert.match(text, /land the result on `relay\/9`/);
    assert.match(text, /# The brief both versions built\n\n## build-2: Canvas/);
    assert.match(text, /# Version A\n\nPainted it another\.\n\nIdeas, not built:\n- \*\*Ports\*\*\n\n```\na\.rs \| 3 \+/);
    assert.match(text, /# Version B\n\nPainted it one way\.\n- Decided: Rivers flow east\n\n```\nb\.rs \| 9 \+/);
    assert.ok(text.indexOf("# Version A") < text.indexOf("# Version B"));
    assert.doesNotMatch(text, /opus|twin|Rivers run downhill/);
    assert.match(text, /Built the easel/);
    assert.match(text, /# Pick step/);
});

test("the steps table and the PR comments say what each run landed and what the pick chose", () => {
    const table = stepsTable([
        { ...step("build-2", "closed", { branch: "relay/9-build-2-b", picked: "none", cost: 2 }), stateReason: "completed" },
        { ...step("build-2-twin", "closed", { branch: "relay/9-build-2-a", picked: "all", cost: 1 }), stateReason: "completed" },
        { ...step("build-2-pick", "closed", { chose: "twin", cost: 1 }), stateReason: "completed" },
    ]);
    assert.match(table, /\| #100 build-2 \| done, none landed \|/);
    assert.match(table, /\| #100 build-2-twin \| done, all landed \|/);
    assert.match(table, /\| #100 build-2-pick \| done, chose twin \|/);
    assert.match(table, /\$4\.00 at list price so far\./);
    const twin = step("build-2-twin", "closed", { branch: "relay/9-build-2-a", picked: "all", cost: 1, summary: "Painted." });
    assert.match(stepComment(twin, twin.meta), /\(sonnet medium, version A, all landed, \$1\.00/);
});
