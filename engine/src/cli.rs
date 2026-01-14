use clap::{Args, Parser, Subcommand, ValueEnum, builder::ValueHint};
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
    /// Evaluate every position in a layer and write to partition files
    Solve(SolveCommand),

    /// Consolidate partition files into a shard file
    Merge(MergeCommand),

    /// Build minimal perfect hash (MPH) over a shard file
    Index(IndexCommand),

    /// Run the Quarto engine
    Engine(EngineCommand),

    /// Utilities for development and analysis
    Lab {
        #[command(subcommand)]
        action: LabCommand,
    },
}

#[derive(Subcommand, Debug)]
#[command(rename_all = "kebab-case")]
pub enum LabCommand {
    /// Generate compile-time constants (line masks, occupancy masks, etc.)
    GenerateConstants {
        #[arg(value_name = "TYPE")]
        constant_type: ConstantType,
    },

    /// Count all of the canonical positions in a given layer
    Count {
        #[arg(
            value_name = "LAYER",
            value_parser = clap::value_parser!(u32).range(0..=16)
        )]
        layer: u32,

        /// Worker threads (>=1). If omitted, auto-detect.
        #[arg(long, value_parser = clap::value_parser!(u32).range(1..))]
        workers: Option<u32>,
    },

    /// Estimate tablebase file size for a given number of positions
    EstimateTablebaseSize {
        /// Number of positions in the tablebase
        #[arg(value_name = "POSITIONS")]
        positions: u64,
    },

    /// Validate tablebase correctness and benchmark performance
    ValidateTablebase {
        #[arg(
            value_name = "LAYER",
            value_parser = clap::value_parser!(u32).range(0..=16)
        )]
        layer: u32,

        /// Directory containing tablebase files
        #[arg(value_name = "TABLEBASE_DIR", value_hint = ValueHint::DirPath)]
        tablebase_dir: PathBuf,

        /// Total number of positions to sample and validate across all workers
        #[arg(long, default_value_t = 2000)]
        samples: usize,

        /// Worker threads (>=1). If omitted, auto-detect.
        #[arg(long, value_parser = clap::value_parser!(u32).range(1..))]
        workers: Option<u32>,

        /// Random seed for reproducible sampling
        #[arg(long)]
        seed: Option<u64>,
    },

    /// Inspect a ply (parse, validate, display, and optionally evaluate)
    InspectPly {
        /// Ply string: either 22 hex chars (binary) or 17 chars (grid: 16 squares + piece)
        #[arg(value_name = "PLY_STRING")]
        ply_string: String,

        /// Evaluate the position
        #[arg(long)]
        evaluate: bool,

        /// Directory containing tablebase files (enables tablebase-based evaluation)
        #[arg(long, value_hint = ValueHint::DirPath)]
        tablebase_dir: Option<PathBuf>,
    },

    /// Update header fields in a tablebase file
    UpdateHeader {
        /// Path to the tablebase file
        #[arg(value_name = "INDEX_PATH", value_hint = ValueHint::FilePath)]
        index_path: PathBuf,

        #[arg(value_name = "LAYER", value_parser = clap::value_parser!(u8).range(0..=16))]
        layer: u8,

        /// Shard number (0-based)
        #[arg(value_name = "SHARD_ID")]
        shard_id: u8,

        /// Number of shard bits (0..=16) → total shards = 2^k
        #[arg(value_name = "SHARD_BITS", value_parser = clap::value_parser!(u8).range(0..=16))]
        shard_bits: u8,
    },

    /// Generate a random valid position at a given layer
    RandomPosition {
        #[arg(
            value_name = "LAYER",
            value_parser = clap::value_parser!(u32).range(0..=16)
        )]
        layer: u32,
    },

    /// Display all 16 pieces and their attributes
    ShowPieces,

    /// Render a complete ply (board + piece to place)
    RenderPly {
        /// Ply string: either 22 hex chars (binary) or 17 chars (grid: 16 squares + piece)
        #[arg(value_name = "PLY_STRING")]
        ply_string: String,
    },
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum ConstantType {
    /// Line masks (rows, columns, diagonals)
    LineMask,
    /// Canonical occupancy patterns for each layer
    Occupancy,
    /// Canonical transformation lookup table (2^16 entries)
    CanonicalTransformations,
}

#[derive(Args, Debug)]
pub struct SolveCommand {
    /// Layer to solve (moves made = 0..=16)
    #[arg(
        value_name = "LAYER",
        value_parser = clap::value_parser!(u32).range(0..=16)
    )]
    pub layer: u32,

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

    #[arg(value_name = "LAYER")]
    pub layer: u8,

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
    #[arg(long, value_hint = ValueHint::DirPath)]
    pub tablebase_dir: Option<PathBuf>,
}
