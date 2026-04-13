# ext4-cli

Read ext4 filesystems from image files and block devices — without mounting them.

## Quick start

```bash
# Install (pick one)
cargo install ext4-cli          # From crates.io
pip install ext4-cli            # Via pip
uvx ext4-cli --help             # Run without installing (via uv)

# Use
ext4 --source disk.img ls /etc
ext4 --source disk.img cat /etc/fstab
ext4 --source disk.img cp /etc /tmp/etc-backup -r
```

## Installation

### From crates.io

```bash
cargo install ext4-cli
```

### From PyPI

```bash
pip install ext4-cli
# or run without installing:
uvx ext4-cli ls /
```

### From Homebrew

```bash
brew install rvben/tap/ext4
```

### From GitHub releases

Pre-built binaries for Linux (x64, arm64) and macOS (x64, arm64) on the [releases page](https://github.com/rvben/ext4-cli/releases).

## Usage

All commands require a source — an ext4 image file or raw block device — via `--source` or the `EXT4_SOURCE` environment variable:

```bash
export EXT4_SOURCE=/dev/rdisk4s2
ext4 ls /
```

### Commands

#### `ls` — list directory contents

```bash
ext4 --source disk.img ls                     # List root directory
ext4 --source disk.img ls /etc                # List /etc
ext4 --source disk.img ls -l /etc             # Long format (permissions, uid, gid, size)
ext4 --source disk.img ls -a /etc             # Include dotfiles
ext4 --source disk.img ls --json /etc         # JSON output
```

#### `cat` — print file contents

```bash
ext4 --source disk.img cat /etc/fstab
ext4 --source disk.img cat /etc/passwd
```

#### `cp` — extract files

```bash
ext4 --source disk.img cp /etc/fstab ./fstab          # Extract a file
ext4 --source disk.img cp /etc /tmp/etc-backup -r     # Extract a directory tree
```

#### `stat` — show file metadata

```bash
ext4 --source disk.img stat /etc/fstab
ext4 --source disk.img stat --json /etc/fstab
```

#### `info` — show filesystem metadata

```bash
ext4 --source disk.img info
ext4 --source disk.img info --json
```

## Raw block devices (macOS)

On macOS, use the raw disk path (`/dev/rdisk*`) for better performance. Reading raw block devices requires root access:

```bash
sudo ext4 --source /dev/rdisk4s2 ls /
```

## Exit codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Permission denied |
| 3 | Path not found |

## Development

```bash
make fixtures   # Generate test fixtures (requires e2fsprogs)
make test       # Run tests
make lint       # Run fmt + clippy
make install    # Build and install
```

## License

MIT OR Apache-2.0
