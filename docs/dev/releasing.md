## Release Checklist

- Ensure local `master` is up to date to `origin/master`.
- Run `just up` to check outdated dependencies. Run `just up --write` and review dependency updates.
  Commit updated `Cargo` files.
- Run `just check`. To make sure that everything is ok.
- Run the release task `just release v<major.minor.path>`. Such `just release v0.1.7`.
- **Push the release commit to GitHub**, NOT including the tag. (But do not publish a new version of bilal to crates.io yet.)
- Once CI for `master` finishes successfully, **push the version tag**.
  (Trying to do this in one step seems to result in GitHub Actions not seeing the tag
  push and thus not run the release workflow.)
- Wait for CI to finish creating the release. If the release build fails, then
  delete the tag from GitHub, make fixes, re-tag, delete the release, and push.
- Copy the relevant section of the CHANGELOG.md to the tagged release notes.
- Run `cargo publish`.
