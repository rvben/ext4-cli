use anyhow::{Context, Result};
use ext4_view::{Ext4, Ext4Read};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

/// Sector size assumed for raw block device alignment on macOS.
const SECTOR_SIZE: u64 = 512;

/// Wraps a `File` and ensures every read is sector-aligned in both
/// offset and length. Required for raw block devices on macOS
/// (`/dev/rdisk*`), which reject reads that are not multiples of the
/// device's physical sector size.
struct AlignedReader {
    file: File,
}

impl Ext4Read for AlignedReader {
    fn read(
        &mut self,
        start_byte: u64,
        dst: &mut [u8],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        if dst.is_empty() {
            return Ok(());
        }
        let aligned_start = (start_byte / SECTOR_SIZE) * SECTOR_SIZE;
        let lead = (start_byte - aligned_start) as usize;
        let total = lead + dst.len();
        let aligned_len = total.div_ceil(SECTOR_SIZE as usize) * SECTOR_SIZE as usize;

        let mut buf = vec![0u8; aligned_len];
        self.file
            .seek(SeekFrom::Start(aligned_start))
            .map_err(Box::new)?;
        self.file.read_exact(&mut buf).map_err(Box::new)?;
        dst.copy_from_slice(&buf[lead..lead + dst.len()]);
        Ok(())
    }
}

pub fn open_source(path: &str) -> Result<Ext4> {
    let file = File::open(path).with_context(|| format!("failed to open '{path}'"))?;
    Ext4::load(Box::new(AlignedReader { file }))
        .with_context(|| format!("failed to read ext4 filesystem from '{path}'"))
}
