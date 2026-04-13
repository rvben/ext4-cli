use anyhow::{anyhow, bail, Result};
use serde::Serialize;
use std::io::{Read, Seek, SeekFrom};

use crate::output;

const SUPERBLOCK_OFFSET: u64 = 1024;
const SUPERBLOCK_MAGIC: u16 = 0xEF53;

fn read_u16_le(buf: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([buf[offset], buf[offset + 1]])
}

fn read_u32_le(buf: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap())
}

fn parse_uuid(bytes: &[u8]) -> String {
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5],
        bytes[6], bytes[7],
        bytes[8], bytes[9],
        bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
    )
}

fn parse_label(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}

fn parse_features(compat: u32, incompat: u32, ro_compat: u32) -> Vec<&'static str> {
    let mut features = Vec::new();
    if compat & 0x0004 != 0 { features.push("has_journal"); }
    if compat & 0x0008 != 0 { features.push("ext_attr"); }
    if compat & 0x0010 != 0 { features.push("resize_inode"); }
    if compat & 0x0020 != 0 { features.push("dir_index"); }
    if incompat & 0x0002 != 0 { features.push("filetype"); }
    if incompat & 0x0040 != 0 { features.push("extents"); }
    if incompat & 0x0080 != 0 { features.push("64bit"); }
    if incompat & 0x0200 != 0 { features.push("flex_bg"); }
    if incompat & 0x10000 != 0 { features.push("encrypt"); }
    if ro_compat & 0x0001 != 0 { features.push("sparse_super"); }
    if ro_compat & 0x0002 != 0 { features.push("large_file"); }
    if ro_compat & 0x0004 != 0 { features.push("btree_dir"); }
    if ro_compat & 0x0008 != 0 { features.push("huge_file"); }
    if ro_compat & 0x0010 != 0 { features.push("gdt_csum"); }
    if ro_compat & 0x0020 != 0 { features.push("dir_nlink"); }
    if ro_compat & 0x0040 != 0 { features.push("extra_isize"); }
    if ro_compat & 0x0400 != 0 { features.push("metadata_csum"); }
    features
}

#[derive(Serialize)]
struct FsInfo {
    uuid: String,
    label: String,
    block_size: u32,
    inodes_count: u32,
    free_inodes_count: u32,
    blocks_count: u32,
    free_blocks_count: u32,
    features: Vec<&'static str>,
}

pub fn run_info(source_path: &str, json: bool) -> Result<()> {
    let mut file = std::fs::File::open(source_path)?;
    file.seek(SeekFrom::Start(SUPERBLOCK_OFFSET))?;

    let mut buf = [0u8; 1024];
    file.read_exact(&mut buf)?;

    let magic = read_u16_le(&buf, 0x38);
    if magic != SUPERBLOCK_MAGIC {
        bail!(
            "not a valid ext4 filesystem (magic: {:#06x}, expected {:#06x})",
            magic,
            SUPERBLOCK_MAGIC
        );
    }

    let log_block_size = read_u32_le(&buf, 0x18);
    let block_size = 1024u32
        .checked_shl(log_block_size)
        .ok_or_else(|| anyhow!("invalid s_log_block_size: {log_block_size}"))?;

    let info = FsInfo {
        uuid: parse_uuid(&buf[0x68..0x78]),
        label: parse_label(&buf[0x78..0x88]),
        block_size,
        inodes_count: read_u32_le(&buf, 0x00),
        free_inodes_count: read_u32_le(&buf, 0x10),
        blocks_count: read_u32_le(&buf, 0x04),
        free_blocks_count: read_u32_le(&buf, 0x0C),
        features: parse_features(
            read_u32_le(&buf, 0x5C),
            read_u32_le(&buf, 0x60),
            read_u32_le(&buf, 0x64),
        ),
    };

    if json {
        output::print_json(&info);
    } else {
        println!("uuid:              {}", info.uuid);
        println!(
            "label:             {}",
            if info.label.is_empty() { "(none)" } else { &info.label }
        );
        println!("block_size:        {}", info.block_size);
        println!("inodes_count:      {}", info.inodes_count);
        println!("free_inodes_count: {}", info.free_inodes_count);
        println!("blocks_count:      {}", info.blocks_count);
        println!("free_blocks_count: {}", info.free_blocks_count);
        println!("features:          {}", info.features.join(", "));
    }

    Ok(())
}
