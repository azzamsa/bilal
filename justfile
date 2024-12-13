#!/usr/bin/env -S just --justfile

alias d := dev
alias r := run
alias f := fmt
alias l := lint
alias t := test
alias c := comply
alias k := check

[doc('List available commands')]
_default:
    just --list --unsorted

[confirm('⚠️ This command will alter your system. Run recipe `setup`?')]
[doc('Setup the repository')]
setup:
    just _cargo-install 'cargo-edit cargo-nextest cargo-outdated dprint git-cliff bacon typos-cli'

[doc('Tasks to make the code-base comply with the rules. Mostly used in git hooks')]
comply: _doc-check fmt lint test

[doc('Check if the repository comply with the rules and ready to be pushed')]
check: _doc-check fmt-check lint test

[doc('Develop the app')]
dev:
    bacon

[doc('Run the app')]
run:
    cargo run

[doc('Format the codebase.')]
fmt:
    cargo fmt --all
    dprint fmt

[doc('Check is the codebase properly formatted')]
fmt-check:
    cargo fmt --all -- --check
    dprint check

[doc('Lint the codebase')]
lint:
    cargo clippy --all-targets --all-features
    typos

[doc('Test the codebase')]
test:
    cargo nextest run

[doc('Create a new release. Example `cargo-release release minor --tag-name v0.2.0`')]
release level:
    cargo-release release {{ level }} --execute

[doc('Make sure the repo is ready for release')]
release-check level: check
    just up
    cargo-release release {{ level }}

[doc('Check the documentation')]
_doc-check:
    cargo doc --all-features --no-deps

[doc('Prepare release hooks')]
_release-prepare version:
    git-cliff --config .cliff.toml --output CHANGELOG.md --tag {{ version }}
    just fmt

[doc('Check dependencies health. Pass `--write` to upgrade dependencies')]
[unix]
up arg="":
    #!/usr/bin/env bash
    if [ "{{ arg }}" = "--write" ]; then
        cargo upgrade
        cargo update
        dprint config update
    else
        cargo outdated --root-deps-only
    fi;

#
# Helper
#

[doc('Install using plain cargo or cargo-binstall')]
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
