# rust-wasm-indexeddb

```
cargo install wasm-pack
# https://rustwasm.github.io/wasm-pack/book/commands/build.html
~/.cargo/bin/wasm-pack build --dev --target web
npx http-server

```

https://github.com/rustwasm/wasm-bindgen/pull/2012
https://github.com/rustwasm/wasm-bindgen/releases/tag/0.2.59

```json
{
   "rust-analyzer.cargo.target": "wasm32-unknown-unknown",
    "rust-analyzer.checkOnSave.allTargets": false,
    "rust-analyzer.procMacro.enable": true,
    "rust-analyzer.experimental.procAttrMacros": true,
    "rust-analyzer.updates.channel": "nightly",
    "rust-analyzer.cargo.runBuildScripts": true,
    "rust-analyzer.trace.server": "messages",
    "rust-analyzer.workspace.symbol.search.kind": "all_symbols",
    "rust-analyzer.workspace.symbol.search.scope": "workspace_and_dependencies"
}
```