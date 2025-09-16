# Changelog

All notable changes to this project will be documented in this file.

## [1.11.0] - 2025-09-16

### 🚀 Features

- Support `id` locale ([d784565](https://github.com/azzamsa/bilal/commit/d7845652ac0979638525e46aa7f2f6f7e7abf254))

  This makes prayer time labels more familiar
  for Indonesian-speaking users.

### 💼 Other

- Simplify just recipe ([ab4970d](https://github.com/azzamsa/bilal/commit/ab4970d90388743980cb87bbfe985369f20228cb))

  Some of them not needed at all.

- Upgrade dependencies ([f325298](https://github.com/azzamsa/bilal/commit/f325298488ea31dfd046d37f6b5c8475fc2e38a2))

### 📚 Documentation

- Update readme ([075b318](https://github.com/azzamsa/bilal/commit/075b318f51d4f1a56c8b90e0cdcd0ddd8916083f))
- Reformat ([d9dab44](https://github.com/azzamsa/bilal/commit/d9dab44b710ed8da85ecbb07060b7a146760a12f))

### ⚙️ Miscellaneous Tasks

- Put `install-action` definition in one place ([dba2a06](https://github.com/azzamsa/bilal/commit/dba2a06eec59319ae91e8a8bdeb6a29b81931ef8))
- Update git-cliff config ([9747dd0](https://github.com/azzamsa/bilal/commit/9747dd07182654d2184774ea1413c5a0ee9d7fe0))

## [1.10.1] - 2025-06-09

### 🐛 Bug Fixes

- Upstream `islam` renamed to `misykat` ([cf5357d](https://github.com/azzamsa/bilal/commit/cf5357d9dc4d5e5ef1652db6f7c72092f1be83af))

### 💼 Other

- Upgrade dependencies ([4e27b71](https://github.com/azzamsa/bilal/commit/4e27b71dd836f1070b9fea0ab808794743fd2977))

### 📚 Documentation

- Align examples ([3bdc6fd](https://github.com/azzamsa/bilal/commit/3bdc6fde855da72cc2658a9543770e194344568f))
- Add macOS security instructions to readme ([7f1cada](https://github.com/azzamsa/bilal/commit/7f1cada47be7dbafe9671baa610591a83f240ce1))

  Add instructions for handling macOS Gatekeeper security warnings when running the application. Using xattr command-line solution to help users bypass the "Apple could not verify" warning.

### ⚙️ Miscellaneous Tasks

- Upgrade runners ([767e5d9](https://github.com/azzamsa/bilal/commit/767e5d939719a3f21f58bfcb57a4bfe53247a64b))

## [1.10.0] - 2025-03-12

### 🚀 Features

- New custom config argument ([21aaccd](https://github.com/azzamsa/bilal/commit/21aaccdfa967489da69124f38db78802dfec8fe0))

### 💼 Other

- Upgrade to latest `islam` ([3bfb81b](https://github.com/azzamsa/bilal/commit/3bfb81b83bcefbe8f65434eaab135b4b67dc85e1))

### ⚙️ Miscellaneous Tasks

- Upgrade runners ([dcfb934](https://github.com/azzamsa/bilal/commit/dcfb934336542e5d969177c9e618da7d00f47462))

## [1.9.2] - 2024-12-13

### 🐛 Bug Fixes

- Round the prayer times to the nearest minute ([024d628](https://github.com/azzamsa/bilal/commit/024d62804d565abbaeaa64339f7703fb98581ce6))

### 💼 Other

- Upgrade dependencies ([ba8ccf2](https://github.com/azzamsa/bilal/commit/ba8ccf2a5eff04568727a5f9e4e914ae338046cf))
- Upgrade the toolchain ([e730aa2](https://github.com/azzamsa/bilal/commit/e730aa2dbef5ca4838f0afd34c2d875d2a433f84))

### ⚙️ Miscellaneous Tasks

- Macos-12 is deprecated ([a865ad7](https://github.com/azzamsa/bilal/commit/a865ad77824c5d977ce288587242b799b4c3d255))

## [1.9.1] - 2024-08-10

### 💼 Other

- Upgrade dependencies ([65c6f34](https://github.com/azzamsa/bilal/commit/65c6f34c8d24460b29dcc45317bc40aee8bcab4f))

### ⚙️ Miscellaneous Tasks

- Set timezone before running the tests on CI ([699be53](https://github.com/azzamsa/bilal/commit/699be5326e2b6b6566dba196f431f7b42f6db89a))

## [1.9.0] - 2024-08-07

### 🚀 Features

- Upgrade to islam v4 ([67af05b](https://github.com/azzamsa/bilal/commit/67af05b4170ad2bea094926f0dc93b610d66ca99))

### 💼 Other

- Upgrade dependencies ([78cf7d6](https://github.com/azzamsa/bilal/commit/78cf7d68bd67dc166f1d453ad05a5b17febabb87))
- Adopt `just` new features ([5d2b31e](https://github.com/azzamsa/bilal/commit/5d2b31e9d9523c8c42dd75c5dfd3ff7cb8effa98))

  It simplifies my recipes.

- Put formatter and linter configs in the root directory ([97b8e5c](https://github.com/azzamsa/bilal/commit/97b8e5ccbca68b0f3bc02435a6cf20b3f1eb89e6))

  Otherwise, we would have to deal with a custom `--config-path` for every possible formatter on Earth.
  It's okay to let them clutter the directory (check out other popular repos!).
  Even if they are visible on the GitHub repo, locally they are hidden by your favorite editor.

- Lint everything ([ff0d459](https://github.com/azzamsa/bilal/commit/ff0d459ae4c25a988b72037f4a074cca6e6de48d))

### 🚜 Refactor

- Remove unused imports ([705322c](https://github.com/azzamsa/bilal/commit/705322cb53a83a351e7bba78bfbe7fc72a7089b0))
- Remove unused files ([f072a81](https://github.com/azzamsa/bilal/commit/f072a811f60f4f91b28fe9ccdf416016a1d8548c))

### 📚 Documentation

- Explain that the app "works completely offline" ([076ab8c](https://github.com/azzamsa/bilal/commit/076ab8c7b5b3381d3f8e83024f98007fab212c5b))
- Fix outdated CI badge link ([6259406](https://github.com/azzamsa/bilal/commit/625940624318416dba8dd56f93000f1dd11b0d72))
- Fix invalid config file name ([9a63940](https://github.com/azzamsa/bilal/commit/9a63940db22d68a2365a99c9a6640cc35f96f4b2))

### 🧪 Testing

- Close temp directories after usage ([1a14652](https://github.com/azzamsa/bilal/commit/1a1465277ff60991a6132393148cf94e95a4dd0d))
- Add time format tests ([4b01c8e](https://github.com/azzamsa/bilal/commit/4b01c8e699d044d572da4bc24c70beb1aed787ac))

## [1.8.0] - 2024-06-18

### 🚀 Features

- Support custom config location ([46428b4](https://github.com/azzamsa/bilal/commit/46428b4ce63b61b3bd2031e5b9ce008bb781f69c))

  Don't use real fixture file in integration tests.

- Reject invalid time format ([eafe9d9](https://github.com/azzamsa/bilal/commit/eafe9d941866249d141a383a54769ec7ad06b6ca))

  To avoid confusion.

- Time format feature ([9e151c3](https://github.com/azzamsa/bilal/commit/9e151c38f70c0256c92ae4e9a1d92a90f2683241))

  Add a setting to choose between 24 Hour of 12 Hour format.

### 🐛 Bug Fixes

- Capitalize the time format ([33458bb](https://github.com/azzamsa/bilal/commit/33458bbd9b48353c08b8780f6705a700c93105ce))

### 💼 Other

- Put formatter and linter configs in the root directory ([7dd8405](https://github.com/azzamsa/bilal/commit/7dd84053efe34c970414b5a44053a107f37a4487))

  Otherwise, we would have to deal with a custom `--config-path` for every possible formatter on Earth.
  It's okay to let them clutter the directory (check out other popular repos!).
  Even if they are visible on the GitHub repo, locally they are hidden by your favorite editor.

- Upgrade `miette` ([a57de91](https://github.com/azzamsa/bilal/commit/a57de9199b448d46bda976ca55016fe0f5553e3e))
- Upgrade dependencies ([b9f0a70](https://github.com/azzamsa/bilal/commit/b9f0a7046d0d670f5443eadcf144732ad73b291b))
- Upgrade msrv ([fb39123](https://github.com/azzamsa/bilal/commit/fb391236bcecc5e739c6f76f5d6fa794d5cd7b35))
- Upgrade dependencies ([52ad6b3](https://github.com/azzamsa/bilal/commit/52ad6b3c762f86631a8f800f2a9fb9d5141c78df))
- Upgrade MSRV ([90fb61b](https://github.com/azzamsa/bilal/commit/90fb61bb9acfd1078c9eb1bf816f518f771e3aa9))
- Upgrade dependencies ([8867e02](https://github.com/azzamsa/bilal/commit/8867e0216d314ef1da29f50b1af48f4a6de4cfee))
- Bacon is better than `cargo-watch` ([8457421](https://github.com/azzamsa/bilal/commit/84574218129f767afca1a970b5b816eb47ee1430))

  - Shows errors before warnings
    - Keep the list updated without flickering when the code changes. I don't need
      `spacer` anymore
    - Has scroll indicator
    - Doesn't have issue with `Permission denied (os error 13)`

- Use `rust-version` ([5b0b229](https://github.com/azzamsa/bilal/commit/5b0b22977f5de1521dd23e1c32ac60a6957a1ace))

### 🚜 Refactor

- Avoid deep clone ([a59cf3e](https://github.com/azzamsa/bilal/commit/a59cf3e66d3422893dd37593761ac2b3e60db0ad))
- Simplify color logic ([8a8f1a9](https://github.com/azzamsa/bilal/commit/8a8f1a98c9570feea398b3e7bd7e7d619683a54f))
- Avoid comparing string ([06cf2ea](https://github.com/azzamsa/bilal/commit/06cf2eacc1b92c2f1e25c0dea904933e528d1423))
- Fix typos with `typos-cli` ([c984320](https://github.com/azzamsa/bilal/commit/c984320bec5c414f6afcf1515e988473d170d380))

  I never knew I had many typos until I tried typos-cli by Ed Page.
  All of them are valid typos; there are no false positives.
  I will definitely embrace it in all my codebases.

### 📚 Documentation

- Update ([2988808](https://github.com/azzamsa/bilal/commit/29888084dc3687ebb63851467db40c4cdf49d0e3))
- Enable github sponsor ([44fd5b1](https://github.com/azzamsa/bilal/commit/44fd5b18deef90d7e3a996406e923301b0059d05))

### ⚙️ Miscellaneous Tasks

- Update action dependencies ([fc651bd](https://github.com/azzamsa/bilal/commit/fc651bdc4a4cb004cef9e59973eeb1cfafa63516))
- `actions-rust-lang/setup-rust-toolchain` is better ([67fa2b6](https://github.com/azzamsa/bilal/commit/67fa2b6d1ae883783657e77835dadd0ee8177667))

  - It uses `rust-toolchain.toml` by default
    - Has built-in support for `Swatinem/rust-cache

## [1.7.0] - 2023-09-03

### 🚀 Features

- Upgrade islam to `v3.2` ([0183896](https://github.com/azzamsa/bilal/commit/0183896198b3c1fcec9ac00515c2e013b72b542b))

## [1.6.0] - 2023-09-01

### 🚀 Features

- Upgrade islam to `v3.1` ([ca6665d](https://github.com/azzamsa/bilal/commit/ca6665d8409be28bd1ed2ac025e244dbce363066))

## [1.5.1] - 2023-08-30

### 🐛 Bug Fixes

- Wrong app name in release step ([3a4f2fe](https://github.com/azzamsa/bilal/commit/3a4f2fe832033c8089458656d3c9a872a13336b2))

## [1.5.0] - 2023-08-30

### 🚀 Features

- Use clearer terms to indicate the remaining time ([9ba35c5](https://github.com/azzamsa/bilal/commit/9ba35c552ef1dccc4c0f8ee5cc66289a7633c5ff))

## [1.4.0] - 2023-08-30

### 🐛 Bug Fixes

- Upgrade to islam v3 ([d45b8c2](https://github.com/azzamsa/bilal/commit/d45b8c2a65469f7f40fcb9243d140043ad00992e))

### 💼 Other

- Upgrade dependencies ([75993c9](https://github.com/azzamsa/bilal/commit/75993c9632c5b35e874b48dd72496f61af11ecb4))

### 🚜 Refactor

- New workflow ([7b4ef4e](https://github.com/azzamsa/bilal/commit/7b4ef4e75dccf83661aa66a0d81bf554dc797ad9))

## [1.3.0] - 2023-06-22

### 🐛 Bug Fixes

- Upgrade to islam v2.0.0 ([eb780eb](https://github.com/azzamsa/bilal/commit/eb780eb860525c7896475f74ae28c68f3603d306))

### 💼 Other

- Upgrade dependencies ([b65d2eb](https://github.com/azzamsa/bilal/commit/b65d2eb87db33faff082c5d7cc98ff0b4244c169))
- Upgrade toml ([2efc2c0](https://github.com/azzamsa/bilal/commit/2efc2c0927a93233000a546c123472531dcda1fc))
- New approach ([a168547](https://github.com/azzamsa/bilal/commit/a1685478636fbef2c73d9a13fd521ced7074f3ed))

## [1.2.0] - 2023-01-22

### 🐛 Bug Fixes

- Some API changes in islam v1.0 ([735b442](https://github.com/azzamsa/bilal/commit/735b442c616402b7209068315cb1ffd506f81e20))

### 📚 Documentation

- List more feature ([97c13ba](https://github.com/azzamsa/bilal/commit/97c13ba31fc65d421965e44a9c4c6a5cce1d4c75))
- Update copyrights year. ([e3c7c5d](https://github.com/azzamsa/bilal/commit/e3c7c5df6ff78754cc5a67dc44ea56180f44af33))

### ⚙️ Miscellaneous Tasks

- `build` matrix is not needed ([b50545d](https://github.com/azzamsa/bilal/commit/b50545d55e9c0eb2e23c266950e6b416c166edc2))
- Remove debug step ([1518a99](https://github.com/azzamsa/bilal/commit/1518a996722bb3623ecbea16723773d496b5d68b))

## [1.1.0] - 2023-01-03

### 🚀 Features

- Print config snippets of error location ([734fab4](https://github.com/azzamsa/bilal/commit/734fab40a8e43e2e4a41d50e156eeb49a30a5575))

### 💼 Other

- Support binstall ([dc0535c](https://github.com/azzamsa/bilal/commit/dc0535c007728408a95d945b52dacfc5ac5c207b))
- Remove unused `git-cliff` arg ([7c72c32](https://github.com/azzamsa/bilal/commit/7c72c3208d4e9b966ceaf44271b6e6dca62dd617))

### 🚜 Refactor

- Capitalize error message ([5fd38e6](https://github.com/azzamsa/bilal/commit/5fd38e6c16e9e07beb6864a0982fafac514d0fd3))

### ⚙️ Miscellaneous Tasks

- Better workflow ([0aeb463](https://github.com/azzamsa/bilal/commit/0aeb4634164d30f1e4c4a4a2161e7e061b1d3f18))

## [1.0.0] - 2022-12-28

### 💼 Other

- Add git hooks ([8fdf712](https://github.com/azzamsa/bilal/commit/8fdf7129b6421e044ec7cc4d9307759fc4b366b7))
- `content` parser is not used here ([ee037ac](https://github.com/azzamsa/bilal/commit/ee037acb365e1f4dd1306847c636c10c7d97ad79))

### 🚜 Refactor

- Remove unused `derive` traits ([483d6fa](https://github.com/azzamsa/bilal/commit/483d6fac921f83eb5bf938bdb0f2183ba2728481))

### 📚 Documentation

- Update readme ([c392234](https://github.com/azzamsa/bilal/commit/c39223422dce639f0eed75d966c756fe45714c2c))

### ⚡ Performance

- Migrate to `owo-color` ([6777ba9](https://github.com/azzamsa/bilal/commit/6777ba9759a559884022163565a2ee19242dec3c))

  It has fewer dependencies

### 🧪 Testing

- Don't hardcode crate name ([d0ffc2b](https://github.com/azzamsa/bilal/commit/d0ffc2b9f986bf8849049138d5fe51c3e990fda0))

## [0.2.1] - 2022-12-22

### 🚀 Features

- Improve error message ([d9f0296](https://github.com/azzamsa/bilal/commit/d9f0296961bd6aab042f869d59734acde49982bb))

  Improve error message for `Invalid Config`, and `Config not found`.

### 🐛 Bug Fixes

- Avoid checking if the config exists ([f931099](https://github.com/azzamsa/bilal/commit/f9310994465a985801d0cb1aa3076446f7cac819))

  This eliminates a [race condition between "check to see if file exists" and "open file if it exists"](https://en.wikipedia.org/wiki/Time-of-check_to_time-of-use).

### 🚜 Refactor

- Error types ([498ea25](https://github.com/azzamsa/bilal/commit/498ea25944fc375196469c64301a616302428a06))

### 📚 Documentation

- Add commit message format ([973ce43](https://github.com/azzamsa/bilal/commit/973ce43b53b5b8cf8577a2f0e6bf442232cadec6))

### ⚙️ Miscellaneous Tasks

- Simplify release ([8a49e5f](https://github.com/azzamsa/bilal/commit/8a49e5f9ddfc80d18848d980e26e072f6b39a2ba))
- `dprint` not found ([4ae0bd9](https://github.com/azzamsa/bilal/commit/4ae0bd9f4638a0c271ab0058fcbce3f81e6eea71))
- Only install `cross` at specific requirement ([470f5dc](https://github.com/azzamsa/bilal/commit/470f5dc01fa307bf12b29088b4e458695964a0a0))
- Update release changelog manually ([7a7d817](https://github.com/azzamsa/bilal/commit/7a7d817e934a177929fd52919a4c20484662fa81))

  Using CI to do this task is error prone.

## [0.2.0] - 2022-12-19

### 🐛 Bug Fixes

- Upgrade `islam` dependency ([d51db32](https://github.com/azzamsa/bilal/commit/d51db32cab86c23e9e8f6d5725cfb81446dbff6e))

### 💼 Other

- Update dependencies ([d3073a8](https://github.com/azzamsa/bilal/commit/d3073a8fe2772c3ff966bd1677f2428b9184fd45))

### 🚜 Refactor

- Upgrade `clap` ([5dc2047](https://github.com/azzamsa/bilal/commit/5dc204756f1ae874ea08475a859bc34d38122edd))
- Update ([d38e6b5](https://github.com/azzamsa/bilal/commit/d38e6b59ed04318dfd2188b6a145bed67e3a4e7e))

### 📚 Documentation

- Update old documentation ([02fa930](https://github.com/azzamsa/bilal/commit/02fa930941d333aeeeecc8535fe9581e0ec7d99d))

### ⚙️ Miscellaneous Tasks

- Update workflows ([13e9a7d](https://github.com/azzamsa/bilal/commit/13e9a7d648bcbf0adee2551fb81c887a07147451))

## [0.1.9] - 2021-11-14

### 🚜 Refactor

- Port to clap v2 ([a67d472](https://github.com/azzamsa/bilal/commit/a67d472c8995e6d16dc1aa9b3d60aac4302027a8))

  The v3 API is always changing.

- Clippy ([b84f8e1](https://github.com/azzamsa/bilal/commit/b84f8e1d46d397503cf77d54efe200b8d9d5f70e))

### 📚 Documentation

- Add missing link in public inbox ([3d1201f](https://github.com/azzamsa/bilal/commit/3d1201f030dd846bcd7e7a89ddecc90a59bf674b))
- Update contributing ([5756572](https://github.com/azzamsa/bilal/commit/57565721a019c65662fdb38578fd77c13220bda9))

### ⚙️ Miscellaneous Tasks

- Fix failing test in Windows ([37df260](https://github.com/azzamsa/bilal/commit/37df2600e32e87b14010ca87f71c583cfcb76a97))
- Fix failure in GitHub action ([4bbdbac](https://github.com/azzamsa/bilal/commit/4bbdbac289f944428daf4cc0a98244deebf1152a))
- Migrate to GitHub ([54bf4d3](https://github.com/azzamsa/bilal/commit/54bf4d3f1f4bb485333ae094e1f9ce49e702796c))

## [0.1.8] - 2021-04-22

### 🐛 Bug Fixes

- Wrong critical state ([3194a15](https://github.com/azzamsa/bilal/commit/3194a1567c5f6ec531243eace6bc2e24026fe3f3))

  The app only check for the remaining time.
  So it will says `critical` even the hour > 0.

- Leading zero in minute not shown ([d391811](https://github.com/azzamsa/bilal/commit/d3918115f6b7e91241c5c344362d91d08dce8cc0))

  Use `strftime` instead of plain `string format`.

- Wrong remaining time ([8831a8b](https://github.com/azzamsa/bilal/commit/8831a8b70e1a7d0122dc226cb9d4244543172dd1))

  The hour doesn't shown even if it's `not equal to 0`.

### 📚 Documentation

- Update demo ([8e468be](https://github.com/azzamsa/bilal/commit/8e468beb831234f17a314f70dab322a5ecf3ed4a))

  Cut the end of gif

## [0.1.7] - 2021-04-20

### 🐛 Bug Fixes

- Remove `chrono` dependency ([f608c49](https://github.com/azzamsa/bilal/commit/f608c495dddfaea12b73771a540ecd139a91a99f))

  Now it's included as module in `islam`

- Salah now uses shorter module name ([b2d8c0f](https://github.com/azzamsa/bilal/commit/b2d8c0f624c45725594b87edc35cfd4dc763fa09))
- State must be present ([37d6f21](https://github.com/azzamsa/bilal/commit/37d6f21717a531ce2e7c6b8a636234f7b7a25235))
- Use salah as an argument instead of an option ([44c107a](https://github.com/azzamsa/bilal/commit/44c107a2b421914231c06b8d45cd7da62b42b0b6))

  Using salah as an argument is somewhat wrong

### 💼 Other

- Remove unused dependencies ([c7c8f40](https://github.com/azzamsa/bilal/commit/c7c8f409df7748c076f5b1894c6480fb37ddf41b))

### 🚜 Refactor

- Clippy ([3a89fcb](https://github.com/azzamsa/bilal/commit/3a89fcb9e956794098d364f603e57e51c794f347))
- Don't use glob import for `chrono` ([74519d2](https://github.com/azzamsa/bilal/commit/74519d23832240e5422cc73435a025fc9c17e0f1))

  Just for readability.

- Refactor ([397f8de](https://github.com/azzamsa/bilal/commit/397f8dec4d9dfdcffe53bb4a467dc06596171617))

### 📚 Documentation

- Add development installation ([d3b33eb](https://github.com/azzamsa/bilal/commit/d3b33eb90062cadc295b5a86a91b38408c3e155f))

## [0.1.6] - 2021-04-19

### 💼 Other

- Binary needs Cargo.lock ([559ca4a](https://github.com/azzamsa/bilal/commit/559ca4adbc1543541a7686385d52f78557ff3da4))

### 🚜 Refactor

- Migrate to our own islam library ([a620e35](https://github.com/azzamsa/bilal/commit/a620e35bbb40cf2605390e3145202c2af3f2c084))

  I have difficulties working with `insha/salah`.
  `abougouffa/pyIslam` instead more precise and has simpler algorithm.

### 📚 Documentation

- Add release steps ([4fa3c02](https://github.com/azzamsa/bilal/commit/4fa3c02d4802b505548c73f0dae7370ac0496e92))

## [0.1.5] - 2020-09-30

### 💼 Other

- Bump version to v0.1.5 ([df9d754](https://github.com/azzamsa/bilal/commit/df9d7546eba30446b12bd0b7254eedd762a2e990))
- Add warning to update related release file ([c4d38a8](https://github.com/azzamsa/bilal/commit/c4d38a8286b953ae5bf43fda19a03eac333eb3c7))

### 📚 Documentation

- Update changelog ([f8d48cb](https://github.com/azzamsa/bilal/commit/f8d48cb0df6ddf26ba81c47e1fea0e925811f98c))

## [0.1.4] - 2020-09-30

### 💼 Other

- Bump version to v0.1.4 ([fe01a2b](https://github.com/azzamsa/bilal/commit/fe01a2b58dde9b7c3b9b82602f4fb32b345c7571))
- Fix wrong file name ([361718a](https://github.com/azzamsa/bilal/commit/361718a1f55237df570ed5ead4b33c4b2879d6e3))
- Use RagibHasin/salah-rs@uptosnuff ([4a58024](https://github.com/azzamsa/bilal/commit/4a5802484186b4331eebb8087d6b53c96f6db291))

### 🚜 Refactor

- Pass only one config to `get_prayers_time` ([fa5b395](https://github.com/azzamsa/bilal/commit/fa5b3952261e9b7a93be7c27210a0435ff86ef85))

### 📚 Documentation

- Add Py3status config example ([9a2d125](https://github.com/azzamsa/bilal/commit/9a2d1258acfae31131e33d3f8e106279a3a89afb))
- Update cotribution how-to ([7ddca24](https://github.com/azzamsa/bilal/commit/7ddca24b348e94a757f3b939c116eaec5271669f))
- Host its own demo ([a7bad60](https://github.com/azzamsa/bilal/commit/a7bad605db006b0724440e2b8eba777820a82669))
- Add windows config location ([bc36717](https://github.com/azzamsa/bilal/commit/bc36717ac38cb08be433ac8a2346b896b3f16b98))
- Update docs ([d09b36e](https://github.com/azzamsa/bilal/commit/d09b36e864c66a56bcd4fc14e821f10c5a8de8b4))

  - add features
    - fix typos
    - rephrase some word

- Update `bilal` description ([c1b86be](https://github.com/azzamsa/bilal/commit/c1b86be27250b892cdff5f992fb60bcf15c21c02))
- Add i3status-rs demo ([178ba99](https://github.com/azzamsa/bilal/commit/178ba99153b5721e16e300bc1d6feb9239144779))

## [0.1.3] - 2020-09-27

### 🚀 Features

- Add icon to make it easier to differentiate ([2c0f620](https://github.com/azzamsa/bilal/commit/2c0f620dd0669cddffaf2958e88e312e086e4655))

### 💼 Other

- Bump version to v0.1.3 ([8e81307](https://github.com/azzamsa/bilal/commit/8e81307077e3ab5b57f1777df8fde82876251bb0))

### 📚 Documentation

- Add more explanation to help arguments ([dc21aee](https://github.com/azzamsa/bilal/commit/dc21aee655aed2732b62be65e0be1c1acf10e36d))

## [0.1.2] - 2020-09-27

### 🚀 Features

- Support `madhab` and `method` in configuration ([65ea4f9](https://github.com/azzamsa/bilal/commit/65ea4f9190e2dbb8fe5ff47fd8b1505b2ecc5a6a))
- Support windows-pc config location ([69e37d7](https://github.com/azzamsa/bilal/commit/69e37d72b7cdbb830440ba03dc10c420309eb982))

  Initial support for windows-pc

- Make config handling more robust ([03bf047](https://github.com/azzamsa/bilal/commit/03bf0472c44074b06b11e8a2d27ed6d922bd2625))

### 💼 Other

- Bump version to v0.1.2 ([44f6163](https://github.com/azzamsa/bilal/commit/44f6163a82acc1f5e493306f3186091b9c205f1b))
- Add release script ([0d37088](https://github.com/azzamsa/bilal/commit/0d37088f56740d7f2ba11c24a676a45f895f54e7))
- Change dependency branch name ([bc0df40](https://github.com/azzamsa/bilal/commit/bc0df40333df50fe825151506f9dd4846c08ba79))
- Change upstream to my own patch ([6fe082e](https://github.com/azzamsa/bilal/commit/6fe082e322461c6028ef66501510e1629f9f51b3))

  Fixes wrong time on fajr and sunrise.

### 🚜 Refactor

- Avoid using implicit import ([8f5e994](https://github.com/azzamsa/bilal/commit/8f5e9940741bd11d7d79fc99d693214c7ce424cd))

  Easier to read/maintenance in the future.

### 📚 Documentation

- Add release doc ([1bda8a3](https://github.com/azzamsa/bilal/commit/1bda8a3418263a4f90da57b0a198dbb9f6793598))
- Add `madhab` and `method` documentation ([3b8c962](https://github.com/azzamsa/bilal/commit/3b8c962d02c329bd5b890ea6ff48b7ff02576b76))

## [0.1.1] - 2020-09-25

### 🚀 Features

- Support json output ([417c24f](https://github.com/azzamsa/bilal/commit/417c24f81fedb401e41995ec27437e7fcec4b134))

  To integrate with i3status-rs

- Print salah in red if remaining time is low ([2cce260](https://github.com/azzamsa/bilal/commit/2cce26049c0b44048939e92d45c66d5099f3d8a1))
- Use color in help ([d20c8cb](https://github.com/azzamsa/bilal/commit/d20c8cb5d66dd2e290b0e9443f1e1488c8ee1d95))

### 🐛 Bug Fixes

- Don't supply any key if it's normal ([922967a](https://github.com/azzamsa/bilal/commit/922967a1f163e184a8b549697d31e227e5f8ae02))

  `Info` still gives a color in status bar.

- Don't hard-code clap version ([abe2a8f](https://github.com/azzamsa/bilal/commit/abe2a8f05d53218031c0d64d5cb4a829a2974cca))

### 💼 Other

- Bump version to v0.1.1 ([0313833](https://github.com/azzamsa/bilal/commit/03138330f6d85293ebc60347676f675a60eeb767))
- Initial build config ([2f65e5c](https://github.com/azzamsa/bilal/commit/2f65e5ce593da92ede5806dc6ef904dfea54f820))

### 🚜 Refactor

- Clean up duplication ([7e0e29f](https://github.com/azzamsa/bilal/commit/7e0e29f1ae2b251131343e41e9ba8eb340adca9f))
- Use `prayer` instead of `salah` in internal code ([08d4956](https://github.com/azzamsa/bilal/commit/08d49565add5c44c8d1d74340fda3680ccd57886))

### 📚 Documentation

- Fix directory name ([ef5692f](https://github.com/azzamsa/bilal/commit/ef5692f2d1fc7a0a459b12f721706a0061512788))
- Initial docs ([d06f89f](https://github.com/azzamsa/bilal/commit/d06f89f2509583fb2f51245be592be57422cb751))
- Update release page ([ce581e5](https://github.com/azzamsa/bilal/commit/ce581e5dd6b87beec6e50dbd265e6bbbd9e69e21))

### 🎨 Styling

- Cargo fmt ([fd7f840](https://github.com/azzamsa/bilal/commit/fd7f8400dfc3aaf842545424e50e4221dc170f3b))

## [0.1.0] - 2020-09-25

### 🚀 Features

- Show help if no argument supplied ([9b545c8](https://github.com/azzamsa/bilal/commit/9b545c889c0f76f0a4e007caf4914340ba016732))
- Support displaying next salah ([87935ab](https://github.com/azzamsa/bilal/commit/87935ab2db72d104d98ad2468a81432e2c9057df))
- Support displaying current salah ([4f7f87b](https://github.com/azzamsa/bilal/commit/4f7f87b99f059e7dff40b7883262fe1f9ae81ee1))

### 🐛 Bug Fixes

- Use shorter output ([c07db76](https://github.com/azzamsa/bilal/commit/c07db7664a9d92cec67c377cdb01d781931d9a40))

  Because mostly it will be used with other tools. Such as i3status-rs.

- Display remaining time instead of exact time of current salah ([5a34e1b](https://github.com/azzamsa/bilal/commit/5a34e1bbfc337cf35cafbc197deb0d8ffac02c50))

### 💼 Other

- Make build artifact smaller ([5f45a85](https://github.com/azzamsa/bilal/commit/5f45a85c64c4325b5e374445d6df68ddba677b13))
- Ingore uninteresting file ([29a6e3e](https://github.com/azzamsa/bilal/commit/29a6e3e1927a139fb51afed8c674a71fe63f1603))

### 📚 Documentation

- Add README ([5532b6a](https://github.com/azzamsa/bilal/commit/5532b6a3858afbd5ab2633a200ed36ba8d6cac29))
- Add LICENSE ([fd559bd](https://github.com/azzamsa/bilal/commit/fd559bd08343a91e2d422f02088aff30b902444a))
