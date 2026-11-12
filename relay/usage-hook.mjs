// A Claude Code PostToolUse hook: tells a Relay session its usage windows every
// USAGE_EVERY_MIN minutes, as context after a tool call. The crew sets it on every session
// (relay.mjs, runSession). It never fails the tool call: any trouble is silence.

import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { readUsage } from "./usage.mjs";

export const USAGE_EVERY_MIN = 10;

// Whether the hook speaks now: `last` is when it last did (ms), or null on the session's first
// tool call, which only starts the clock since the prompt gave the reading.
export const due = (last, now, every = USAGE_EVERY_MIN) => last != null && now - last >= every * 60_000;

// What the session reads.
export const say = (u, now = new Date()) => {
    const left = u.sessionResetsAt ? `, which resets in ${Math.max(0, Math.round((u.sessionResetsAt - now) / 60_000))} minutes` : "";
    const head = `Usage: ${Math.round(u.session)}% of the 5-hour window${left}; ${Math.round(u.weekly)}% of the week.`;
    if (u.session >= 90) return `${head} The window is nearly spent: commit and push now, and write result.json as \`pending\` with a note if the step isn't done.`;
    if (u.session >= 70) return `${head} Commit and push what you have before the next long run.`;
    return head;
};

const main = async () => {
    let input = "";
    for await (const chunk of process.stdin) input += chunk;
    const session = String(JSON.parse(input || "{}").session_id ?? "session").replace(/[^\w-]/g, "");
    const stamp = path.join(os.tmpdir(), `relay-usage-${session}`);
    const last = fs.existsSync(stamp) ? fs.statSync(stamp).mtimeMs : null;
    if (last != null && !due(last, Date.now())) return;
    fs.writeFileSync(stamp, "");
    if (last == null) return;
    const { usage } = await readUsage();
    console.log(JSON.stringify({ hookSpecificOutput: { hookEventName: "PostToolUse", additionalContext: say(usage) } }));
};

if (process.argv[1] === fileURLToPath(import.meta.url)) main().catch(() => {});
