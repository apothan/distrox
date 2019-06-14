all: check

check:
	cargo check --target wasm32-unknown-unknown

build: wasmdir
	cargo build --target wasm32-unknown-unknown
	wasm-bindgen \
		target/wasm32-unknown-unknown/debug/distrox.wasm \
		--out-dir wasm \
		--target web

release: wasmdir
	cargo build --target wasm32-unknown-unknown --release
	wasm-bindgen \
		target/wasm32-unknown-unknown/release/distrox.wasm \
		--out-dir wasm \
		--target web

wasmdir:
	mkdir -p wasm
