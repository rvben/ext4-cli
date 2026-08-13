use anyhow::Result;

pub fn run_schema() -> Result<()> {
    let schema = serde_json::json!({
        "clispec": "0.3",
        "name": "ext4",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "Read ext4 filesystem images and block devices",
        "output": {"tty": "text", "piped": "json"},
        "global_args": [
            {
                "name": "--source",
                "short": "-s",
                "type": "path",
                "required": false,
                "description": "Image file or block device path (or set EXT4_SOURCE)"
            },
            {
                "name": "--output",
                "short": "-o",
                "type": "string",
                "required": false,
                "default": "auto",
                "enum": ["auto", "text", "json"],
                "description": "Output format: auto detects TTY (JSON when piped)"
            }
        ],
        "commands": [
            {
                "name": "capabilities",
                "description": "Show supported ext4 operations without opening a filesystem.",
                "effects": "read_only",
                "mutating": false,
                "cardinality": "bounded",
                "output_fields": [
                    {"name": "capabilities", "type": "array", "items": {"type": "string"}}
                ],
                "example": {"args": ["capabilities"]}
            },
            {
                "name": "ls",
                "description": "List directory contents",
                "effects": "read_only",
                "mutating": false,
                "cardinality": "unbounded",
                "pagination": {"style": "offset", "offset_arg": "--offset", "limit_arg": "--limit"},
                "fields_arg": "--fields",
                "args": [
                    {"name": "--long", "type": "boolean", "required": false, "description": "Show permissions, uid, gid, size"},
                    {"name": "--all", "type": "boolean", "required": false, "description": "Include dotfiles"},
                    {"name": "--limit", "type": "integer", "required": false, "description": "Maximum number of entries to return"},
                    {"name": "--offset", "type": "integer", "required": false, "default": 0, "description": "Number of entries to skip"},
                    {"name": "--fields", "type": "string", "required": false, "description": "Comma-separated list of fields to include in JSON output"},
                    {"name": "path", "type": "path", "required": false, "description": "Path inside the filesystem (default: /)"}
                ],
                "output_fields": [
                    {"name": "name", "type": "string"},
                    {"name": "type", "type": "string"},
                    {"name": "size", "type": "integer"},
                    {"name": "mode", "type": "string"},
                    {"name": "uid", "type": "integer"},
                    {"name": "gid", "type": "integer"}
                ]
            },
            {
                "name": "cat",
                "description": "Print file contents to stdout",
                "effects": "read_only",
                "mutating": false,
                "output_kind": "opaque",
                "media_type": "application/octet-stream",
                "args": [
                    {"name": "path", "type": "path", "required": true, "description": "Path inside the filesystem"}
                ],
            },
            {
                "name": "cp",
                "description": "Extract files from the filesystem",
                "effects": "idempotent",
                "mutating": true,
                "cardinality": "single",
                "args": [
                    {"name": "src_path", "type": "path", "required": true, "description": "Source path inside the filesystem"},
                    {"name": "local_dest", "type": "path", "required": true, "description": "Local destination path"},
                    {"name": "--recursive", "type": "boolean", "required": false, "description": "Copy directory tree recursively"}
                ],
                "output_fields": [
                    {"name": "source", "type": "string", "description": "Source path inside the ext4 filesystem."},
                    {"name": "destination", "type": "string", "description": "Requested local destination path."}
                ]
            },
            {
                "name": "stat",
                "description": "Show file or directory metadata",
                "effects": "read_only",
                "mutating": false,
                "cardinality": "single",
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
                "effects": "read_only",
                "mutating": false,
                "cardinality": "single",
                "args": [],
                "output_fields": [
                    {"name": "uuid", "type": "string"},
                    {"name": "label", "type": "string"},
                    {"name": "block_size", "type": "integer"},
                    {"name": "inodes_count", "type": "integer"},
                    {"name": "free_inodes_count", "type": "integer"},
                    {"name": "blocks_count", "type": "integer"},
                    {"name": "free_blocks_count", "type": "integer"},
                    {"name": "features", "type": "array", "items": {"type": "string"}}
                ]
            },
            {
                "name": "schema",
                "description": "Print the machine-readable clispec v0.3 candidate contract",
                "effects": "read_only",
                "mutating": false,
                "cardinality": "single",
                "args": [],
                "stdout_schema": {"$ref": "https://clispec.dev/schema/v0.3.json"}
            }
        ],
        "errors": [
            {"kind": "io", "exit_code": 1, "retryable": false, "description": "I/O error reading the filesystem image or writing an extracted file"},
            {"kind": "permission_denied", "exit_code": 2, "retryable": false, "description": "Insufficient permissions to open the source"},
            {"kind": "not_found", "exit_code": 3, "retryable": false, "description": "Path not found inside the filesystem"},
            {"kind": "invalid_input", "exit_code": 4, "retryable": false, "description": "Invalid argument or flag value"},
            {"kind": "invalid_filesystem", "exit_code": 5, "retryable": false, "description": "Source is not a valid ext4 filesystem"}
        ]
    });

    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    Ok(())
}
