set shell := ["bash", "-uc"]

# Needed at compile time, actual value only matters for trunk serve/build.
export BACKEND := "dummy"

check:
	cargo check --tests

fmt:
	cargo +nightly fmt

fmt-check:
	cargo +nightly fmt --check

lint:
	cargo clippy --no-deps -- -D warnings

test:
	cargo test

fix:
	cargo fix --allow-dirty --allow-staged

all: check fmt lint test

run backend="http://localhost:8080":
	cd realworld-frontend && BACKEND={{backend}} trunk serve --port 8090 --open
