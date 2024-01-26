ARG RUST_VERSION=1.75.0

FROM rust:${RUST_VERSION}-bookworm AS builder
ARG APP_BACKEND
WORKDIR /build
COPY . .
RUN \
  toolchain=$(grep channel rust-toolchain.toml | sed -r 's/channel = "(.*)"/\1/') && \
  rustup toolchain install $toolchain && \
  rustup target add --toolchain $toolchain wasm32-unknown-unknown && \
  cargo install cargo-leptos
RUN \
  --mount=type=cache,target=/build/target/ \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  APP_BACKEND=$APP_BACKEND cargo leptos build --release -p realworld-frontend && \
  mkdir /app && \
  cp ./target/release/realworld-frontend /app/ && \
  cp -R ./target/site /app/

FROM debian:bookworm-slim AS final
RUN \
  apt-get update && \
  apt-get install -y --no-install-recommends ca-certificates
RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "10001" \
  appuser
COPY --from=builder /app/realworld-frontend /usr/local/bin/
RUN chown appuser /usr/local/bin/realworld-frontend
COPY --from=builder /app/site /app/site/
RUN chown -R appuser /app/site
USER appuser
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
ENV RUST_LOG="realworld_frontend=debug,info"
WORKDIR /app
ENTRYPOINT ["realworld-frontend"]
EXPOSE 8080/tcp
