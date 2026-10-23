# Builds the wasm front end into www/pkg. Needs: rustup target add wasm32-unknown-unknown;
# cargo install wasm-bindgen-cli --version 0.2.128 (must match web/Cargo.toml).
$ErrorActionPreference = 'Stop'
Set-Location $PSScriptRoot
cargo build -p web --target wasm32-unknown-unknown --release
if ($LASTEXITCODE) { exit $LASTEXITCODE }
wasm-bindgen --target web --no-typescript --out-dir www/pkg target/wasm32-unknown-unknown/release/web.wasm
exit $LASTEXITCODE
