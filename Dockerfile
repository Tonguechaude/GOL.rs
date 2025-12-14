FROM rust:latest

# Install nightly toolchain and WASM target
RUN rustup toolchain install nightly --allow-downgrade && \
  rustup default nightly && \
  rustup target add wasm32-unknown-unknown

# Install WASM tools
RUN cargo install wasm-bindgen-cli --version 0.2.104 && \
  cargo install wasm-server-runner

# Keep container running for CI jobs
CMD ["bash"]
