// A Claude Code PostToolUse hook: tells a Relay session its usage windows as context after a
// tool call, every USAGE_EVERY_MIN minutes and as the 5-hour window passes each of MARKS. The
// crew sets it on every session and keeps the reading in RELAY_USAGE_FILE from the session's
// own stream (relay.mjs, runSession). It never fails the tool call: any trouble is silence.

import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { readUsage } from "./usage.mjs";

export const USAGE_EVERY_MIN = 10;
// The 5-hour window's percentages the session is told at: commit, then stop.
export const MARKS = [70, 85];

// Whether the clock says to speak: `last` is when the hook last did (ms), or null on the
// session's first tool call, which only starts the clock since the prompt gave the reading.
export const due = (last, now, every = USAGE_EVERY_MIN) => last != null && now - last >= every * 60_000;

// Whether the window passed a mark between the reading the hook last gave and this one.
export const crossed = (was, is) => MARKS.some((m) => was < m && is >= m);

// What the session reads.
export const say = (u, now = new Date()) => {
    const left = u.sessionResetsAt ? `, which resets in ${Math.max(0, Math.round((u.sessionResetsAt - now) / 60_000))} minutes` : "";
    const head = `Usage: ${Math.round(u.session)}% of the 5-hour window${left}; ${Math.round(u.weekly)}% of the week.`;
    if (u.session >= MARKS[1]) {
        return `${head} Stop here, before the window ends the session: commit and push, write result.json (\`pending\` with a note on where the next attempt picks up, if the step isn't done), and end. A helper reading this reports back to its session now.`;
    }
    if (u.session >= MARKS[0]) return `${head} Commit and push what you have, and start no helper or long run that the rest of the window can't finish.`;
    return head;
};

// The reading the crew last took from the session's stream, or null.
export const fromFile = (file) => {
    if (!file || !fs.existsSync(file)) return null;
    const u = JSON.parse(fs.readFileSync(file, "utf8"));
    return { ...u, sessionResetsAt: u.sessionResetsAt ? new Date(u.sessionResetsAt) : null };
};

const main = async () => {
    let input = "";
    for await (const chunk of process.stdin) input += chunk;
    const session = String(JSON.parse(input || "{}").session_id ?? "session").replace(/[^\w-]/g, "");
    // The stamp's time is when the hook last spoke, and its text the 5-hour reading it gave.
    const stamp = path.join(os.tmpdir(), `relay-usage-${session}`);
    const last = fs.existsSync(stamp) ? fs.statSync(stamp).mtimeMs : null;
    if (last == null) {
        fs.writeFileSync(stamp, "0");
        return;
    }
    const was = Number(fs.readFileSync(stamp, "utf8")) || 0;
    const clock = due(last, Date.now());
    let usage = fromFile(process.env.RELAY_USAGE_FILE);
    if (!clock && !(usage && crossed(was, usage.session))) return;
    usage ??= (await readUsage()).usage;
    fs.writeFileSync(stamp, String(usage.session));
    console.log(JSON.stringify({ hookSpecificOutput: { hookEventName: "PostToolUse", additionalContext: say(usage) } }));
};

if (process.argv[1] === fileURLToPath(import.meta.url)) main().catch(() => {});
