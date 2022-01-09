all: build

build:
	cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/token.wasm .
	wasm-opt -O3 token.wasm -o token.wasm