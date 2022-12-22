# Changelog

All notable changes to this project will be documented in this file.

## [0.2.1] - 2022-12-22

### Features

- Improve error message ([d9f0296](d9f0296961bd6aab042f869d59734acde49982bb))

### Bug fixes

- Avoid checking if the config exists ([f931099](f9310994465a985801d0cb1aa3076446f7cac819))

## [0.2.0] - 2022-12-19

### Bug fixes

- Upgrade `islam` dependency ([d51db32](d51db32cab86c23e9e8f6d5725cfb81446dbff6e))

## [0.1.8] - 2021-04-22

### Bug fixes

- Wrong critical state ([3194a15](3194a1567c5f6ec531243eace6bc2e24026fe3f3))
- Leading zero in minute not shown ([d391811](d3918115f6b7e91241c5c344362d91d08dce8cc0))
- Wrong remaining time ([8831a8b](8831a8b70e1a7d0122dc226cb9d4244543172dd1))

## [0.1.7] - 2021-04-20

### Bug fixes

- Remove `chrono` dependency ([f608c49](f608c495dddfaea12b73771a540ecd139a91a99f))
- Salah now uses shorter module name ([b2d8c0f](b2d8c0f624c45725594b87edc35cfd4dc763fa09))
- State must be present ([37d6f21](37d6f21717a531ce2e7c6b8a636234f7b7a25235))
- Use salah as an argument instead of an option ([44c107a](44c107a2b421914231c06b8d45cd7da62b42b0b6))

## [0.1.3] - 2020-09-27

### Features

- Add icon to make it easier to differentiate ([2c0f620](2c0f620dd0669cddffaf2958e88e312e086e4655))

## [0.1.2] - 2020-09-27

### Features

- Support `madhab` and `method` in configuration ([65ea4f9](65ea4f9190e2dbb8fe5ff47fd8b1505b2ecc5a6a))
- Support windows-pc config location ([69e37d7](69e37d72b7cdbb830440ba03dc10c420309eb982))
- Make config handling more robust ([03bf047](03bf0472c44074b06b11e8a2d27ed6d922bd2625))

## [0.1.1] - 2020-09-25

### Features

- Support json output ([417c24f](417c24f81fedb401e41995ec27437e7fcec4b134))
- Print salah in red if remaining time is low ([2cce260](2cce26049c0b44048939e92d45c66d5099f3d8a1))
- Use color in help ([d20c8cb](d20c8cb5d66dd2e290b0e9443f1e1488c8ee1d95))

### Bug fixes

- Don't supply any key if it's normal ([922967a](922967a1f163e184a8b549697d31e227e5f8ae02))
- Don't hard-code clap version ([abe2a8f](abe2a8f05d53218031c0d64d5cb4a829a2974cca))

## [0.1.0] - 2020-09-25

### Features

- Show help if no argument supplied ([9b545c8](9b545c889c0f76f0a4e007caf4914340ba016732))
- Support displaying next salah ([87935ab](87935ab2db72d104d98ad2468a81432e2c9057df))
- Support displaying current salah ([4f7f87b](4f7f87b99f059e7dff40b7883262fe1f9ae81ee1))

### Bug fixes

- Use shorter output ([c07db76](c07db7664a9d92cec67c377cdb01d781931d9a40))
- Display remaining time instead of exact time of current salah ([5a34e1b](5a34e1bbfc337cf35cafbc197deb0d8ffac02c50))
