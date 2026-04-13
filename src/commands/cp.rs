use anyhow::{Result, bail};
use ext4_view::Ext4;
use std::io::Read;
use std::path::Path;

pub fn run_cp(fs: &Ext4, src_path: &str, local_dest: &str, recursive: bool) -> Result<()> {
    let meta = fs.symlink_metadata(src_path)?;

    if meta.is_dir() {
        if !recursive {
            bail!("'{}' is a directory — use -r to copy recursively", src_path);
        }
        copy_dir(fs, src_path, Path::new(local_dest))
    } else {
        copy_file(fs, src_path, Path::new(local_dest))
    }
}

fn copy_file(fs: &Ext4, src_path: &str, dest: &Path) -> Result<()> {
    let dest = if dest.is_dir() {
        let name = src_path.split('/').next_back().unwrap_or("file");
        dest.join(name)
    } else {
        dest.to_path_buf()
    };

    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut file = fs.open(src_path)?;
    let mut out = std::fs::File::create(&dest)?;
    let mut buf = vec![0u8; 64 * 1024];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        std::io::Write::write_all(&mut out, &buf[..n])?;
    }

    Ok(())
}

fn copy_dir(fs: &Ext4, src_path: &str, dest: &Path) -> Result<()> {
    std::fs::create_dir_all(dest)?;

    for entry in fs.read_dir(src_path)? {
        let entry = entry?;
        let name = entry.file_name().as_str().unwrap_or("?").to_string();

        if name == "." || name == ".." {
            continue;
        }

        let child_src = format!("{}/{}", src_path.trim_end_matches('/'), name);
        let child_dest = dest.join(&name);
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            copy_dir(fs, &child_src, &child_dest)?;
        } else if file_type.is_regular_file() {
            copy_file(fs, &child_src, &child_dest)?;
        }
    }

    Ok(())
}
