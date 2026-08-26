# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/).

## [0.0.5](https://github.com/rvben/ext4-cli/compare/v0.0.4...v0.0.5) - 2026-08-26

### Added

- **packaging**: add package-named launcher ([3a12760](https://github.com/rvben/ext4-cli/commit/3a12760b9ddaa8f72fa2d40b7827b3e75602ceb6))

### Fixed

- **ci**: install pinned Rust components ([64427e4](https://github.com/rvben/ext4-cli/commit/64427e437efb681ee3a96f089fefb792e9ae88a0))

## [0.0.3](https://github.com/rvben/ext4-cli/compare/v0.0.2...v0.0.3) - 2026-06-20

### Fixed

- **schema**: remove conflict error kind not emitted by the binary ([1105e6e](https://github.com/rvben/ext4-cli/commit/1105e6e2f3ca11580956847feec6e3a341f01d2b))

## [0.0.2](https://github.com/rvben/ext4-cli/compare/v0.0.1...v0.0.2) - 2026-06-11

### Added

- add clispec v0.2 compliance (24/24 score) ([810bb87](https://github.com/rvben/ext4-cli/commit/810bb87f26c9091bda70acd862b62b5d153369d3))
- add PyPI distribution via maturin ([6909fce](https://github.com/rvben/ext4-cli/commit/6909fcef7a0f00d39a50c9c3cbb4dfe6f1979c76))

### Fixed

- **ci**: use --check-url for uv publish to skip already-uploaded files ([ed9e19a](https://github.com/rvben/ext4-cli/commit/ed9e19a3c90afda0c272234cbe12f22612185bd6))
- **ci**: correct wheel artifact path and add --skip-existing for PyPI publish ([9ec39c6](https://github.com/rvben/ext4-cli/commit/9ec39c62e618178f1dd855ef4114befef2c94f8b))

## [0.0.1] - 2026-04-13

### Added

- **cp**: extract files and directories from ext4 filesystem ([6f1c1a0](https://github.com/rvben/ext4-cli/commit/6f1c1a0a0b77d4e573ce73323317912c6fa656fe))
- **cat**: stream file contents to stdout ([7331180](https://github.com/rvben/ext4-cli/commit/7331180d30af9ab8039cb6d53fc02bb239cd5405))
- **stat**: show inode metadata for files and directories ([544b10e](https://github.com/rvben/ext4-cli/commit/544b10e3c48e8872651d9601ae43f912cf84e602))
- **ls**: list ext4 directory contents with JSON support ([9a65503](https://github.com/rvben/ext4-cli/commit/9a655032fe0963f3e54243b2ae4bedb5b920d3d4))
- **info**: show filesystem info via superblock parsing ([b420404](https://github.com/rvben/ext4-cli/commit/b4204041844b7b690ad8310cdc5495a8b705df7b))
- **output**: mode formatting and JSON printer ([33a2497](https://github.com/rvben/ext4-cli/commit/33a24979f7ccd4d849b6b2721904da1c60c8ff78))
- **source**: open ext4 image files and block devices ([6913ee9](https://github.com/rvben/ext4-cli/commit/6913ee9586c93ef81c6221fe316be6bb70884b88))

### Fixed

- **tests**: remove redundant serde_json import ([e41fb6e](https://github.com/rvben/ext4-cli/commit/e41fb6e7166472d26b1694a5ee3204031c6341e6))
- **main**: collapse nested if-let for permission denied check ([a89b255](https://github.com/rvben/ext4-cli/commit/a89b2556cf5cf508b68b2707c6d702fb4918ff0f))
- **source**: use sector-aligned reads for raw block device compatibility ([7183d50](https://github.com/rvben/ext4-cli/commit/7183d50dc3f29a0f02d4db253cf68899fbff99c7))
- **ls**: classify all FileType variants correctly, add --all flag tests ([607c742](https://github.com/rvben/ext4-cli/commit/607c7429801d4410e028300cf448bc5361274bad))
- **info**: correct metadata_csum bitmask, add gdt_csum/dir_nlink/extra_isize, safe block_size shift ([4d3b73d](https://github.com/rvben/ext4-cli/commit/4d3b73d4deb69e6c236ebe0e05c2e5a2fb071291))
- **fixtures**: clean up partial images on failure ([9375a46](https://github.com/rvben/ext4-cli/commit/9375a465d8b14669662c177af03594f0941eca85))
