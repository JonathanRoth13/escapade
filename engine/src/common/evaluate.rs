use crate::{
    common::{LAYER_0_SENTINEL, LAYER_1_CANONICAL, Ply, canonicalize_ply, check_after_place},
    tablebase::TablebaseIndex,
};

// Outcome encoding
// it is possible for the game to end on layers 5 through 17
// if the game ends on layers 5,7,9,11,13,15, it is a win for white
// if the game ends on layers 6,8,10,12,14,16, it is a win for black
// if the game ends on layer 17, it is either a tie or a win for white
//
// outcome is encoded as 1 through 13 and 15
// if the game ends on layer 17 with a tie, the outcome is 15
// otherwise, the outcome is calculated using the formula: outcome = 18-layer

// Score encoding
// score is a linear value that corresponds directly to outcome, it's used for alpha-beta pruning
// score can be either positive or negative it is not absolute but is from the perspective of one
// side
// however, the absolute value of the score does correspond with the outcome with the following
// formula: score = 0 if outcome is 15, otherwise score = outcome + 99

const OUTCOME_DRAW: u8 = 15;
const SCORE_DRAW: i16 = 0;
const SCORE_BASE: i16 = 100; // outcome - 1 + SCORE_BASE = outcome + 99

/// Convert layer to outcome for immediate win
#[inline]
fn layer_to_outcome(layer: usize) -> u8 {
    (18 - layer) as u8
}

/// Convert outcome to score (absolute value)
#[inline]
fn outcome_to_score(outcome: u8) -> i16 {
    if outcome == OUTCOME_DRAW {
        SCORE_DRAW
    } else {
        (outcome - 1) as i16 + SCORE_BASE
    }
}

/// Convert outcome to sort score from a given perspective for descending sort
/// Higher scores = better outcomes
pub fn outcome_to_sort_score(outcome: u8, is_player_first: bool) -> i16 {
    if outcome == OUTCOME_DRAW {
        return SCORE_DRAW; // 0 in the middle
    }
    let first_player_wins = outcome % 2 == 1;
    let score = outcome_to_score(outcome);

    if is_player_first {
        if first_player_wins { score } else { -score }
    } else if first_player_wins {
        -score
    } else {
        score
    }
}

/// Convert score to outcome (from absolute value)
#[inline]
fn score_to_outcome(score: i16) -> u8 {
    if score == SCORE_DRAW {
        OUTCOME_DRAW
    } else {
        (score.abs() - SCORE_BASE + 1) as u8
    }
}

// Do not pass a terminal ply to this function.
pub fn evaluate(ply: &Ply, tablebase: Option<&TablebaseIndex>) -> u8 {
    evaluate_with_mask(ply, None, tablebase)
}

// Do not pass a terminal ply to this function.
// Optimized version that accepts remaining_pieces_mask to avoid recomputation.
// remaining_pieces_mask should be the mask of pieces still available AFTER placing ply.piece_to_place.
pub fn evaluate_with_remaining_pieces(
    ply: &Ply,
    remaining_pieces_mask: u16,
    tablebase: Option<&TablebaseIndex>,
) -> u8 {
    evaluate_with_mask(ply, Some(remaining_pieces_mask), tablebase)
}

// Do not pass a terminal ply to this function.
fn evaluate_with_mask(
    ply: &Ply,
    remaining_pieces_opt: Option<u16>,
    tablebase: Option<&TablebaseIndex>,
) -> u8 {
    let layer = if *ply == LAYER_0_SENTINEL {
        0
    } else {
        ply.board.occupancy.count_ones() as usize + 1
    };

    let remaining_pieces_mask: u16 = if *ply == LAYER_0_SENTINEL {
        u16::MAX
    } else {
        remaining_pieces_opt.unwrap_or_else(|| compute_unused_pieces_mask(ply))
    };

    // Use negamax with alpha-beta pruning (work in score space)
    let alpha = i16::MIN + 1;
    let beta = i16::MAX;

    let score = negamax(ply, remaining_pieces_mask, layer, alpha, beta, tablebase);

    score_to_outcome(score)
}

/// Negamax with alpha-beta pruning with optional tablebase cutoffs.
/// Works entirely in linear score space for efficiency.
/// Returns the best score from the current player's perspective.
fn negamax(
    ply: &Ply,
    remaining_pieces_mask: u16,
    layer: usize,
    mut alpha: i16,
    beta: i16,
    tablebase: Option<&TablebaseIndex>,
) -> i16 {
    // Check tablebase first if available
    if let Some(tb) = tablebase {
        let available = tb.available_layers();
        if available[layer] {
            let canonical_ply = canonicalize_ply(ply);
            if let Some(outcome) = tb.query(&canonical_ply) {
                if outcome == OUTCOME_DRAW {
                    return SCORE_DRAW;
                }
                let score = outcome_to_score(outcome);
                let current_player_parity = (layer % 2) as u8;
                let outcome_parity = outcome % 2;
                // If outcome winner matches current player, it's our win (positive)
                // Layer parity: 0=white's turn (even layers), 1=black's turn (odd layers)
                // Outcome parity: 1=white wins (odd outcomes), 0=black wins (even outcomes)
                // So if outcome_parity != current_player_parity, current player wins
                return if outcome_parity != current_player_parity {
                    score // Our win
                } else {
                    -score // Opponent's win
                };
            }
        }
    }

    if layer == 0 {
        return negamax(
            &LAYER_1_CANONICAL,
            remaining_pieces_mask ^ 1,
            layer,
            alpha,
            beta,
            tablebase,
        );
    }

    // Terminal position check (layer 16 = 15 pieces on board)
    if layer == 16 {
        let (_, is_quarto) =
            check_after_place(&ply.board, ply.piece_to_place, !ply.board.occupancy);
        return if is_quarto {
            outcome_to_score(layer_to_outcome(layer + 1))
        } else {
            SCORE_DRAW
        };
    }

    // Pass 1: Check for immediate wins (move ordering)
    let mut empty_squares_mask: u16 = !ply.board.occupancy;
    while empty_squares_mask != 0 {
        let square = lowest_bit(empty_squares_mask);
        empty_squares_mask ^= square;

        let (_, is_quarto) = check_after_place(&ply.board, ply.piece_to_place, square);
        if is_quarto {
            return outcome_to_score(layer_to_outcome(layer + 1));
        }
    }

    // Pass 2: No immediate wins, evaluate all moves with alpha-beta pruning
    let mut best_score = i16::MIN + 1;
    empty_squares_mask = !ply.board.occupancy;
    let mut processed = Vec::new();

    while empty_squares_mask != 0 {
        let square = lowest_bit(empty_squares_mask);
        empty_squares_mask ^= square;

        let (child_board, _) = check_after_place(&ply.board, ply.piece_to_place, square);

        // Evaluate each possible piece assignment for this square
        let mut pieces_to_assign = remaining_pieces_mask;

        while pieces_to_assign != 0 {
            let piece_mask = lowest_bit(pieces_to_assign);
            pieces_to_assign ^= piece_mask;
            let piece_index: u8 = piece_mask.trailing_zeros() as u8;

            let next_ply = Ply {
                board: child_board,
                piece_to_place: piece_index,
            };

            // Skip if we've already processed this canonical ply
            let canonical_next_ply = canonicalize_ply(&next_ply);
            if processed.contains(&canonical_next_ply) {
                continue;
            }
            processed.push(canonical_next_ply);

            let opponent_score = negamax(
                &next_ply,
                remaining_pieces_mask ^ piece_mask,
                layer + 1,
                -beta,
                -alpha,
                tablebase,
            );

            let score = -opponent_score;

            best_score = best_score.max(score);
            alpha = alpha.max(score);

            if alpha >= beta {
                return best_score; // Beta cutoff
            }
        }
    }

    best_score
}

#[inline(always)]
fn lowest_bit(mask: u16) -> u16 {
    mask & mask.wrapping_neg()
}

/// Compute a 16-bit mask of unused pieces (1 bit per piece-id 0..15).
/// We iterate occupied squares and reconstruct each placed piece-id from the 4 attribute masks.
/// Any seen piece toggles its bit off from an initial all-ones mask.
#[inline]
pub fn compute_unused_pieces_mask(ply: &Ply) -> u16 {
    let mut unused_mask: u16 = u16::MAX;
    let mut occupied: u16 = ply.board.occupancy;

    let piece_to_place_bit: u16 = 1u16 << ply.piece_to_place;
    unused_mask ^= piece_to_place_bit;

    while occupied != 0 {
        let square = lowest_bit(occupied);
        occupied ^= square;

        // Rebuild the piece id by checking which attribute masks hit this square.
        let mut piece_id: u8 = 0;
        if (ply.board.attribute_masks[0] & square) != 0 {
            piece_id |= 1;
        }
        if (ply.board.attribute_masks[1] & square) != 0 {
            piece_id |= 2;
        }
        if (ply.board.attribute_masks[2] & square) != 0 {
            piece_id |= 4;
        }
        if (ply.board.attribute_masks[3] & square) != 0 {
            piece_id |= 8;
        }

        let piece_bit: u16 = 1u16 << piece_id;
        unused_mask ^= piece_bit;
    }

    unused_mask
}
