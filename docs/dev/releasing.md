# Release Checklist

- Run `cargo update` and review dependency updates.
- Run the lint check: `cargo clippy --all-features -- --deny warnings --deny clippy::pedantic --deny clippy::nursery`
- Update the CHANGELOG.
- Update version numbers in `Cargo.toml`, then run `cargo update -p bilal` so that the Cargo.lock is updated.
- Create a commit with a message format: `v[0-9]+.[0-9]+.[0-9]+`, and push.
- Wait for the checks to pass, tag a commit with a release tag, then push the tag.
- Create a new GitHub release with the created tag above.
