use anyhow::anyhow;
use clap::{Parser, Subcommand};
use std::process;

mod commands;
mod output;
mod source;

#[derive(Parser)]
#[command(name = "ext4", about = "Read ext4 filesystems", version)]
struct Cli {
    /// Image file or block device path
    #[arg(short = 's', long, env = "EXT4_SOURCE", global = true)]
    source: Option<String>,

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
        /// Output as JSON
        #[arg(long)]
        json: bool,
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
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show filesystem information
    Info {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

fn main() {
    if let Err(e) = run() {
        // Check for permission denied
        if let Some(io_err) = e.downcast_ref::<std::io::Error>() {
            if io_err.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("Error: {e}");
                eprintln!("Hint: try running with sudo");
                process::exit(2);
            }
        }
        // Check for ext4-view not-found errors
        let msg = e.to_string();
        if msg.contains("not found") || msg.contains("NotFound") {
            eprintln!("Error: {e}");
            process::exit(3);
        }
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Info { json } => {
            let src = require_source(cli.source)?;
            commands::run_info(&src, json)
        }
        Commands::Ls { long, all, json, path } => {
            let src = require_source(cli.source)?;
            let fs = source::open_source(&src)?;
            let path = path.as_deref().unwrap_or("/");
            commands::run_ls(&fs, path, long, all, json)
        }
        Commands::Cat { path } => {
            let src = require_source(cli.source)?;
            let fs = source::open_source(&src)?;
            commands::run_cat(&fs, &path)
        }
        Commands::Cp { src_path, local_dest, recursive } => {
            let src = require_source(cli.source)?;
            let fs = source::open_source(&src)?;
            commands::run_cp(&fs, &src_path, &local_dest, recursive)
        }
        Commands::Stat { path, json } => {
            let src = require_source(cli.source)?;
            let fs = source::open_source(&src)?;
            commands::run_stat(&fs, &path, json)
        }
    }
}

fn require_source(source: Option<String>) -> anyhow::Result<String> {
    source.ok_or_else(|| {
        anyhow!("no source specified — use --source <PATH> or set EXT4_SOURCE")
    })
}
