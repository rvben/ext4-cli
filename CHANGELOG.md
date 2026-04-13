# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/).

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
