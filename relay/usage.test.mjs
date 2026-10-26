import assert from "node:assert/strict";
import { test } from "node:test";
import { oauthToken, parseUsage, probeInfo, readUsage } from "./usage.mjs";

test("parseUsage reads the direct route's rows and the probe's fractions", () => {
    const direct = parseUsage({
        limits: [
            { kind: "session", percent: 2, resets_at: "2026-10-12T08:09:59Z" },
            { kind: "weekly_all", percent: 9, resets_at: "2026-10-17T08:59:59Z" },
        ],
    });
    assert.deepEqual([direct.session, direct.weekly], [2, 9]);
    assert.equal(direct.sessionResetsAt.toISOString(), "2026-10-12T08:09:59.000Z");
    const probe = parseUsage({
        unifiedWindows: { five_hour: { utilization: 0.3, resetsAt: 1 }, seven_day: { utilization: 0.5, resetsAt: 2 } },
    });
    assert.deepEqual([probe.session, probe.weekly], [30, 50]);
    assert.throws(() => parseUsage({ limits: [] }), /no session\/weekly/);
});

test("oauthToken prefers the environment, then a credentials file that hasn't expired", () => {
    assert.equal(oauthToken({ CLAUDE_CODE_OAUTH_TOKEN: "env" }, null), "env");
    const creds = (expiresAt) => JSON.stringify({ claudeAiOauth: { accessToken: "file", expiresAt } });
    assert.equal(oauthToken({}, creds(2000), 1000), "file");
    assert.throws(() => oauthToken({}, creds(500), 1000), /expired/);
    assert.throws(() => oauthToken({}, null), /no OAuth/);
});

test("probeInfo finds the rate-limit frame among other lines", () => {
    const out = ['{"type":"system"}', "not json", '{"type":"rate_limit_event","rate_limit_info":{"a":1}}'].join("\n");
    assert.deepEqual(probeInfo(out), { a: 1 });
    assert.throws(() => probeInfo('{"type":"system"}'), /no rate_limit_event/);
});

test("readUsage falls back to the probe and says which route answered", async () => {
    const windows = { unifiedWindows: { five_hour: { utilization: 0.1, resetsAt: 1 }, seven_day: { utilization: 0.2, resetsAt: 2 } } };
    const viaProbe = await readUsage({ direct: async () => ({ limits: [] }), probe: async () => windows });
    assert.equal(viaProbe.route, "probe");
    assert.equal(viaProbe.usage.session, 10);
    const viaDirect = await readUsage({ direct: async () => windows, probe: async () => assert.fail("probe ran") });
    assert.equal(viaDirect.route, "direct");
    await assert.rejects(
        readUsage({ direct: async () => { throw new Error("HTTP 401"); }, probe: async () => { throw new Error("no frame"); } }),
        /direct: HTTP 401; probe: no frame/,
    );
});
