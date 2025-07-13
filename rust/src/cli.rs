// Command-line interface definitions

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "bbcpr",
    about = "Secure and fast copy utility - Rust implementation",
    version,
    author = "Andrew Mello <andrew@88plug.com>",
    long_about = "bbcpr (Berkeley Byte Copy Rust) is a modern Rust implementation of bbcp, providing \
                  high-performance parallel file transfers with support for SSH, checksums, advanced resume functionality, \
                  and cross-platform operation. Unlike rsync/aria2c/rclone, bbcpr can split individual large \
                  files into multiple parallel streams for maximum speed."
)]
pub struct Args {
    /// Source file(s) or directory
    #[arg(value_name = "SOURCE")]
    pub source: Vec<String>,

    /// Destination file or directory
    #[arg(value_name = "DEST")]
    pub destination: String,

    /// Append mode to restart a previously failed copy
    #[arg(short = 'a', long = "append", value_name = "DIR")]
    pub append_dir: Option<PathBuf>,

    /// Read blocking factor (default: 1)
    #[arg(short = 'b', long = "block-factor", default_value = "1")]
    pub block_factor: u32,

    /// Read/write I/O buffer size
    #[arg(short = 'B', long = "buffer-size", value_name = "SIZE")]
    pub buffer_size: Option<String>,

    /// Compress data before sending (level 1-9)
    #[arg(short = 'c', long = "compress", value_name = "LEVEL")]
    pub compress_level: Option<u8>,

    /// Configuration file to process
    #[arg(short = 'C', long = "config", value_name = "FILE")]
    pub config_file: Option<PathBuf>,

    /// Enable debugging output
    #[arg(short = 'D', long = "debug")]
    pub debug: bool,

    /// Error check data using checksum
    #[arg(short = 'e', long = "error-check")]
    pub error_check: bool,

    /// Checksum algorithm specification
    #[arg(short = 'E', long = "checksum", value_name = "ALGO")]
    pub checksum_algo: Option<String>,

    /// Force copy by unlinking target first
    #[arg(short = 'f', long = "force")]
    pub force: bool,

    /// Skip space check on target
    #[arg(short = 'F', long = "no-space-check")]
    pub no_space_check: bool,

    /// Print help information
    #[arg(short = 'h', long = "help")]
    pub help: bool,

    /// SSH identity file
    #[arg(short = 'i', long = "identity", value_name = "FILE")]
    pub identity_file: Option<PathBuf>,

    /// File containing list of files to copy
    #[arg(short = 'I', long = "file-list", value_name = "FILE")]
    pub file_list: Option<PathBuf>,

    /// Keep destination file on failure
    #[arg(short = 'k', long = "keep")]
    pub keep_partial: bool,

    /// Log file for stderr
    #[arg(short = 'l', long = "log", value_name = "FILE")]
    pub log_file: Option<PathBuf>,

    /// Target file mode
    #[arg(short = 'm', long = "mode", value_name = "MODE")]
    pub file_mode: Option<String>,

    /// Don't resolve hostnames
    #[arg(short = 'n', long = "no-dns")]
    pub no_dns: bool,

    /// Number of parallel network streams (default: 4)
    #[arg(short = 's', long = "streams", default_value = "4")]
    pub streams: u32,

    /// Enforce output ordering
    #[arg(short = 'o', long = "ordered")]
    pub ordered: bool,

    /// Omit existing files at target
    #[arg(short = 'O', long = "omit-existing")]
    pub omit_existing: bool,

    /// Preserve source attributes
    #[arg(short = 'p', long = "preserve")]
    pub preserve: bool,

    /// Progress message interval (seconds)
    #[arg(short = 'P', long = "progress", value_name = "SEC")]
    pub progress_interval: Option<u32>,

    /// Recursively copy directories
    #[arg(short = 'r', long = "recursive")]
    pub recursive: bool,

    /// Time limit for copy (seconds)
    #[arg(short = 't', long = "time-limit", value_name = "SEC")]
    pub time_limit: Option<u32>,

    /// Verbose mode
    #[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Window size for transmission
    #[arg(short = 'w', long = "window-size", value_name = "SIZE")]
    pub window_size: Option<String>,

    /// Maximum transfer rate
    #[arg(short = 'x', long = "rate-limit", value_name = "RATE")]
    pub rate_limit: Option<String>,

    /// Use reverse connection (target to source)
    #[arg(short = 'z', long = "reverse")]
    pub reverse: bool,

    /// Port range for data connections
    #[arg(short = 'Z', long = "port-range", value_name = "PORT1:PORT2")]
    pub port_range: Option<String>,

    /// Use IPv4 only
    #[arg(short = '4', long = "ipv4")]
    pub ipv4_only: bool,

    /// Resume interrupted transfers automatically
    #[arg(short = 'R', long = "resume")]
    pub resume: bool,

    /// List pending transfers that can be resumed
    #[arg(long = "list-transfers")]
    pub list_transfers: bool,

    /// Cancel a specific transfer by ID
    #[arg(long = "cancel-transfer", value_name = "ID")]
    pub cancel_transfer: Option<String>,

    /// Clean up old transfer state files (days)
    #[arg(long = "cleanup-transfers", value_name = "DAYS")]
    pub cleanup_transfers: Option<u64>,

    /// Keep transfer state files after successful completion
    #[arg(long = "keep-state")]
    pub keep_state: bool,

    /// Print license and exit
    #[arg(long = "license")]
    pub license: bool,

    /// Print version and exit
    #[arg(short = '#', long = "version")]
    pub version: bool,

    /// Resume interrupted transfers automatically
    #[arg(short = 'R', long = "resume")]
    pub resume: bool,

    /// List pending transfers that can be resumed
    #[arg(long = "list-transfers")]
    pub list_transfers: bool,

    /// Cancel a specific transfer by ID
    #[arg(long = "cancel-transfer", value_name = "ID")]
    pub cancel_transfer: Option<String>,

    /// Clean up old transfer state files (days)
    #[arg(long = "cleanup-transfers", value_name = "DAYS")]
    pub cleanup_transfers: Option<u32>,

    /// Keep transfer state files after completion
    #[arg(long = "keep-state")]
    pub keep_state: bool,
}