use crate::{
    common::{LAYER_0_SENTINEL, LAYER_1_CANONICAL, Ply, canonicalize_ply, check_after_place},
    tablebase::TablebaseIndex,
};

// outcome is encoded as 1 through 13 and 15
// if the game ends on layer 17 with a tie, the outcome is 15
// otherwise, the outcome is calculated using the formula: outcome = 18-layer
const OUTCOME_DRAW: u8 = 15;
const SCORE_DRAW: i16 = 0;
const SCORE_BASE: i16 = 100;

#[inline]
fn outcome_to_score(outcome: u8) -> i16 {
    if outcome == OUTCOME_DRAW {
        SCORE_DRAW
    } else {
        (outcome - 1) as i16 + SCORE_BASE
    }
}

pub fn outcome_to_sort_score(outcome: u8, is_player_first: bool) -> i16 {
    if outcome == OUTCOME_DRAW {
        return SCORE_DRAW;
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

#[inline]
fn score_to_outcome(score: i16) -> u8 {
    if score == SCORE_DRAW {
        OUTCOME_DRAW
    } else {
        (score.abs() - SCORE_BASE + 1) as u8
    }
}

pub fn evaluate(ply: &Ply, tablebase: Option<&TablebaseIndex>) -> u8 {
    evaluate_with_mask(ply, None, tablebase)
}

pub fn evaluate_with_remaining_pieces(
    ply: &Ply,
    remaining_pieces_mask: u16,
    tablebase: Option<&TablebaseIndex>,
) -> u8 {
    evaluate_with_mask(ply, Some(remaining_pieces_mask), tablebase)
}

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

    let alpha = i16::MIN + 1;
    let beta = i16::MAX;

    let score = negamax(ply, remaining_pieces_mask, layer, alpha, beta, tablebase);

    score_to_outcome(score)
}

/// Negamax with alpha-beta pruning with optional tablebase cutoffs.
/// Returns the best score from the current player's perspective.
fn negamax(
    ply: &Ply,
    remaining_pieces_mask: u16,
    layer: usize,
    mut alpha: i16,
    beta: i16,
    tablebase: Option<&TablebaseIndex>,
) -> i16 {
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
                return if outcome_parity != current_player_parity {
                    score
                } else {
                    -score
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

    if layer == 16 {
        let (_, is_quarto) =
            check_after_place(&ply.board, ply.piece_to_place, !ply.board.occupancy);
        return if is_quarto {
            outcome_to_score((17 - layer) as u8)
        } else {
            SCORE_DRAW
        };
    }

    let mut empty_squares_mask: u16 = !ply.board.occupancy;
    while empty_squares_mask != 0 {
        let square = lowest_bit(empty_squares_mask);
        empty_squares_mask ^= square;

        let (_, is_quarto) = check_after_place(&ply.board, ply.piece_to_place, square);
        if is_quarto {
            return outcome_to_score((17 - layer) as u8);
        }
    }

    let mut best_score = i16::MIN + 1;
    empty_squares_mask = !ply.board.occupancy;
    let mut processed = Vec::new();

    while empty_squares_mask != 0 {
        let square = lowest_bit(empty_squares_mask);
        empty_squares_mask ^= square;

        let (child_board, _) = check_after_place(&ply.board, ply.piece_to_place, square);
        let mut pieces_to_assign = remaining_pieces_mask;

        while pieces_to_assign != 0 {
            let piece_mask = lowest_bit(pieces_to_assign);
            pieces_to_assign ^= piece_mask;
            let piece_index: u8 = piece_mask.trailing_zeros() as u8;

            let next_ply = Ply {
                board: child_board,
                piece_to_place: piece_index,
            };

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
                return best_score;
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
#[inline]
pub fn compute_unused_pieces_mask(ply: &Ply) -> u16 {
    let mut unused_mask: u16 = u16::MAX;
    let mut occupied: u16 = ply.board.occupancy;

    let piece_to_place_bit: u16 = 1u16 << ply.piece_to_place;
    unused_mask ^= piece_to_place_bit;

    while occupied != 0 {
        let square = lowest_bit(occupied);
        occupied ^= square;

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
