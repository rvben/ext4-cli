use anyhow::anyhow;
use clap::{Parser, Subcommand};
use std::process;

mod commands;
mod output;
mod source;

use commands::LsOptions;
use output::OutputFormat;

#[derive(Parser)]
#[command(
    name = "ext4",
    about = "Read ext4 filesystems. Run 'ext4 schema' for machine-readable command descriptions.",
    version,
    long_about = "Read ext4 filesystem images and block devices.\n\nRun 'ext4 schema' for a machine-readable description of all commands."
)]
struct Cli {
    /// Image file or block device path
    #[arg(short = 's', long, env = "EXT4_SOURCE", global = true)]
    source: Option<String>,

    /// Output format: auto (JSON when piped), text, or json
    #[arg(
        short = 'o',
        long = "output",
        global = true,
        default_value = "auto",
        value_enum
    )]
    output: OutputFormat,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List directory contents
    Ls {
        /// Show permissions, uid, gid, size
        #[arg(short, long)]
        long: bool,
        /// Include dotfiles
        #[arg(short, long)]
        all: bool,
        /// Maximum number of entries to return
        #[arg(long)]
        limit: Option<u64>,
        /// Number of entries to skip
        #[arg(long, default_value = "0")]
        offset: u64,
        /// Comma-separated list of fields to include in JSON output
        #[arg(long)]
        fields: Option<String>,
        /// Path inside the filesystem (default: /)
        path: Option<String>,
    },
    /// Print file contents to stdout
    Cat {
        /// Path inside the filesystem
        path: String,
    },
    /// Extract files from the filesystem
    Cp {
        /// Source path inside the filesystem
        src_path: String,
        /// Local destination path
        local_dest: String,
        /// Copy directory tree recursively
        #[arg(short, long)]
        recursive: bool,
    },
    /// Show file or directory metadata
    Stat {
        /// Path inside the filesystem
        path: String,
    },
    /// Show filesystem information
    Info,
    /// Print machine-readable schema (clispec v0.2)
    Schema,
}

fn main() {
    // Use try_parse so clap errors go through our JSON envelope rather than
    // clap's default prose error format on stderr.
    let cli = match Cli::try_parse() {
        Ok(c) => c,
        Err(e) => {
            if e.use_stderr() {
                // Clap parse error: emit a single-line JSON envelope on stderr.
                // Strip embedded newlines from the clap message so the envelope
                // stays a single line as required by the spec.
                let msg = e
                    .to_string()
                    .lines()
                    .find(|l| !l.trim().is_empty())
                    .unwrap_or("invalid argument")
                    .to_string();
                output::emit_error("invalid_input", &msg, Some("Run 'ext4 --help' for usage."));
                process::exit(4);
            }
            // --help and --version print to stdout and exit 0.
            let _ = e.print();
            process::exit(0);
        }
    };

    if let Err(e) = run(cli) {
        let msg = e.to_string();
        if e.downcast_ref::<std::io::Error>()
            .is_some_and(|io| io.kind() == std::io::ErrorKind::PermissionDenied)
        {
            output::emit_error("permission_denied", &msg, Some("Try running with sudo."));
            process::exit(2);
        }
        if msg.contains("not found")
            || msg.contains("NotFound")
            || msg.contains("not a directory")
            || msg.contains("No such")
        {
            output::emit_error("not_found", &msg, None);
            process::exit(3);
        }
        if msg.contains("not a valid ext4") || msg.contains("magic") {
            output::emit_error(
                "invalid_filesystem",
                &msg,
                Some("Check that the source is a valid ext4 filesystem image."),
            );
            process::exit(5);
        }
        output::emit_error("io_error", &msg, None);
        process::exit(1);
    }
}

fn run(cli: Cli) -> anyhow::Result<()> {
    let fmt = cli.output;

    match cli.command {
        Commands::Schema => commands::run_schema(),
        Commands::Info => {
            let src = require_source(cli.source)?;
            commands::run_info(&src, fmt)
        }
        Commands::Ls {
            long,
            all,
            limit,
            offset,
            fields,
            path,
        } => {
            let src = require_source(cli.source)?;
            let fs = source::open_source(&src)?;
            let path = path.as_deref().unwrap_or("/");
            let fields_list = fields.map(|f| {
                f.split(',')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<_>>()
            });
            commands::run_ls(
                &fs,
                path,
                LsOptions {
                    long,
                    all,
                    format: fmt,
                    limit,
                    offset,
                    fields: fields_list,
                },
            )
        }
        Commands::Cat { path } => {
            let src = require_source(cli.source)?;
            let fs = source::open_source(&src)?;
            commands::run_cat(&fs, &path)
        }
        Commands::Cp {
            src_path,
            local_dest,
            recursive,
        } => {
            let src = require_source(cli.source)?;
            let fs = source::open_source(&src)?;
            commands::run_cp(&fs, &src_path, &local_dest, recursive)
        }
        Commands::Stat { path } => {
            let src = require_source(cli.source)?;
            let fs = source::open_source(&src)?;
            commands::run_stat(&fs, &path, fmt)
        }
    }
}

fn require_source(source: Option<String>) -> anyhow::Result<String> {
    source.ok_or_else(|| anyhow!("no source specified - use --source <PATH> or set EXT4_SOURCE"))
}
