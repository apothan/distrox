all: check

check:
	cargo check --target wasm32-unknown-unknown

build:
	cargo build --target wasm32-unknown-unknown --release
	mkdir -p wasm
	wasm-bindgen \
		target/wasm32-unknown-unknown/debug/wasm_bindgen_minimal_example.wasm \
		--out-dir wasm \
		--target web

release:
	cargo build --target wasm32-unknown-unknown --release
	mkdir -p wasm
	wasm-bindgen \
		target/wasm32-unknown-unknown/release/wasm_bindgen_minimal_example.wasm \
		--out-dir wasm \
		--target web
