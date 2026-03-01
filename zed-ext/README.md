# TDR Scene Zed Extension

This directory hosts the in-progress Zed editor port of the existing VS Code extension.

## Repo layout

- `extension.toml` — wires grammars and language server and points at pinned grammar repos.
- `grammars/tdr*` — vendored grammar builds for Zed; keep `tree-sitter.json` metadata in sync.
- `languages/` — Zed language configs plus highlight/fold/indent/injection queries.
- `tdr-scene-extension` — Rust host launching the bundled `tdr-lsp` binary via `language_server_command`.
- `bin/<platform-arch>/` — ship-ready `tdr-lsp` binaries (e.g. `linux-x64`, `win32-x64`).

## Getting a working preview

1. Regenerate grammars and build WASM artifacts (using the installed `tree-sitter` CLI)
   ```bash
   cd grammars/tdr && tree-sitter generate && tree-sitter build --wasm --output ../tdr.wasm
   cd ../tdr_obj && tree-sitter generate && tree-sitter build --wasm --output ../tdr_obj.wasm
   ```
   Use `tree-sitter parse example.tdr` to sanity-check output.

2. Copy `tdr-lsp` binaries into `bin/<platform-arch>/` for every platform you plan to test. Current layout expects folders like `linux-x64` and `win32-x64`.

3. From this directory, run the Rust extension build in dev mode:
   ```bash
   cargo build --release
   ```
   Then launch Zed with `zed extension run ..\..\extension.toml` (CLI support required).

4. Open sample `.tdr` or `.paf` files in Zed and verify:
   - Syntax colors render (Tree-sitter queries load correctly).
   - `<object type="raw">` blocks inject OBJ highlighting.
   - Language server command starts the bundled `tdr-lsp` binary (logs appear in Zed dev console).

## Remaining polish

- Add Tree-sitter corpus tests plus CI guard.
- Expand queries (folds, indents, injections) as grammar evolves.
- Package binaries for macOS and Linux arm64.
- Document `zed extension build` steps once available.
