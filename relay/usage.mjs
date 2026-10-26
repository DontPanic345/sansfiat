// The Claude subscription's usage windows, for pacing.
//
//   node relay/usage.mjs    # print the 5-hour and weekly percentages
//
// Two routes, tried in order: the OAuth usage endpoint (no model call), then a one-word
// Haiku session whose stream-json carries a `rate_limit_event`.

import { execFile } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

// Either route's payload as { session, weekly, weeklyResetsAt, sessionResetsAt }: percentages
// and Dates (sessionResetsAt null when the payload doesn't say).
const when = (t) => (t ? new Date(t) : null);

export const parseUsage = (raw) => {
    const limits = raw.limits ?? [];
    const session = limits.find((l) => l.kind === "session");
    const weekly = limits.find((l) => l.kind === "weekly_all");
    if (session && weekly) {
        return { session: session.percent, weekly: weekly.percent, weeklyResetsAt: new Date(weekly.resets_at), sessionResetsAt: when(session.resets_at) };
    }
    if (raw.five_hour && raw.seven_day) {
        return {
            session: raw.five_hour.utilization,
            weekly: raw.seven_day.utilization,
            weeklyResetsAt: new Date(raw.seven_day.resets_at),
            sessionResetsAt: when(raw.five_hour.resets_at),
        };
    }
    const uw = raw.unifiedWindows;
    if (uw?.five_hour && uw?.seven_day) {
        return {
            session: uw.five_hour.utilization * 100,
            weekly: uw.seven_day.utilization * 100,
            weeklyResetsAt: new Date(uw.seven_day.resetsAt * 1000),
            sessionResetsAt: when(uw.five_hour.resetsAt && uw.five_hour.resetsAt * 1000),
        };
    }
    throw new Error("no session/weekly windows in the usage payload");
};

// CI signs in with a long-lived `claude setup-token` token; a desktop has a credentials file,
// under CLAUDE_CONFIG_DIR when a second profile is in use.
export const oauthToken = (env, credsText, now = Date.now()) => {
    if (env.CLAUDE_CODE_OAUTH_TOKEN) return env.CLAUDE_CODE_OAUTH_TOKEN;
    const oauth = JSON.parse(credsText ?? "{}").claudeAiOauth ?? {};
    if (!oauth.accessToken) throw new Error("no OAuth access token");
    if (oauth.expiresAt && oauth.expiresAt < now) throw new Error("the OAuth token has expired; run any `claude` command to refresh it");
    return oauth.accessToken;
};

// The `rate_limit_event` in a session's stream-json.
export const probeInfo = (stdout) => {
    for (const line of stdout.split("\n")) {
        try {
            const frame = JSON.parse(line);
            if (frame.type === "rate_limit_event") return frame.rate_limit_info;
        } catch {
            // non-JSON line
        }
    }
    throw new Error("no rate_limit_event in the stream-json");
};

const direct = async () => {
    const dir = process.env.CLAUDE_CONFIG_DIR || path.join(os.homedir(), ".claude");
    const creds = path.join(dir, ".credentials.json");
    const token = oauthToken(process.env, fs.existsSync(creds) ? fs.readFileSync(creds, "utf8") : null);
    const res = await fetch("https://api.anthropic.com/api/oauth/usage?at_wall=1&skip_spend=1", {
        headers: { Authorization: `Bearer ${token}`, "anthropic-beta": "oauth-2025-04-20" },
        signal: AbortSignal.timeout(10_000),
    });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    return res.json();
};

const probe = () =>
    new Promise((resolve, reject) =>
        execFile(
            "claude",
            ["-p", "ok", "--model", "haiku", "--output-format", "stream-json", "--verbose"],
            { timeout: 180_000, maxBuffer: 10_000_000 },
            (err, stdout) => {
                try {
                    resolve(probeInfo(stdout ?? ""));
                } catch (e) {
                    reject(err ?? e);
                }
            },
        ),
    );

// { route, usage }, from the first route whose payload has the windows.
export const readUsage = async (routes = { direct, probe }) => {
    const failed = [];
    for (const [route, read] of Object.entries(routes)) {
        try {
            return { route, usage: parseUsage(await read()) };
        } catch (e) {
            failed.push(`${route}: ${e.message}`);
        }
    }
    throw new Error(`couldn't read usage (${failed.join("; ")})`);
};

if (process.argv[1] === fileURLToPath(import.meta.url)) {
    readUsage()
        .then(({ route, usage }) =>
            console.log(`${usage.session}% of the 5-hour window, ${usage.weekly}% of the week (resets ${usage.weeklyResetsAt.toISOString()}), from the ${route} route`),
        )
        .catch((e) => {
            console.error(e.message);
            process.exit(1);
        });
}
