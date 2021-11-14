# ChangeLog

## [Unreleased]

## [v0.1.9] - 2021-11-15

### Development
- Port to clap v2. The v3 API is always changing.

## [v0.1.8] - 2021-04-22

### Bug Fixes
- Fixed wrong remaining time. The hour doesn't shown even if it's `not equal to 0`.
- Leading zero in minute not shown.
- Wrong critical state. The app only check for the remaining time. So it will says `critical` even the hour > 0.

## [v0.1.7] - 2021-04-20

### Bug Fixes
- Use salah as an argument instead of an option. Now it's `bilal all` instead of `bilal --all`.
- Icon prefix only shown in JSON mode. Otherwise, it's useless.

## [v0.1.6] - 2021-04-19

### Bug Fixes
- removed many `panics` issue, by migrating to our own `islam` library.

## [v0.1.1] - 2020-09-25

### Features
- Support json output
- Print salah in red if remaining time is low
- Use color in help menu

## [v0.1.0] - 2020-09-25

Initial release
