set shell := ["bash", "-uc"]

check:
	@echo "RUSTUP_TOOLCHAIN is ${RUSTUP_TOOLCHAIN:-not set}"
	APP_BACKEND= cargo check --features hydrate,ssr --tests

fmt:
	@echo "RUSTUP_TOOLCHAIN is ${RUSTUP_TOOLCHAIN:-not set}"
	cargo fmt

fmt-check:
	@echo "RUSTUP_TOOLCHAIN is ${RUSTUP_TOOLCHAIN:-not set}"
	cargo fmt --check

lint:
	@echo "RUSTUP_TOOLCHAIN is ${RUSTUP_TOOLCHAIN:-not set}"
	APP_BACKEND= cargo clippy --no-deps -- -D warnings

test:
	@echo "RUSTUP_TOOLCHAIN is ${RUSTUP_TOOLCHAIN:-not set}"
	APP_BACKEND= cargo test

fix:
	@echo "RUSTUP_TOOLCHAIN is ${RUSTUP_TOOLCHAIN:-not set}"
	APP_BACKEND= cargo fix --allow-dirty --allow-staged

all: check fmt lint test

run backend="http://localhost:8080":
	cd realworld-frontend && \
		RUST_LOG=info,realworld_frontend=debug APP_BACKEND={{backend}} cargo leptos serve | jq

docker backend="http://localhost:8080":
	docker build \
		-t hseeberger/realworld-frontend \
		--build-arg "APP_BACKEND={{backend}}" \
		.
