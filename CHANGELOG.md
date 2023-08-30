# Changelog

All notable changes to this project will be documented in this file.

## [1.5.1] - 2023-08-30

### Bug fixes

- Wrong app name in release step ([3a4f2fe](https://github.com/azzamsa/bilal/commit/3a4f2fe832033c8089458656d3c9a872a13336b2))

## [1.5.0] - 2023-08-30

### Features

- Use clearer terms to indicate the remaining time ([9ba35c5](https://github.com/azzamsa/bilal/commit/9ba35c552ef1dccc4c0f8ee5cc66289a7633c5ff))

## [1.4.0] - 2023-08-30

### Bug fixes

- Upgrade to islam v3 ([d45b8c2](https://github.com/azzamsa/bilal/commit/d45b8c2a65469f7f40fcb9243d140043ad00992e))

## [1.3.0] - 2023-06-22

### Bug fixes

- Upgrade to islam v2.0.0 ([eb780eb](https://github.com/azzamsa/bilal/commit/eb780eb860525c7896475f74ae28c68f3603d306))

## [1.2.0] - 2023-01-22

### Bug fixes

- Some API changes in islam v1.0 ([735b442](https://github.com/azzamsa/bilal/commit/735b442c616402b7209068315cb1ffd506f81e20))

## [1.1.0] - 2023-01-03

### Features

- Print config snippets of error location ([734fab4](https://github.com/azzamsa/bilal/commit/734fab40a8e43e2e4a41d50e156eeb49a30a5575))

## [1.0.0] - 2022-12-28

### Performance

- Migrate to `owo-color` ([6777ba9](https://github.com/azzamsa/bilal/commit/6777ba9759a559884022163565a2ee19242dec3c))

  It has fewer dependencies

## [0.2.1] - 2022-12-22

### Features

- Improve error message ([d9f0296](https://github.com/azzamsa/bilal/commit/d9f0296961bd6aab042f869d59734acde49982bb))

  Improve error message for `Invalid Config`, and `Config not found`.

### Bug fixes

- Avoid checking if the config exists ([f931099](https://github.com/azzamsa/bilal/commit/f9310994465a985801d0cb1aa3076446f7cac819))

  This eliminates a [race condition between "check to see if file exists" and "open file if it exists"](https://en.wikipedia.org/wiki/Time-of-check_to_time-of-use).

## [0.2.0] - 2022-12-19

### Bug fixes

- Upgrade `islam` dependency ([d51db32](https://github.com/azzamsa/bilal/commit/d51db32cab86c23e9e8f6d5725cfb81446dbff6e))

## [0.1.8] - 2021-04-22

### Bug fixes

- Wrong critical state ([3194a15](https://github.com/azzamsa/bilal/commit/3194a1567c5f6ec531243eace6bc2e24026fe3f3))

  The app only check for the remaining time.
  So it will says `critical` even the hour > 0.

- Leading zero in minute not shown ([d391811](https://github.com/azzamsa/bilal/commit/d3918115f6b7e91241c5c344362d91d08dce8cc0))

  Use `strftime` instead of plain `string format`.

- Wrong remaining time ([8831a8b](https://github.com/azzamsa/bilal/commit/8831a8b70e1a7d0122dc226cb9d4244543172dd1))

  The hour doesn't shown even if it's `not equal to 0`.

## [0.1.7] - 2021-04-20

### Bug fixes

- Remove `chrono` dependency ([f608c49](https://github.com/azzamsa/bilal/commit/f608c495dddfaea12b73771a540ecd139a91a99f))

  Now it's included as module in `islam`

- Salah now uses shorter module name ([b2d8c0f](https://github.com/azzamsa/bilal/commit/b2d8c0f624c45725594b87edc35cfd4dc763fa09))
- State must be present ([37d6f21](https://github.com/azzamsa/bilal/commit/37d6f21717a531ce2e7c6b8a636234f7b7a25235))
- Use salah as an argument instead of an option ([44c107a](https://github.com/azzamsa/bilal/commit/44c107a2b421914231c06b8d45cd7da62b42b0b6))

  Using salah as an argument is somewhat wrong

## [0.1.3] - 2020-09-27

### Features

- Add icon to make it easier to differentiate ([2c0f620](https://github.com/azzamsa/bilal/commit/2c0f620dd0669cddffaf2958e88e312e086e4655))

## [0.1.2] - 2020-09-27

### Features

- Support `madhab` and `method` in configuration ([65ea4f9](https://github.com/azzamsa/bilal/commit/65ea4f9190e2dbb8fe5ff47fd8b1505b2ecc5a6a))
- Support windows-pc config location ([69e37d7](https://github.com/azzamsa/bilal/commit/69e37d72b7cdbb830440ba03dc10c420309eb982))

  Initial support for windows-pc

- Make config handling more robust ([03bf047](https://github.com/azzamsa/bilal/commit/03bf0472c44074b06b11e8a2d27ed6d922bd2625))

## [0.1.1] - 2020-09-25

### Features

- Support json output ([417c24f](https://github.com/azzamsa/bilal/commit/417c24f81fedb401e41995ec27437e7fcec4b134))

  To integrate with i3status-rs

- Print salah in red if remaining time is low ([2cce260](https://github.com/azzamsa/bilal/commit/2cce26049c0b44048939e92d45c66d5099f3d8a1))
- Use color in help ([d20c8cb](https://github.com/azzamsa/bilal/commit/d20c8cb5d66dd2e290b0e9443f1e1488c8ee1d95))

### Bug fixes

- Don't supply any key if it's normal ([922967a](https://github.com/azzamsa/bilal/commit/922967a1f163e184a8b549697d31e227e5f8ae02))

  `Info` still gives a color in status bar.

- Don't hard-code clap version ([abe2a8f](https://github.com/azzamsa/bilal/commit/abe2a8f05d53218031c0d64d5cb4a829a2974cca))

## [0.1.0] - 2020-09-25

### Features

- Show help if no argument supplied ([9b545c8](https://github.com/azzamsa/bilal/commit/9b545c889c0f76f0a4e007caf4914340ba016732))
- Support displaying next salah ([87935ab](https://github.com/azzamsa/bilal/commit/87935ab2db72d104d98ad2468a81432e2c9057df))
- Support displaying current salah ([4f7f87b](https://github.com/azzamsa/bilal/commit/4f7f87b99f059e7dff40b7883262fe1f9ae81ee1))

### Bug fixes

- Use shorter output ([c07db76](https://github.com/azzamsa/bilal/commit/c07db7664a9d92cec67c377cdb01d781931d9a40))

  Because mostly it will be used with other tools. Such as i3status-rs.

- Display remaining time instead of exact time of current salah ([5a34e1b](https://github.com/azzamsa/bilal/commit/5a34e1bbfc337cf35cafbc197deb0d8ffac02c50))
