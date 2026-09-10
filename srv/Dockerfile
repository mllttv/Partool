# Rust Builder Stage (Rust -> srv/main)
FROM debian:forky-slim AS rust-builder
ENV APP /srv
WORKDIR $APP
RUN apt-get update -q && \
    DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends -yq gcc cargo rustc ca-certificates libssl-dev pkg-config && \
    rm -rf /var/lib/apt/lists/*
COPY Rust/ ./Rust/
WORKDIR $APP/Rust
RUN cargo build --release

# Runner Stage
FROM debian:forky-slim
ENV APP /srv
WORKDIR $APP
COPY --from=rust-builder $APP/Rust/target/release/main $APP/main
COPY html/ $APP/html/
COPY tmp/ $APP/tmp/

EXPOSE 8080
CMD bash -c "./main --host 0.0.0.0 --port 8080"