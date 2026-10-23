// Installation tokens for the sansfiat-relay GitHub App, from RELAY_APP_ID and RELAY_APP_KEY.
//
//   node relay/token.mjs    # print a token (for a workflow step that needs one)
//
// A token lasts an hour and a step job runs for up to 345 minutes, so the crew asks for a
// token before each GitHub call and gets the cached one until it has under 15 minutes left.

import crypto from "node:crypto";
import { fileURLToPath } from "node:url";

const API = "https://api.github.com";
const REFRESH_MS = 15 * 60_000;

const b64url = (data) => Buffer.from(data).toString("base64url");

// A JWT the App signs for itself, good for nine minutes (GitHub allows ten). Backdated a
// minute for clock drift.
export const appJwt = (appId, key, now = Date.now()) => {
    const iat = Math.floor(now / 1000) - 60;
    const head = b64url(JSON.stringify({ alg: "RS256", typ: "JWT" }));
    const body = b64url(JSON.stringify({ iat, exp: iat + 540, iss: String(appId) }));
    const sig = crypto.createSign("RSA-SHA256").update(`${head}.${body}`).sign(key);
    return `${head}.${body}.${b64url(sig)}`;
};

const call = async (method, route, jwt) => {
    const res = await fetch(`${API}${route}`, {
        method,
        headers: { Accept: "application/vnd.github+json", Authorization: `Bearer ${jwt}` },
    });
    if (!res.ok) throw new Error(`GitHub ${method} ${route}: HTTP ${res.status} ${await res.text()}`);
    return res.json();
};

let cached = null;

// The installation token for `repo` ("owner/name"), minted again when under 15 minutes remain.
export const appToken = async (
    repo,
    appId = process.env.RELAY_APP_ID,
    key = process.env.RELAY_APP_KEY,
) => {
    if (cached && cached.expires - Date.now() > REFRESH_MS) return cached.token;
    if (!appId || !key) throw new Error("RELAY_APP_ID and RELAY_APP_KEY must be set");
    const jwt = appJwt(appId, key);
    const { id } = await call("GET", `/repos/${repo}/installation`, jwt);
    const { token, expires_at } = await call("POST", `/app/installations/${id}/access_tokens`, jwt);
    cached = { token, expires: new Date(expires_at).getTime() };
    return token;
};

if (process.argv[1] === fileURLToPath(import.meta.url)) {
    appToken(process.env.GITHUB_REPOSITORY ?? "DontPanic345/sansfiat")
        .then((t) => console.log(t))
        .catch((e) => {
            console.error(e.message);
            process.exit(1);
        });
}
