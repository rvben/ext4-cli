use anyhow::Result;
use ext4_view::Ext4;

pub fn open_source(path: &str) -> Result<Ext4> {
    Ok(Ext4::load_from_path(path)?)
}
