use ext4_view::FileType;

/// Convert FileType + Unix mode bits into a string like "drwxr-xr-x".
pub fn format_mode(file_type: FileType, mode: u16) -> String {
    let type_char = match file_type {
        FileType::Directory => 'd',
        FileType::Symlink => 'l',
        FileType::BlockDevice => 'b',
        FileType::CharacterDevice => 'c',
        FileType::Fifo => 'p',
        FileType::Socket => 's',
        FileType::Regular => '-',
    };
    let checks: &[(u16, char)] = &[
        (0o400, 'r'),
        (0o200, 'w'),
        (0o100, 'x'),
        (0o040, 'r'),
        (0o020, 'w'),
        (0o010, 'x'),
        (0o004, 'r'),
        (0o002, 'w'),
        (0o001, 'x'),
    ];
    let perms: String = checks
        .iter()
        .map(|&(bit, ch)| if mode & bit != 0 { ch } else { '-' })
        .collect();
    format!("{type_char}{perms}")
}

/// Serialize `value` to pretty-printed JSON and print to stdout.
pub fn print_json<T: serde::Serialize>(value: &T) {
    println!("{}", serde_json::to_string_pretty(value).unwrap());
}
