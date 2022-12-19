#!/usr/bin/env bash

untracked_files=$(git ls-files . --exclude-standard --others)

# check for untracked files
if [ -n "$untracked_files" ]; then
  echo "warn: Please stash or remove the untracked files"
  exit 1
fi

# if tag is not passed
if [ -z "$1" ]; then
  echo "warn: Please provide a tag"
  exit 1
fi

# takes the tag as an argument (e.g. v0.1.0)
# update the version
new_version=${1#v} # strip the `v` prefix
cargo set-version "$new_version"

# update the changelog
git-cliff --tag "$1" --sort newest --config configs/cliff.toml >CHANGELOG.md
# format newly added changelog file
just fmt

git add --all && git commit --message="$1"
git show
git tag --sign --annotate "$1" --message="$1" --message="For details, see the CHANGELOG.md"
git tag --verify "$1"
