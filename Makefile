SHELL := bash

.PHONY: help
help: ## Show this help
	@egrep -h '\s##\s' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

install: ## Install or update dependencies
	echo "Install rustup using https://rustup.rs/ first"
	rustup component add rust-analyzer rustfmt clippy

run-hello-cargo: ## Run hello world
	cd a-getting-started || exit
	cargo run

run-the-book-tests: ## Run exercises from the Rust Programming Language book
	cd c-the-book-tests || exit
	cargo run
 	
run-guessing-game: ## Run guessing game
	cd b-game || exit
	cargo run

run-azure-ai102-quiz: ## Run quiz for Azure AI 102 Designing and Implementing a Microsoft Azure AI Solution
	cd b-azure-ai102-quiz || exit
	cargo run -p ai102_quiz_tui ./AI-102-Quiz.org ./data

run-guessing-game-debug: build-guessing-game ## Debug guessing game
	cd b-game || exit
	rust-gdb ../target/debug/b-game

clean: ## Clean rust projects
	cargo clean

build: ## Build for development
	cargo build

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

TAGS: ## Create tags file with universal ctags for emacs (-e)
	# Remove -e for vim
	ctags -e -R --languages=Rust --exclude=target --exclude=.git .

.PHONY: docs-open
docs-open: ## Open documentation for program and dependencies
	cargo doc --open
