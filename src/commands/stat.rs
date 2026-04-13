use anyhow::Result;
use ext4_view::{Ext4, FileType};
use serde::Serialize;

use crate::output;

#[derive(Serialize)]
struct StatOutput {
    path: String,
    #[serde(rename = "type")]
    entry_type: String,
    size: u64,
    mode: String,
    mode_octal: String,
    uid: u32,
    gid: u32,
}

pub fn run_stat(fs: &Ext4, path: &str, json: bool) -> Result<()> {
    let meta = fs.symlink_metadata(path)?;
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

    let stat = StatOutput {
        path: path.to_string(),
        entry_type: type_str.to_string(),
        size: meta.len(),
        mode: output::format_mode(file_type, meta.mode()),
        mode_octal: format!("{:04o}", meta.mode()),
        uid: meta.uid(),
        gid: meta.gid(),
    };

    if json {
        output::print_json(&stat);
    } else {
        println!("path:       {}", stat.path);
        println!("type:       {}", stat.entry_type);
        println!("size:       {}", stat.size);
        println!("mode:       {} ({})", stat.mode, stat.mode_octal);
        println!("uid:        {}", stat.uid);
        println!("gid:        {}", stat.gid);
    }

    Ok(())
}
