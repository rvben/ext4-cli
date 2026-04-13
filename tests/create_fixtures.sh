#!/usr/bin/env bash
set -euo pipefail

# Locate mkfs.ext4 (e2fsprogs is keg-only on macOS)
if command -v mkfs.ext4 &>/dev/null; then
    MKFS=mkfs.ext4
elif [ -f "/opt/homebrew/opt/e2fsprogs/sbin/mkfs.ext4" ]; then
    MKFS="/opt/homebrew/opt/e2fsprogs/sbin/mkfs.ext4"
elif [ -f "/usr/local/opt/e2fsprogs/sbin/mkfs.ext4" ]; then
    MKFS="/usr/local/opt/e2fsprogs/sbin/mkfs.ext4"
else
    echo "mkfs.ext4 not found. Install with: brew install e2fsprogs" >&2
    exit 1
fi

DIR="$(cd "$(dirname "$0")/fixtures" && pwd)"
mkdir -p "$DIR"

STAGING=$(mktemp -d)
cleanup() {
    local rc=$?
    rm -rf "$STAGING"
    if [ $rc -ne 0 ]; then
        rm -f "$DIR/minimal.img" "$DIR/rich.img"
    fi
}
trap cleanup EXIT

# minimal.img: 4 MB, nearly empty
dd if=/dev/zero of="$DIR/minimal.img" bs=1M count=4 2>/dev/null
$MKFS -F -q -L "minimal" "$DIR/minimal.img"

# rich.img: 8 MB with a directory tree

mkdir -p "$STAGING/etc"
mkdir -p "$STAGING/home/alice"
mkdir -p "$STAGING/var/log"
printf "UUID=deadbeef-0000 / ext4 defaults 0 1\n" > "$STAGING/etc/fstab"
printf "root:x:0:0:root:/root:/bin/bash\nalice:x:1000:1000::/home/alice:/bin/sh\n" > "$STAGING/etc/passwd"
printf "hello world\n" > "$STAGING/home/alice/hello.txt"
printf "log entry 1\nlog entry 2\n" > "$STAGING/var/log/syslog"

dd if=/dev/zero of="$DIR/rich.img" bs=1M count=8 2>/dev/null
$MKFS -F -q -L "rich" -d "$STAGING" "$DIR/rich.img"

echo "Fixtures created in $DIR:"
ls -lh "$DIR"
