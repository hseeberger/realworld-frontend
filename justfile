set shell := ["bash", "-uc"]

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

all: fmt check lint test

run:
	cd realworld-frontend && dx serve --port 8082 --hot-reload

@jwt-generate-key:
	cargo run --quiet --example jwt -- generate-key

@jwt-create-token subject:
	cargo run --quiet --example jwt -- create-token {{subject}}

@jwt-verify-token token:
	cargo run --quiet --example jwt -- verify-token {{token}}
