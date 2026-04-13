use ext4_view::FileType;

pub fn format_mode(_file_type: FileType, _mode: u16) -> String {
    String::new()
}

pub fn print_json<T: serde::Serialize>(_value: &T) {}
