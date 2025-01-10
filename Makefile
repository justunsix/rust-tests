SHELL := /bin/bash

.PHONY: help
help: ## Show this help
	@egrep -h '\s##\s' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

install: ## Install or update dependencies
	echo "Install rustup using https://rustup.rs/ first"
	rustup component add rust-analyzer rustfmt clippy

run-hello-cargo: ## Run hello world
	cargo run -p a-getting-started

run-guessing-game: ## Run guessing game
	cargo run -p b-game

clean: ## Clean rust projects
	cargo clean

build: ## Build for development
	cargo build

build-release: ## Build for release
	cargo build --release

check: ## Check project(s) compile
	cargo check
	
.PHONY: lint-and-fix ## Format, Analyze and autofix files if possible
lint-and-fix:
	# -v : verbose
	cargo fmt -v
	cargo clippy
	cargo fix
