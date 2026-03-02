use clap::{Args, Parser, Subcommand, builder::ValueHint};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "escapade",
    version,
    about = "Escapade engine CLI",
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
#[command(rename_all = "kebab-case", subcommand_required = true)]
pub enum Commands {
    /// Evaluate every position in a depth and write to partition files
    Solve(SolveCommand),

    /// Consolidate partition files into a shard file
    Merge(MergeCommand),

    /// Build minimal perfect hash (MPH) over a shard file
    Index(IndexCommand),

    /// Run the Quarto engine
    Engine(EngineCommand),
}

#[derive(Args, Debug)]
pub struct SolveCommand {
    /// Depth
    #[arg(
        value_name = "DEPTH",
        value_parser = clap::value_parser!(u32).range(0..=16)
    )]
    pub depth: u32,

    /// Directory to write partition files
    #[arg(value_name = "PARTITION_DIR", value_hint = ValueHint::DirPath)]
    pub partition_dir: PathBuf,

    /// Number of shard bits (0..=16) → number of shards = 2^k
    #[arg(long, default_value_t = 7, value_parser = clap::value_parser!(u8).range(0..=16))]
    pub shard_bits: u8,

    /// Directory containing tablebase files for O(1) lookups
    #[arg(long, value_hint = ValueHint::DirPath)]
    pub tablebase_dir: Option<PathBuf>,

    /// Worker threads (>=1). If omitted, auto-detect.
    #[arg(long, value_parser = clap::value_parser!(u32).range(1..))]
    pub workers: Option<u32>,

    /// Fraction of RAM to reserve for OS (default: 0.20)
    #[arg(long, default_value_t = 0.20)]
    pub reserve_os: f64,

    /// Optional TB memory budget in bytes
    #[arg(long)]
    pub tb_bytes: Option<u64>,

    /// Resume config file (JSON with worker resume points)
    #[arg(long, value_hint = ValueHint::FilePath)]
    pub resume_config: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct MergeCommand {
    /// Directory containing partition files from solve
    #[arg(value_name = "PARTITION_DIR", value_hint = ValueHint::DirPath)]
    pub input_dir: PathBuf,

    /// Path to output shard file
    #[arg(value_name = "SHARD_PATH", value_hint = ValueHint::FilePath)]
    pub output: PathBuf,
}

#[derive(Args, Debug)]
pub struct IndexCommand {
    /// Path to input shard file (from merge)
    #[arg(value_name = "SHARD_PATH", value_hint = ValueHint::FilePath)]
    pub shard_path: PathBuf,

    /// Path to output tablebase file
    #[arg(value_name = "INDEX_PATH", value_hint = ValueHint::FilePath)]
    pub index_path: PathBuf,

    /// Depth
    #[arg(value_name = "DEPTH")]
    pub depth: u8,

    /// Shard number (0-based)
    #[arg(value_name = "SHARD_ID")]
    pub shard_id: u8,

    /// Number of shard bits (0..=16) → total shards = 2^k
    #[arg(value_name = "SHARD_BITS", value_parser = clap::value_parser!(u8).range(0..=16))]
    pub shard_bits: u8,

    /// Target bucket size (t): r = 2^⌈log2(n/t)⌉
    #[arg(long, short = 't', default_value_t = 16)]
    pub target_bucket_size: u64,

    /// Percent of RAM reserved for OS/other (e.g., 0.20 = 20%)
    #[arg(long, default_value_t = 0.20)]
    pub reserve_os: f64,

    /// Worker threads (>=1). If omitted, auto-detect.
    #[arg(long, short = 'w')]
    pub workers: Option<u32>,
}

#[derive(Args, Debug)]
pub struct EngineCommand {
    /// Directory containing tablebase files
    #[arg(long, env = "TABLEBASE_DIR", value_hint = ValueHint::DirPath)]
    pub tablebase_dir: Option<PathBuf>,

    /// Address to listen on
    #[arg(long, env = "LISTEN_ADDR", default_value = "0.0.0.0:8080")]
    pub listen: String,
}
