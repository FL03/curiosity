FROM rust:latest as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as builder-base

RUN apt-get install -y \
    clang \
    git \
    protobuf-compiler

RUN rustup default nightly && \
    rustup target add wasm32-wasi --toolchain nightly

RUN curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash

FROM builder-base as builder

ENV CARGO_TERM_COLOR=always

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN cargo build --release --target wasm32-wasi
RUN /root/.wasmedge/bin/wasmedgec target/wasm32-wasi/release/curiosity.wasm curiosity.aot.wasm

# FROM wasmedge/slim:0.12.0 as post-builder

# COPY --from=builder /workspace/target/wasm32-wasi/release/curiosity.wasm /curiosity.wasm
# RUN wasmedgec curiosity.wasm curiosity.aot.wasm

FROM scratch

COPY --from=builder /workspace/curiosity.aot.wasm /curiosity.aot.wasm

EXPOSE 8080

ENTRYPOINT [ "curiosity.aot.wasm" ]