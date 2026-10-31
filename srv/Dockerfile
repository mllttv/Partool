ARG BUILD_MODE=debug

# Stage 1: npm
FROM debian:forky-slim AS npm
WORKDIR /srv
COPY html/static/package*.json ./html/static/
RUN apt-get update -q && \
    DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends -yq ca-certificates curl xz-utils && \
    rm -rf /var/lib/apt/lists/*
COPY . .

# Stage 2 (Debug): Rust
FROM npm AS builder-debug
RUN apt-get update -q && \
    DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends -yq \
    cargo rustc libstd-rust-dev-wasm32 rust-llvm \
    mold pkg-config libsqlite3-dev dart-sass binaryen && \
    cargo install wasm-bindgen-cli --version 0.2.130 --root /usr/local && \
    ./compile.sh --no-run && \
    rm -rf /var/lib/apt/lists/*

# Stage 2 (Release): Rust
FROM npm AS builder-release
RUN apt-get update -q && \
    DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends -yq \
    cargo rustc libstd-rust-dev-wasm32 rust-llvm \
    mold pkg-config libsqlite3-dev dart-sass binaryen && \
    cargo install wasm-bindgen-cli --version 0.2.130 --root /usr/local && \
    ./compile.sh --release --no-run && \
    rm -rf /var/lib/apt/lists/*

# Stage 2 (Prebuilt): Rust
FROM npm AS builder-prebuilt
RUN if ! ([ -f ./main ] && [ -f ./html/pkg/htmlvm_bg.wasm ]); then \
        echo "Error: Prebuilt artifacts are missing." >&2; exit 1; \
    fi

FROM builder-${BUILD_MODE} AS builder

# Stage 3: Runner
FROM debian:forky-slim
WORKDIR /srv
RUN install -d -m 0755 -o 10001 -g 10001 /srv/var /srv/tmp && \
    apt-get update -q && \
    DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends -yq ca-certificates libsqlite3-0 curl 7zip && \
    rm -rf /var/lib/apt/lists/*
COPY --chown=10001:10001 --from=builder /srv/html/ ./html/
COPY --chown=10001:10001 --from=builder /srv/main ./main
COPY --chown=10001:10001 --from=builder /srv/Rust/keys/ ./Rust/keys/
USER 10001:10001
EXPOSE 8080
CMD ["./main", "--host", "0.0.0.0", "--port", "8080"]