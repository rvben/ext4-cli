use anyhow::Result;
use ext4_view::Ext4;
use std::io::{self, Read, Write};

pub fn run_cat(fs: &Ext4, path: &str) -> Result<()> {
    let mut file = fs.open(path)?;
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut buf = vec![0u8; 64 * 1024];

    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        out.write_all(&buf[..n])?;
    }

    Ok(())
}
