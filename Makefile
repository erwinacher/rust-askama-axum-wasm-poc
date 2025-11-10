.PHONY: wasm run build watch clean

WASM_DIR=wasm
STATIC_DIR=static/pkg

wasm:
	cd $(WASM_DIR) && wasm-pack build --target web --out-dir ../$(STATIC_DIR)

run: wasm
	cargo run

build: wasm
	cargo build --release

watch:
	cargo watch -x "run"

clean:
	cargo clean
	rm -rf $(STATIC_DIR)
