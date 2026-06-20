use anyhow::Result;

pub fn run_schema() -> Result<()> {
    let schema = serde_json::json!({
        "clispec": "0.2",
        "name": "ext4",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "Read ext4 filesystem images and block devices",
        "global_args": [
            {
                "name": "--source",
                "type": "path",
                "required": false,
                "description": "Image file or block device path (or set EXT4_SOURCE)"
            },
            {
                "name": "--output",
                "type": "string",
                "required": false,
                "default": "auto",
                "enum": ["auto", "text", "json"],
                "description": "Output format: auto detects TTY (JSON when piped)"
            }
        ],
        "commands": [
            {
                "name": "ls",
                "description": "List directory contents",
                "mutating": false,
                "args": [
                    {"name": "--long", "type": "boolean", "required": false, "description": "Show permissions, uid, gid, size"},
                    {"name": "--all", "type": "boolean", "required": false, "description": "Include dotfiles"},
                    {"name": "--limit", "type": "integer", "required": false, "description": "Maximum number of entries to return"},
                    {"name": "--offset", "type": "integer", "required": false, "default": 0, "description": "Number of entries to skip"},
                    {"name": "--fields", "type": "string", "required": false, "description": "Comma-separated list of fields to include in JSON output"},
                    {"name": "path", "type": "path", "required": false, "description": "Path inside the filesystem (default: /)"}
                ],
                "output_fields": [
                    {"name": "items", "type": "object[]"},
                    {"name": "total", "type": "integer"},
                    {"name": "limit", "type": "integer | null"},
                    {"name": "offset", "type": "integer"}
                ]
            },
            {
                "name": "cat",
                "description": "Print file contents to stdout",
                "mutating": false,
                "args": [
                    {"name": "path", "type": "path", "required": true, "description": "Path inside the filesystem"}
                ],
                "output_fields": []
            },
            {
                "name": "cp",
                "description": "Extract files from the filesystem",
                "mutating": false,
                "args": [
                    {"name": "src_path", "type": "path", "required": true, "description": "Source path inside the filesystem"},
                    {"name": "local_dest", "type": "path", "required": true, "description": "Local destination path"},
                    {"name": "--recursive", "type": "boolean", "required": false, "description": "Copy directory tree recursively"}
                ],
                "output_fields": []
            },
            {
                "name": "stat",
                "description": "Show file or directory metadata",
                "mutating": false,
                "args": [
                    {"name": "path", "type": "path", "required": true, "description": "Path inside the filesystem"}
                ],
                "output_fields": [
                    {"name": "path", "type": "string"},
                    {"name": "type", "type": "string"},
                    {"name": "size", "type": "integer"},
                    {"name": "mode", "type": "string"},
                    {"name": "mode_octal", "type": "string"},
                    {"name": "uid", "type": "integer"},
                    {"name": "gid", "type": "integer"}
                ]
            },
            {
                "name": "info",
                "description": "Show filesystem information",
                "mutating": false,
                "args": [],
                "output_fields": [
                    {"name": "uuid", "type": "string"},
                    {"name": "label", "type": "string"},
                    {"name": "block_size", "type": "integer"},
                    {"name": "inodes_count", "type": "integer"},
                    {"name": "free_inodes_count", "type": "integer"},
                    {"name": "blocks_count", "type": "integer"},
                    {"name": "free_blocks_count", "type": "integer"},
                    {"name": "features", "type": "string[]"}
                ]
            },
            {
                "name": "schema",
                "description": "Print machine-readable schema (clispec v0.2)",
                "mutating": false,
                "args": [],
                "output_fields": []
            }
        ],
        "errors": [
            {"kind": "io_error", "exit_code": 1, "retryable": false, "description": "I/O error reading the filesystem image"},
            {"kind": "permission_denied", "exit_code": 2, "retryable": false, "description": "Insufficient permissions to open the source"},
            {"kind": "not_found", "exit_code": 3, "retryable": false, "description": "Path not found inside the filesystem"},
            {"kind": "invalid_input", "exit_code": 4, "retryable": false, "description": "Invalid argument or flag value"},
            {"kind": "invalid_filesystem", "exit_code": 5, "retryable": false, "description": "Source is not a valid ext4 filesystem"}
        ]
    });

    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    Ok(())
}
