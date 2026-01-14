use crate::common::{Board, Ply, check_after_place, validate_ply};
use rand::Rng;
use rand::rngs::StdRng;

/// Generate a random valid ply at the specified layer
/// Layer = moves made, so pieces on board = max(0, layer - 1)
pub fn generate_random_ply(layer: usize, rng: &mut StdRng) -> Ply {
    if layer > 16 {
        panic!("Invalid layer: {} (must be 0-16)", layer);
    }

    let pieces_on_board = layer.saturating_sub(1);

    loop {
        // Choose (pieces_on_board + 1) pieces randomly without duplicates
        let mut pieces: Vec<u8> = (0..16).collect();
        let num_pieces_needed = pieces_on_board + 1;
        for i in 0..num_pieces_needed.min(16) {
            let j = rng.random_range(i..16);
            pieces.swap(i, j);
        }
        let selected_pieces = &pieces[0..num_pieces_needed];

        // Choose pieces_on_board squares randomly without duplicates
        let mut squares: Vec<u8> = (0..16).collect();
        for i in 0..pieces_on_board {
            let j = rng.random_range(i..16);
            squares.swap(i, j);
        }
        let selected_squares = &squares[0..pieces_on_board];

        // Build the board incrementally, checking for quarto after each placement
        let mut board = Board {
            occupancy: 0,
            attribute_masks: [0, 0, 0, 0],
        };

        let mut valid = true;
        for i in 0..pieces_on_board {
            let square = selected_squares[i];
            let piece = selected_pieces[i];
            let square_bit = 1u16 << square;

            // Place the piece and check for quarto
            let (next_board, is_quarto) = check_after_place(&board, piece, square_bit);

            if is_quarto {
                // This path creates a quarto before reaching the target layer
                valid = false;
                break;
            }

            board = next_board;
        }

        if !valid {
            // Try again with a different random configuration
            continue;
        }

        // The next piece is piece_to_place
        let piece_to_place = selected_pieces[pieces_on_board];

        let ply = Ply {
            board,
            piece_to_place,
        };

        // Only need to validate quarto intersection rule
        if validate_ply(&ply).is_ok() {
            return ply;
        }
    }
}
