.PHONY: help build test run clippy fmt clean docker-build docker-up

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

setup: ## Install Rust components required for CI
	rustup component add clippy rustfmt

build: ## Build the Vella Engine for release
	cargo build --release

test: ## Run the full test suite
	cargo test --all-features

run: ## Boot the God-Tier OS locally
	cargo run --release

clippy: ## Run strict linter
	cargo clippy --all-targets --all-features -- -D warnings

fmt: ## Format code
	cargo fmt --all

clean: ## Clean target directory
	cargo clean

docker-build: ## Build the Docker image
	docker compose build

docker-up: ## Start the Edge Gateway in the background
	docker compose up -d
