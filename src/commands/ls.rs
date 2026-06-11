use anyhow::Result;
use ext4_view::{Ext4, FileType};
use serde::Serialize;
use serde_json::Value;

use crate::output::{self, OutputFormat};

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

pub struct LsOptions {
    pub long: bool,
    pub all: bool,
    pub format: OutputFormat,
    pub limit: Option<u64>,
    pub offset: u64,
    pub fields: Option<Vec<String>>,
}

pub fn run_ls(fs: &Ext4, path: &str, opts: LsOptions) -> Result<()> {
    let LsOptions {
        long,
        all,
        format,
        limit,
        offset,
        fields,
    } = opts;
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

    let total = entries.len() as u64;
    let offset = offset.min(total);
    let sliced: Vec<_> = entries.into_iter().skip(offset as usize).collect();
    let sliced: Vec<_> = if let Some(lim) = limit {
        sliced.into_iter().take(lim as usize).collect()
    } else {
        sliced
    };

    if output::is_json(format) {
        let items: Vec<Value> = sliced
            .iter()
            .map(|e| {
                let mut obj = serde_json::to_value(e).unwrap();
                if let Some(ref fs) = fields {
                    let map = obj.as_object_mut().unwrap();
                    map.retain(|k, _| fs.contains(k));
                }
                obj
            })
            .collect();
        let envelope = serde_json::json!({
            "items": items,
            "total": total,
            "limit": limit,
            "offset": offset
        });
        output::print_json(&envelope);
    } else if long {
        for e in &sliced {
            println!(
                "{:10}  {:5}  {:5}  {:8}  {}",
                e.mode, e.uid, e.gid, e.size, e.name
            );
        }
    } else {
        for e in &sliced {
            println!("{}", e.name);
        }
    }

    Ok(())
}
