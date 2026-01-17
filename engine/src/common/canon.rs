use crate::common::board::Board;
use crate::common::ply::Ply;
use crate::common::transform::apply;
use crate::common::{CANONICAL_TRANSFORMATIONS, LAYER_0_SENTINEL, LAYER_1_CANONICAL};

/// Helper function to orient a single attribute mask - don't use on empty board
#[inline(always)]
fn orient_attribute(occupancy: u16, attribute_mask: u16, piece_to_place_bit: u8) -> (u16, u8) {
    let attribute_mask_inverted = occupancy ^ attribute_mask;

    if attribute_mask_inverted < attribute_mask {
        (attribute_mask_inverted, piece_to_place_bit ^ 1)
    } else {
        (attribute_mask, piece_to_place_bit)
    }
}

/// Apply attribute relabeling canonicalization to a ply
#[inline(always)]
pub fn apply_attribute_relabeling(ply: &Ply) -> Ply {
    debug_assert!(ply.board.occupancy != 0);

    let mut oriented_attribute_masks = [0u16; 4];
    let mut oriented_piece_to_place_bits = [0u8; 4];
    for i in 0..4 {
        let piece_to_play_bit = (ply.piece_to_place >> i) & 1;
        let (oriented_attribute_mask, oriented_piece_to_play_bit) = orient_attribute(
            ply.board.occupancy,
            ply.board.attribute_masks[i],
            piece_to_play_bit,
        );
        oriented_attribute_masks[i] = oriented_attribute_mask;
        oriented_piece_to_place_bits[i] = oriented_piece_to_play_bit;
    }

    let mut idx = [0usize, 1, 2, 3];
    idx.sort_by_key(|&i| {
        (
            oriented_attribute_masks[i],
            oriented_piece_to_place_bits[i],
            i,
        )
    });

    let mut new_attribute_masks = [0u16; 4];
    let mut new_piece_to_place: u8 = 0;
    for (i, new_mask) in new_attribute_masks.iter_mut().enumerate() {
        let ii = 3 - i;
        *new_mask = oriented_attribute_masks[idx[ii]];
        new_piece_to_place |= oriented_piece_to_place_bits[idx[ii]] << i;
    }

    Ply {
        board: Board {
            occupancy: ply.board.occupancy,
            attribute_masks: new_attribute_masks,
        },
        piece_to_place: new_piece_to_place,
    }
}

/// Check if a ply is canonical under attribute relabeling only
#[inline(always)]
pub fn is_ply_canonical_under_attribute_relabeling_only(ply: &Ply) -> bool {
    debug_assert!(ply.board.occupancy != 0);

    for i in 0..4 {
        let attribute_mask_inverted = ply.board.occupancy ^ ply.board.attribute_masks[i];
        if attribute_mask_inverted < ply.board.attribute_masks[i] {
            return false;
        }
        for ii in (i + 1)..4 {
            match ply.board.attribute_masks[i].cmp(&ply.board.attribute_masks[ii]) {
                std::cmp::Ordering::Less => return false,
                std::cmp::Ordering::Equal => {
                    if ((ply.piece_to_place >> ii) & 1) > ((ply.piece_to_place >> i) & 1) {
                        return false;
                    }
                }
                std::cmp::Ordering::Greater => {}
            }
        }
    }
    true
}

/// Full ply canonicalization (both board transformations and attribute relabeling)
#[inline(always)]
pub fn canonicalize_ply(ply: &Ply) -> Ply {
    if *ply == LAYER_0_SENTINEL {
        return LAYER_0_SENTINEL;
    }
    if ply.board.occupancy == 0 {
        return LAYER_1_CANONICAL;
    }

    let transformations = CANONICAL_TRANSFORMATIONS[ply.board.occupancy as usize];

    let mut best_ply = *ply;
    for &t in transformations {
        let transformed_board = apply(&ply.board, t);
        let transformed_ply = Ply {
            board: transformed_board,
            piece_to_place: ply.piece_to_place,
        };

        let canonical_candidate = apply_attribute_relabeling(&transformed_ply);

        if canonical_candidate < best_ply {
            best_ply = canonical_candidate;
        }
    }

    best_ply
}
