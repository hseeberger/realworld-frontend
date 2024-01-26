set shell := ["bash", "-uc"]

check:
	APP_BACKEND= cargo check --features hydrate,ssr

fmt toolchain="+nightly":
	cargo {{toolchain}} fmt

fmt-check toolchain="+nightly":
	cargo {{toolchain}} fmt --check

lint:
	APP_BACKEND= cargo clippy --no-deps -- -D warnings

test:
	APP_BACKEND= cargo test

fix:
	APP_BACKEND= cargo fix --allow-dirty --allow-staged

all: check fmt lint test

run backend="http://localhost:8080":
	cd realworld-frontend && \
		RUST_LOG=info,realworld_frontend=debug APP_BACKEND={{backend}} cargo leptos serve

docker backend="http://localhost:8080":
	docker build \
		-t hseeberger/realworld-frontend \
		--build-arg "APP_BACKEND={{backend}}" \
		.
