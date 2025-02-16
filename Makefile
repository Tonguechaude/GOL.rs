# Variables
PROJECT_NAME = Jeu_de_la_Vie
RELEASE_DIR = target/release
WASM_TARGET = wasm32-unknown-unknown

# Default target
all: build
.PHONY: all

# Build the project in release mode
build:
	cargo build --release
.PHONY: build

# Build the project for WebAssembly
build-wasm:
	cargo build --release --target $(WASM_TARGET)
	wasm-bindgen --no-typescript --out-dir ./webapp/ --target web ./target/$(WASM_TARGET)/release/$(PROJECT_NAME).wasm
.PHONY: build-wasm

# Run the project in release mode
run:
	cargo run --release
.PHONY: run

# Clean the project
clean:
	cargo clean
.PHONY: clean

# Deploy the web project on your server
deploy:
	$(MAKE) build-wasm
	rsync -av --rsh=ssh webapp/* tongue@tonguechaude.fr:/var/www/tonguechaude.github.io/gol
.PHONY: deploy

# Install dependencies (e.g., wasm-pack)
install-deps:
	#curl https://sh.rustup.rs -sSf | sh
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
.PHONY: install-deps
