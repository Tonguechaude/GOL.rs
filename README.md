# Game Of Life

![License](https://img.shields.io/badge/License-GPLv3-blue.svg)

First of all, sorry for my english. Please feel free to contribute for a better translation <3

The **Game Of Life** is an automated cells simulation created by the mathematician John Conway. This project is an implementation build in **Rust**, he offer two differents GUI. Both of those GUI are propulsed by the **Bevy motor**. One GUI is in pure Rust and the other is also pure Rust but build in the **WASM** target. Thanks to those GUI you can try the project by typing in your terminal `cargo run` and by clicking on this link : [gol.tonguechaude.fr](https://gol.tonguechaude.fr)

---

## What's the point ?

Nice question ! I already did that in Java but I don't like the fact that JVM take 2 GB of RAM, so I try in Rust

---

## Fonctionnality

- **GOL Simulation** : Implementation of the classic algorithm
- **GUI** : Bevy provide a great 2D interactive interface
- **WebAssembly version** : You can play in your browser

---

## Prerequisites

To Run the project you need some tools :

- Your :brain:
- An internet connection

And a real list of prerequisites :

- **Rust Toolchain** :
  - `rustc` : (rust compiler)
  - `cargo` : (Our god :pray:)
  - `rustup` : (version manager)
- **Other tools for WASM** :
  - `wasm32-unknown-unknown` (additional target for WASM compilation)
  - `wasm-bindgen-cli` (to generate JS bindings)
  - `wasm-server-runner` (to run the project in local environment)

---

## Installation

1. Clone the repository :

```bash
git clone https://gitlab.com/Tonguechaude/gol.git
cd gol
```

2. Setup your environment :

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli wasm-server-runner
```

## Usage

### Rust version (classic bevy window)

To compile and run in release mode :

```bash
cargo run --release
```

### WASM version (local)

For running the project in your browser :

```bash
cargo run --target wasm32-unknown-unknown
```

or

```bash
cargo serveur
```

### WASM version (The one i deploy on [gol.tonguechaude.fr](htts://gol.tonguechaude.fr))

To compile the project in WASM and generate JS files

```bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --out-dir ./webapp/ --target web ./target/wasm32-unknown-unknown/release/jeu_de_la_vie.wasm
wasm-opt -Oz -o ./webapp/jeu_de_la_vie_bg.wasm ./webapp/jeu_de_la_vie_bg.wasm # Optimize WASM file size
```

Testing WASM in webserver environment :
(You can't access from file:// cause Browsers dont allow import module to prevent malicious malware to access to your filesystem so you have to do this)

```bash
cd webapp
python3 -m http.server 8080
```

## Docker

Actually I exposed a docker image here : [tonguechaude/rust-wasm-builder](https://hub.docker.com/r/tonguechaude/rust-wasm-builder)

This image exist just because my runner CPU is so bad :(, so I need to optimize compute time in CI.

## License

We are doing free software here !! The code is under **GNU GPL v3**

Have fun with this project !

