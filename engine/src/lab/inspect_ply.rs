use crate::common::{
    Board, Ply, canonicalize_ply, evaluate, format_ply, format_ply_hex, parse_ply,
    pretty_print_ply, validate_ply,
};
use crate::tablebase::TablebaseIndex;
use anyhow::Result;
use std::path::PathBuf;

pub fn run(ply_string: String, evaluate_flag: bool, tablebase_dir: Option<PathBuf>) -> Result<()> {
    // Step 1: Parse the ply string
    let ply = parse_ply(&ply_string)?;

    // Step 2: Validate the ply
    validate_ply(&ply)?;

    // Step 3: Pretty print the original ply
    println!("═══════════════════════════════════════════════════════════");
    print!("{}", pretty_print_ply(&ply));
    println!("Grid format:  \"{}\"", format_ply(&ply));
    println!("Binary (hex): {}", format_ply_hex(&ply));

    // Step 4: Canonicalize and pretty print
    let canonical_ply = if ply.board.occupancy == 0 {
        // Special case: empty board - hardcode canonical form with piece_to_place = 0
        Ply {
            board: Board {
                occupancy: 0,
                attribute_masks: [0, 0, 0, 0],
            },
            piece_to_place: 0,
        }
    } else {
        canonicalize_ply(&ply)
    };
    println!("═══════════════════════════════════════════════════════════");
    print!("{}", pretty_print_ply(&canonical_ply));
    println!("Grid format:  \"{}\"", format_ply(&canonical_ply));
    println!("Binary (hex): {}", format_ply_hex(&canonical_ply));

    // Step 5: Evaluate (if enabled)
    if evaluate_flag {
        println!("═══════════════════════════════════════════════════════════");
        let (outcome, _tablebase_opt) = if let Some(ref tb_dir) = tablebase_dir {
            println!("Loading tablebase from {}...", tb_dir.display());
            let tablebase = TablebaseIndex::load_from_dir(tb_dir)?;

            // Show what layers were loaded
            let available_layers = tablebase.available_layers();
            let loaded_layers: Vec<usize> = (0..18).filter(|&i| available_layers[i]).collect();
            if loaded_layers.is_empty() {
                println!("WARNING: No tablebase layers found!");
            } else {
                println!("Loaded layers: {:?}", loaded_layers);
            }

            println!("Using tablebase evaluation");
            (evaluate(&ply, Some(&tablebase)), Some(tablebase))
        } else {
            println!("Using minimax evaluation");
            (evaluate(&ply, None), None)
        };

        println!("Outcome: {}", outcome);

        // Decode outcome
        if outcome == 15 {
            println!("Result:  Draw");
        } else if outcome % 2 == 1 {
            println!("Result:  White wins");
            let pieces_when_win = 17 - outcome as usize;
            println!("Pieces on board when game ends: {}", pieces_when_win);
        } else {
            println!("Result:  Black wins");
            let pieces_when_win = 17 - outcome as usize;
            println!("Pieces on board when game ends: {}", pieces_when_win);
        }
    }
    println!("═══════════════════════════════════════════════════════════");

    Ok(())
}
