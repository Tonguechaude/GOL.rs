# Variables
PROJECT_NAME = jeu_de_la_vie
WASM_TARGET = wasm32-unknown-unknown

# Default target
all: build-wasm
.PHONY: all

# Build the project for WebAssembly
build-wasm:
	cargo build-wasm
	wasm-bindgen --no-typescript --out-dir ./webapp/ --target web ./target/$(WASM_TARGET)/release/$(PROJECT_NAME).wasm
.PHONY: build-wasm

# Install dependencies (e.g., wasm-pack)
install-deps:
	#curl https://sh.rustup.rs -sSf | sh
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
	cargo install wasm-bindgen-cli wasm-server-runner
.PHONY: install-deps
