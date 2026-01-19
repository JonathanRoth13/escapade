use crate::core::bitboard::Board;
use crate::core::bitboard::node::Node;
use crate::core::canonicalization::transform::apply;
use crate::core::{CANONICAL_TRANSFORMATIONS, DEPTH_0_SENTINEL, DEPTH_1_CANONICAL};

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

/// Apply attribute relabeling canonicalization to a node
#[inline(always)]
pub fn apply_attribute_relabeling(node: &Node) -> Node {
    debug_assert!(node.board.occupancy != 0);

    let mut oriented_attribute_masks = [0u16; 4];
    let mut oriented_piece_to_place_bits = [0u8; 4];
    for i in 0..4 {
        let piece_to_play_bit = (node.piece_to_place >> i) & 1;
        let (oriented_attribute_mask, oriented_piece_to_play_bit) = orient_attribute(
            node.board.occupancy,
            node.board.attribute_masks[i],
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

    Node {
        board: Board {
            occupancy: node.board.occupancy,
            attribute_masks: new_attribute_masks,
        },
        piece_to_place: new_piece_to_place,
    }
}

/// Check if a node is canonical under attribute relabeling only
#[inline(always)]
pub fn is_node_canonical_under_attribute_relabeling_only(node: &Node) -> bool {
    debug_assert!(node.board.occupancy != 0);

    for i in 0..4 {
        let attribute_mask_inverted = node.board.occupancy ^ node.board.attribute_masks[i];
        if attribute_mask_inverted < node.board.attribute_masks[i] {
            return false;
        }
        for ii in (i + 1)..4 {
            match node.board.attribute_masks[i].cmp(&node.board.attribute_masks[ii]) {
                std::cmp::Ordering::Less => return false,
                std::cmp::Ordering::Equal => {
                    if ((node.piece_to_place >> ii) & 1) > ((node.piece_to_place >> i) & 1) {
                        return false;
                    }
                }
                std::cmp::Ordering::Greater => {}
            }
        }
    }
    true
}

/// Full node canonicalization (both board transformations and attribute relabeling)
#[inline(always)]
pub fn canonicalize(node: &Node) -> Node {
    if *node == DEPTH_0_SENTINEL {
        return DEPTH_0_SENTINEL;
    }
    if node.board.occupancy == 0 {
        return DEPTH_1_CANONICAL;
    }

    let transformations = CANONICAL_TRANSFORMATIONS[node.board.occupancy as usize];

    let mut best_node = *node;
    for &t in transformations {
        let transformed_board = apply(&node.board, t);
        let transformed_node = Node {
            board: transformed_board,
            piece_to_place: node.piece_to_place,
        };

        let canonical_candidate = apply_attribute_relabeling(&transformed_node);

        if canonical_candidate < best_node {
            best_node = canonical_candidate;
        }
    }

    best_node
}
