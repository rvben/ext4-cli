use is_terminal::IsTerminal;
use serde::Serialize;
use std::io::{self, Write};

#[derive(Clone, Copy, Debug, PartialEq, clap::ValueEnum)]
pub enum OutputFormat {
    Auto,
    Text,
    Json,
}

pub fn is_json(format: OutputFormat) -> bool {
    match format {
        OutputFormat::Json => true,
        OutputFormat::Text => false,
        OutputFormat::Auto => !io::stdout().is_terminal(),
    }
}

/// Convert FileType and Unix mode bits into a string like "drwxr-xr-x".
pub fn format_mode(file_type: ext4_view::FileType, mode: u16) -> String {
    let type_char = match file_type {
        ext4_view::FileType::Directory => 'd',
        ext4_view::FileType::Symlink => 'l',
        ext4_view::FileType::BlockDevice => 'b',
        ext4_view::FileType::CharacterDevice => 'c',
        ext4_view::FileType::Fifo => 'p',
        ext4_view::FileType::Socket => 's',
        ext4_view::FileType::Regular => '-',
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

/// Serialize value to JSON and print to stdout.
pub fn print_json<T: Serialize>(value: &T) {
    println!("{}", serde_json::to_string_pretty(value).unwrap());
}

/// Emit a structured error envelope as the last line of stderr.
pub fn emit_error(kind: &str, message: &str, hint: Option<&str>) {
    let err = if let Some(h) = hint {
        serde_json::json!({"error": {"kind": kind, "message": message, "hint": h}})
    } else {
        serde_json::json!({"error": {"kind": kind, "message": message}})
    };
    let _ = writeln!(io::stderr(), "{}", serde_json::to_string(&err).unwrap());
}
