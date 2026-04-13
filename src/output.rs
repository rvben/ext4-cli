use ext4_view::FileType;

pub fn format_mode(_file_type: FileType, _mode: u16) -> String {
    String::new() // implemented in Task 4
}

pub fn print_json<T: serde::Serialize>(_value: &T) {
    // implemented in Task 4
}
