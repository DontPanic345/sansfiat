import assert from "node:assert/strict";
import crypto from "node:crypto";
import { test } from "node:test";
import { appJwt } from "./token.mjs";

test("appJwt signs a nine-minute RS256 token the App's public key verifies", () => {
    const { privateKey, publicKey } = crypto.generateKeyPairSync("rsa", { modulusLength: 2048 });
    const key = privateKey.export({ type: "pkcs1", format: "pem" });
    const now = Date.UTC(2026, 9, 23);
    const [head, body, sig] = appJwt(1234, key, now).split(".");
    assert.deepEqual(JSON.parse(Buffer.from(head, "base64url")), { alg: "RS256", typ: "JWT" });
    const claims = JSON.parse(Buffer.from(body, "base64url"));
    assert.deepEqual(claims, { iat: now / 1000 - 60, exp: now / 1000 + 480, iss: "1234" });
    const ok = crypto.createVerify("RSA-SHA256").update(`${head}.${body}`).verify(publicKey, Buffer.from(sig, "base64url"));
    assert.equal(ok, true);
});
