use crate::common::{
    Board, OCCUPANCY_MASKS, Ply, ROOT_BOARD, hash_worker_ply,
    is_ply_canonical_under_attribute_relabeling_only,
};

const SPLIT_DEPTH: usize = 4;

#[derive(Default)]
pub struct CountStats {
    pub total_positions: usize,
    pub canonical_positions: usize,
    pub canonical_non_terminal: usize,
}

pub fn worker_root(worker_id: u32, total_workers: u32, layer: usize) -> CountStats {
    if layer == 0 {
        panic!("Layer 0 enumeration not implemented");
    }

    if layer > 16 {
        panic!("Invalid layer: {} (must be 0-16)", layer);
    }

    let split_depth = (layer - 2).min(SPLIT_DEPTH);

    let available_pieces: u16 = u16::MAX ^ 1; // the zero piece has to be placed in the greatest square if the board is to be canonical

    let mut stats = CountStats::default();
    for &mask in OCCUPANCY_MASKS[layer] {
        let mask_stats = backtrack_split(
            worker_id,
            total_workers,
            layer,
            0,
            &ROOT_BOARD,
            mask,
            available_pieces,
            split_depth,
        );
        stats.total_positions += mask_stats.total_positions;
        stats.canonical_positions += mask_stats.canonical_positions;
        stats.canonical_non_terminal += mask_stats.canonical_non_terminal;
    }
    stats
}

#[allow(clippy::too_many_arguments)]
fn backtrack_split(
    worker_id: u32,
    total_workers: u32,
    layer: usize,
    depth: usize,
    board: &Board,
    available_squares: u16,
    available_pieces: u16,
    split_depth: usize,
) -> CountStats {
    // ----- Split-by-worker decision -----
    if depth == split_depth {
        let ply: Ply = Ply {
            board: *board,
            piece_to_place: 0,
        };
        // Use worker hash for thread partitioning (TRANSIENT)
        let worker_hash = hash_worker_ply(&ply);
        if (worker_hash % total_workers as u64) != worker_id as u64 {
            return CountStats::default();
        }
    }

    let mut stats = CountStats::default();

    if depth == layer - 2 {
        // all leaf nodes will have this board
        let (final_board, is_quarto) =
            crate::common::check_after_place(board, 0, available_squares);

        let mut running_pieces_available: u16 = available_pieces;
        while running_pieces_available != 0 {
            let piece: u16 = lowest_bit(running_pieces_available);
            running_pieces_available ^= piece;
            let piece_index: u8 = piece.trailing_zeros() as u8;
            let ply = Ply {
                board: final_board,
                piece_to_place: piece_index,
            };

            stats.total_positions += 1;

            if is_ply_canonical_under_attribute_relabeling_only(&ply) {
                stats.canonical_positions += 1;

                if !is_quarto {
                    stats.canonical_non_terminal += 1;
                }
            }
        }

        return stats;
    }

    // Recurse: place next piece
    let square: u16 = lowest_bit(available_squares);
    let mut running_pieces_available: u16 = available_pieces;

    while running_pieces_available != 0 {
        let piece: u16 = lowest_bit(running_pieces_available);
        running_pieces_available ^= piece;
        let piece_index: u8 = piece.trailing_zeros() as u8;

        let (board_next, is_quarto) = crate::common::check_after_place(board, piece_index, square);

        // we are only interested in the non-terminals
        if is_quarto {
            continue;
        }

        let available_squares_next: u16 = available_squares ^ square;
        let available_pieces_next: u16 = available_pieces ^ piece;

        let child_stats = backtrack_split(
            worker_id,
            total_workers,
            layer,
            depth + 1,
            &board_next,
            available_squares_next,
            available_pieces_next,
            split_depth,
        );

        stats.total_positions += child_stats.total_positions;
        stats.canonical_positions += child_stats.canonical_positions;
        stats.canonical_non_terminal += child_stats.canonical_non_terminal;
    }
    stats
}

#[inline(always)]
pub fn lowest_bit(mask: u16) -> u16 {
    mask & mask.wrapping_neg()
}
