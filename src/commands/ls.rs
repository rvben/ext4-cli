use anyhow::Result;
use ext4_view::{Ext4, FileType};
use serde::Serialize;

use crate::output;

#[derive(Serialize)]
struct LsEntry {
    name: String,
    #[serde(rename = "type")]
    entry_type: String,
    size: u64,
    mode: String,
    uid: u32,
    gid: u32,
}

pub fn run_ls(fs: &Ext4, path: &str, long: bool, all: bool, json: bool) -> Result<()> {
    let entries_iter = fs.read_dir(path)?;
    let mut entries = Vec::new();

    for entry in entries_iter {
        let entry = entry?;
        let name = entry.file_name().as_str().unwrap_or("?").to_string();

        if !all && name.starts_with('.') {
            continue;
        }

        let meta = entry.metadata()?;
        let file_type = meta.file_type();
        let type_str = match file_type {
            FileType::Directory => "directory",
            FileType::Symlink => "symlink",
            FileType::Regular => "file",
            FileType::BlockDevice => "block_device",
            FileType::CharacterDevice => "char_device",
            FileType::Fifo => "fifo",
            FileType::Socket => "socket",
        };

        entries.push(LsEntry {
            name,
            entry_type: type_str.to_string(),
            size: meta.len(),
            mode: output::format_mode(file_type, meta.mode()),
            uid: meta.uid(),
            gid: meta.gid(),
        });
    }

    entries.sort_by(|a, b| a.name.cmp(&b.name));

    if json {
        output::print_json(&entries);
    } else if long {
        for e in &entries {
            println!(
                "{:10}  {:5}  {:5}  {:8}  {}",
                e.mode, e.uid, e.gid, e.size, e.name
            );
        }
    } else {
        for e in &entries {
            println!("{}", e.name);
        }
    }

    Ok(())
}
