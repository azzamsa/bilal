# Release Checklist

- Run linting:

``` bash
$ find . | grep "\.rs" | xargs touch ; cargo clippy --all-features -- --deny warnings --deny clippy::pedantic --deny clippy::nursery
```

- Run `cargo update` and review dependency updates.
- Update the CHANGELOG.
- Update version numbers in `Cargo.toml` and `README.md`, Run `cargo update -p bilal` so that the Cargo.lock is updated.
- Create new branch (master branch is protected), commit with a message format: `v[0-9]+.[0-9]+.[0-9]+`, and push.
- Wait for a checks to pass, merge the branch to master, then tag a release with a copy of the relevant section of the CHANGELOG.
