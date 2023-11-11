set shell := ["bash", "-uc"]

check:
	BACKEND="dummy" cargo check --features hydrate,ssr --tests

fmt:
	cargo fmt

fmt-check:
	cargo fmt --check

lint:
	BACKEND="dummy" cargo clippy --no-deps -- -D warnings

test:
	BACKEND="dummy" cargo test

fix:
	BACKEND="dummy" cargo fix --allow-dirty --allow-staged

all: check fmt lint test

run backend="http://localhost:8080":
	cd realworld-frontend && \
		RUST_LOG=info,realworld_frontend=debug BACKEND={{backend}} cargo leptos serve | jq
