cargo install wasm-pack
cd web/wasm
wasm-pack build --target web --out-dir ../static/pkg
