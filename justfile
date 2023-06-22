#!/usr/bin/env -S just --justfile

alias d := dev
alias t := test

# List available commands.
_default:
    just --list --unsorted

# Setup the repository.
setup:

# Setup the development tools.
_setup-dev:
    cargo install --locked cargo-nextest git-cliff cargo-watch dprint cargo-edit cargo-outdated spacer

# Develop the app.
dev:
    cargo watch -x 'clippy --all-targets --all-features' | spacer

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

# Create a new release. Example `cargo-release release minor --tag-name v0.2.0`
release level:
    cargo-release release {{ level }} --execute

# Make sure the repo is ready for release
_release-check level:
    just up
    cargo-release release {{ level }}

# Release hooks
_prepare-release version:
    git-cliff --config configs/cliff.toml --output CHANGELOG.md --tag {{ version }}
    just fmt

# Check dependencies health. Pass `--write` to uppgrade dependencies.
[unix]
up arg="":
    #!/usr/bin/env bash
    if [ "{{ arg }}" = "--write" ]; then
        cargo upgrade
        cargo update
    else
        cargo outdated --root-deps-only
    fi;

[windows]
up arg="":
    #!powershell.exe
    if ( "{{ arg }}" -eq "--write") {
        cargo upgrade
        cargo update
    }
    else {
        cargo outdated --root-deps-only
    }
