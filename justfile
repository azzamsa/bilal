#!/usr/bin/env -S just --justfile

alias d := dev
alias t := test

# List available commands.
_default:
    just --list --unsorted

# Setup the repository.
setup: _areyousure
    just _cargo-install 'cargo-edit cargo-nextest cargo-outdated cargo-watch dprint git-cliff spacer'

# Tasks to make the code-base comply with the rules. Mostly used in git hooks.
comply: _doc-check fmt lint test

# Check if the repository comply with the rules and ready to be pushed.
check: fmt-check lint test

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

# Lint the codebase.
lint:
    cargo clippy

# Test the codebase.
test: test-unit
    cargo test --doc
    cargo nextest run

# Test unit tests only
test-unit:
    cargo nextest run --lib

# Create a new release. Example `cargo-release release minor --tag-name v0.2.0`
release level:
    cargo-release release {{ level }} --execute

# Make sure the repo is ready for release
release-check level:
    just up
    cargo-release release {{ level }}

# Lint the docstring.
_doc-check:
    cargo doc --all-features --no-deps

# Release hooks
_release-prepare version:
    git-cliff --config configs/cliff.toml --output CHANGELOG.md --tag {{ version }}
    just fmt

# Check dependencies health. Pass `--write` to uppgrade dependencies.
[unix]
up arg="":
    #!/usr/bin/env bash
    if [ "{{ arg }}" = "--write" ]; then
        cargo upgrade && cargo update
    else
        cargo outdated --root-deps-only
    fi;

[windows]
up arg="":
    #!powershell.exe
    if ( "tool" -eq "--write") {
        cargo upgrade
        cargo update
    }
    else {
        cargo outdated --root-deps-only
    }

#
# Helper
#

[unix]
_cargo-install tool:
    #!/usr/bin/env bash
    if command -v cargo-binstall >/dev/null 2>&1; then
        echo "cargo-binstall..."
        cargo binstall --no-confirm --no-symlinks {{ tool }}
    else
        echo "Building from source"
        cargo install --locked {{ tool }}
    fi

[unix]
_areyousure:
    #!/usr/bin/env bash
    echo -e "This command will alter your system. ⚠️
    You are advised to run in inside containerized environment.
    Such as [toolbx](https://containertoolbx.org/).

    If you are unsure. Run the installation commands manually.
    Take a look at the 'setup' recipe in the Justfile.\n"

    read -p "Are you sure you want to proceed? (Y/n) " response;
    if [[ $response =~ ^[Yy] ]]; then
        echo "Continue!";
    else
        echo "Cancelled!";
        exit 1;
    fi
