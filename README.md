[![CI](https://github.com/erwinacher/rust-askama-axum-wasm-poc/actions/workflows/ci.yml/badge.svg)](https://github.com/erwinacher/rust-askama-axum-wasm-poc/actions/workflows/ci.yml)

# rust-askama-axum-wasm-poc

> Proof of Concept for Rust + Askama templates + Axum server + WASM frontend

This is a minimal demonstration project showing how to integrate:

- **Rust** backend
- **Axum** web server
- **Askama** server side templates
- **WASM** compiled Rust frontend artifacts
- **static/** serving for wasm bundle output

### Project Structure

- src/ -> server code
- templates/ -> askama templates location, default for askama
- wasm/ -> wasm rust crate (compiled to /static/pkg)
- static/ -> static assets + wasm bundle output

### Requirements

- Rust stable
- `wasm-pack` installed

### Build & Run

```bash
make run
```

### Building WASM manually

```bash
cd wasm
wasm-pack build --target web --out-dir ../static/pkg
```

### Development Mode

to avoid reloading the server, this needs cargo-watch

```
make watch
```

### License

MIT
