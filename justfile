#!/usr/bin/env -S just --justfile

shebang := if os() == 'windows' { 'powershell.exe' } else { '/usr/bin/sh' }

alias d := dev
alias t := test

# List available commands.
_default:
    just --list --unsorted

# Setup the repository.
setup:
    git cliff --version || cargo install --locked git-cliff
    cargo nextest --version || cargo install --locked cargo-nextest
    cargo-set-version --help || cargo install --locked cargo-edit
    cargo watch --version || cargo install --locked cargo-watch
    dprint --version || cargo install --locked dprint

# Develop the app.
dev:
    cargo watch -x 'clippy --all-targets --all-features'

# Format the codebase.
fmt:
    cargo fmt --all
    dprint fmt --config configs/dprint.json

# Check is the codebase properly formatted.
fmt-check:
    cargo fmt --all -- --check
    dprint check --config configs/dprint.json

# Lint the docstring.
_lint_doc:
    cargo doc --all-features --no-deps

# Lint the codebase.
lint:
    cargo clippy

# Test the codebase.
test:
    cargo test --doc
    cargo nextest run

# Tasks to make the code-base comply with the rules. Mostly used in git hooks.
comply: fmt lint _lint_doc test

# Check if the repository comply with the rules and ready to be pushed.
check: fmt-check lint test

# Open documentation.
doc:
    cargo doc --open

# Create a new release. Example `just release v2.2.0`
release version:
    bash scripts/release.sh {{ version }}

# Check dependencies health. Pass `--write` to uppgrade dependencies.
up arg="":
    #!{{ shebang }}
    if [ "{{ arg }}" = "--write" ]; then
    	cargo upgrade
    	cargo update
    else
        cargo outdated
    fi;
