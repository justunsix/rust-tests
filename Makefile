SHELL := /bin/bash

.PHONY: help
help: ## Show this help
		@egrep -h '\s##\s' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

install: ## Install or update dependencies
    echo "Install rustup using https://rustup.rs/ first"
		rustup component add rust-analyzer

run-hello-cargo: ## Run hello world
	  cd 1-Getting-Started/hello_cargo && cargo run

run-guessing-game: ## Run guessing game
		cd 2-Game/guessing_game && cargo run
