#!/usr/bin/env bash

untracked_files=$(git ls-files . --exclude-standard --others)

# check for untracked files
if [ -n "$untracked_files" ]; then
    echo "warn: Please stash or remove the untracked files"
    exit 1
fi

# takes the tag as an argument (e.g. v0.1.0)
if [ -n "$1" ]; then
    # update the version
    new_version=${1#v} # strip the `v` prefix
    sed --in-place "0,/^version = .*/s//version = \"$new_version\"/" Cargo.toml

    # update Cargo.lock
    cargo update -p zman

    # update the changelog
    git-cliff --tag "$1" --sort newest > CHANGELOG.md
    git add -A && git commit -m "$1"
    git show
    git tag -s -a "$1" -m "$1" -m "For details, see the CHANGELOG.md"
    git tag -v "$1"
else
    echo "warn: Please provide a tag"
    exit 1
fi
