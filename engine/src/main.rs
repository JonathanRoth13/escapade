#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

mod cli;
mod engine;
mod index;
mod core;
mod merge;
mod solve;
mod tablebase;

use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::Solve(cmd) => {
            if let Err(e) = solve::run(
                cmd.depth,
                cmd.shard_bits,
                cmd.reserve_os,
                cmd.workers,
                cmd.tb_bytes,
                cmd.partition_dir,
                cmd.resume_config,
                cmd.tablebase_dir,
            ) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Merge(cmd) => {
            if let Err(e) = merge::run(cmd.input_dir, cmd.output) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Index(cmd) => {
            if let Err(e) = index::run(
                cmd.depth as u32,
                cmd.shard_path,
                cmd.index_path,
                cmd.shard_id,
                cmd.shard_bits,
                cmd.target_bucket_size,
                cmd.reserve_os,
                cmd.workers,
            ) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Engine(cmd) => {
            let tablebase = if let Some(ref tb_dir) = cmd.tablebase_dir {
                match tablebase::TablebaseIndex::load_from_dir_silent(tb_dir) {
                    Ok(tb) => Some(tb),
                    Err(e) => {
                        eprintln!("Error loading tablebase: {}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                None
            };
            engine::run(tablebase, &cmd.listen).await;
        }
    }
}
