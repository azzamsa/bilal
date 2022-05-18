.DEFAULT_GOAL := help

help: # https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

setup: ## Setup the repository
	echo "::: Setting up..."
	cargo install git-cliff

fmt: ## Format the codebase.
	cargo fmt --all

fmt_check: ## Check is the codebase properly formatted.
	cargo fmt --all -- --check

lint: ## Lint the codebase.
	cargo clippy --locked --all-targets

test:
	cargo test --all-targets

comply: fmt lint test ## Tasks to make the code-base comply with the rules. Mostly used in git hooks.

check: fmt_check lint test ## Check if the repository comply with the rules and ready to be pushed.

release:  ## Create a release
	bash scripts/release.sh $(version)

update_dependencies:
	cargo update
	cargo outdated --root-deps-only
