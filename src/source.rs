use anyhow::{Context, Result};
use ext4_view::Ext4;

pub fn open_source(path: &str) -> Result<Ext4> {
    Ext4::load_from_path(path)
        .with_context(|| format!("failed to open '{path}'"))
}
