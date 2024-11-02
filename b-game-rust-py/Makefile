SHELL := /bin/bash

.PHONY: help
help: ## Show this help
	@egrep -h '\s##\s' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

develop: ## Build development
	maturin develop

release: ## Build for release - a wheel for distribution
	maturin build 

run: develop ## Run guessing game
	python3 guessing-game-test.py
