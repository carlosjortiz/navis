# Navis

API client desktop, OSS, file-first. Trabajá con tus APIs desde un editor que entiende `.http` con superpoderes — variables jerárquicas, scripts JS, environments por API, persistencia en filesystem (sin DB ni cloud).

## Status

Pre-MVP — desarrollo activo. Primera release pública: **v0.1.0**.

## Stack

- **Backend**: Rust + Tauri 2 + reqwest + tree-sitter + deno_core
- **Frontend**: React + TypeScript + Vite + Tailwind + Zustand + CodeMirror
- **Persistencia**: filesystem local con JSON5 (sin servidor, sin DB)

## License

Dual-licensed at your option under either:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project shall be dual licensed as above, without any additional terms or conditions.
