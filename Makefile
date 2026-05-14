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

clippy-check-azure-ai102-quiz: ## Check ai102 quiz tui
	cargo clippy -p ai102_quiz_tui

run-azure-ai102-quiz: ## Run quiz for Azure AI 102 Designing and Implementing a Microsoft Azure AI Solution
	cargo run -p ai102_quiz_tui ./b-azure-ai102-quiz/AI-102-Quiz.org ./b-azure-ai102-quiz/data
		
run-guessing-game-debug: build-guessing-game ## Debug guessing game
	rust-gdb target/debug/b-game

clean: ## Clean rust projects
	cargo clean

build: ## Build for development
	cargo build

build-guessing-game: ## Build for development
	cargo build -p b-game

build-release: ## Build for release
	cargo build --release

check: ## Check project(s) compile
	cargo check
	
.PHONY: lint-and-fix
lint-and-fix: ## Format, Analyze and autofix files if possible
	# -v : verbose
	cargo fmt -v
	cargo clippy
	cargo fix

.PHONY: docs-open
docs-open: ## Open documentation for program and dependencies
	cargo doc --open
