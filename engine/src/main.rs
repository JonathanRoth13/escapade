#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

mod cli;
mod common;
mod engine;
mod index;
mod lab;
mod merge;
mod solve;
mod tablebase;

use clap::Parser;
use cli::{Cli, Commands, ConstantType, LabCommand};

fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::Solve(cmd) => {
            if let Err(e) = solve::run(
                cmd.layer,
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
                cmd.layer as u32,
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
            engine::run(tablebase);
        }
        Commands::Lab { action } => match action {
            LabCommand::GenerateConstants { constant_type } => match constant_type {
                ConstantType::LineMask => crate::lab::generate_code::generate_line_masks(),
                ConstantType::Occupancy => crate::lab::generate_code::generate_occupancy_masks(),
                ConstantType::CanonicalTransformations => {
                    crate::lab::generate_code::generate_canonical_transformations()
                }
            },
            LabCommand::Count { layer, workers } => {
                let _ = crate::lab::count(layer as usize, workers);
            }
            LabCommand::EstimateTablebaseSize { positions } => {
                crate::lab::estimate_tablebase_size(positions);
            }
            LabCommand::ValidateTablebase {
                layer,
                tablebase_dir,
                samples,
                workers,
                seed,
            } => {
                if let Err(e) = crate::lab::validate_tablebase(
                    layer as usize,
                    tablebase_dir,
                    samples,
                    workers,
                    seed,
                ) {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
            LabCommand::InspectPly {
                ply_string,
                evaluate,
                tablebase_dir,
            } => {
                if let Err(e) = crate::lab::inspect_ply(ply_string, evaluate, tablebase_dir) {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
            LabCommand::UpdateHeader {
                index_path,
                layer,
                shard_id,
                shard_bits,
            } => {
                if let Err(e) = crate::lab::update_header(index_path, layer, shard_id, shard_bits) {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
            LabCommand::RandomPosition { layer } => {
                if let Err(e) = crate::lab::random_position(layer as usize) {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
            LabCommand::ShowPieces => {
                if let Err(e) = crate::lab::show_pieces() {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
            LabCommand::RenderPly { ply_string } => {
                if let Err(e) = crate::lab::render_ply_display(&ply_string) {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        },
    }
}
