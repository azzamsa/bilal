# Release Checklist

- Switch to a **master/main** branch locally.
- Pull the newest change in the upstream. Such a new merged PR.
- Run the lint check: `make check`.
- Run the release task: `make release version=v<mayor.minor.path>`. Such `make release version=v0.1.7`.
- Push the release commit to the **specified feature branch**.
- Check if [Continuous Integration](https://github.com/azzamsa/azzamsa/actions/workflows/ci.yml) workflow is completed successfully.
- Push the release tags: `git push --tags`.
- Wait for [Continuous Deployment](https://github.com/azzamsa/azzamsa/actions/workflows/cd.yml) workflow to finish.
- Create a new GitHub release with the created tag above, and copy the release news from the CHANGELOG.md.
