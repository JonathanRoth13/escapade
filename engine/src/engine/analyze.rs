use super::parsing::{format_node_grid, format_node_hex};
use crate::core::{
    Board, DEPTH_0_SENTINEL, Node, canonicalize, compute_unused_pieces_mask, evaluate,
    outcome_to_sort_score,
};
use crate::core::{LINE_MASKS, check_line_mask};
use crate::tablebase::TablebaseIndex;
use serde::Serialize;

#[derive(Serialize)]
pub struct Move {
    #[serde(skip_serializing_if = "Option::is_none")]
    square: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    piece: Option<u8>,
    hex: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    quarto: Option<Vec<Quarto>>,
}

#[derive(Serialize, Clone)]
pub struct Quarto {
    intersection: [u8; 4],
    attribute: String,
}

#[derive(Serialize)]
pub struct Orbit {
    canon_hex: String,
    outcome: u8,
    moves: Vec<Move>,
}

#[derive(Serialize)]
pub struct AnalysisResult {
    ply_grid: String,
    ply_hex: String,
    canon_hex: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    quartos: Option<Vec<Quarto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orbits: Option<Vec<Orbit>>,
}

struct MoveCandidate {
    square: Option<u8>,
    piece: Option<u8>,
    quartos: Option<Vec<Quarto>>,
    resulting_node: Node,
    canonical_node: Node,
}

struct IntermediateBoard {
    board: Board,
    square_placed: u8,
}

pub fn sort_orbits(orbits: &mut [Orbit], is_second_player: bool) {
    orbits.sort_by(|a, b| {
        let score_a = outcome_to_sort_score(a.outcome, is_second_player);
        let score_b = outcome_to_sort_score(b.outcome, is_second_player);
        score_b
            .cmp(&score_a)
            .then_with(|| a.canon_hex.cmp(&b.canon_hex))
    });
}

pub fn analyze(node: &Node, tablebase: Option<&TablebaseIndex>) -> String {
    let depth = if *node == DEPTH_0_SENTINEL {
        0
    } else {
        node.board.occupancy.count_ones() as usize + 1
    };

    let is_player_first: bool = depth % 2 == 0;

    let ply_grid = format_node_grid(node);
    let ply_hex = format_node_hex(node);

    let canonical_node = canonicalize(node);
    let canon_hex = format_node_hex(&canonical_node);

    if depth > 4 {
        let mut quarto: Vec<Quarto> = Vec::with_capacity(3);

        for mask in &LINE_MASKS {
            if check_line_mask(&node.board, *mask) {
                let quarto_squares = extract_square_indices(*mask);
                let quarto_attributes = get_quarto_attribute(&node.board, *mask);
                for quarto_attribute in quarto_attributes {
                    quarto.push(Quarto {
                        intersection: quarto_squares,
                        attribute: quarto_attribute,
                    });
                }
            }
        }

        if !quarto.is_empty() {
            let result = AnalysisResult {
                ply_grid,
                ply_hex,
                canon_hex,
                quartos: Some(quarto),
                orbits: None,
            };
            return serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string());
        }
    }

    let move_candidates: Vec<MoveCandidate> = match depth {
        0 => get_moves_depth_0(),
        _ => get_moves_general(node, tablebase, depth),
    };

    let mut orbits = Vec::new();
    let mut processed = Vec::new();

    for candidate in &move_candidates {
        if processed.contains(&candidate.canonical_node) {
            continue;
        }
        processed.push(candidate.canonical_node);

        let mut candidates_in_orbit: Vec<&MoveCandidate> = move_candidates
            .iter()
            .filter(|c| c.canonical_node == candidate.canonical_node)
            .collect();

        candidates_in_orbit.sort_by(|a, b| a.resulting_node.cmp(&b.resulting_node));

        let moves: Vec<Move> = candidates_in_orbit
            .iter()
            .map(|c| Move {
                square: c.square,
                piece: c.piece,
                hex: format_node_hex(&c.resulting_node),
                quarto: c.quartos.clone(),
            })
            .collect();

        let canon_hex = format_node_hex(&candidate.canonical_node);

        let outcome = if candidate.quartos.is_some() {
            (17 - depth) as u8
        } else {
            evaluate(&candidate.canonical_node, tablebase)
        };

        orbits.push(Orbit {
            canon_hex,
            outcome,
            moves,
        });
    }

    sort_orbits(&mut orbits, is_player_first);

    let result = AnalysisResult {
        ply_grid,
        ply_hex,
        canon_hex,
        quartos: None,
        orbits: Some(orbits),
    };

    serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string())
}

fn get_moves_depth_0() -> Vec<MoveCandidate> {
    let mut moves = Vec::with_capacity(16);
    let empty_board = Board {
        occupancy: 0,
        attribute_masks: [0, 0, 0, 0],
    };

    for piece_index in 0..16 {
        moves.push(MoveCandidate {
            square: None,
            piece: Some(piece_index),
            resulting_node: Node {
                board: empty_board,
                piece_to_place: piece_index,
            },
            canonical_node: Node {
                board: empty_board,
                piece_to_place: 0,
            },
            quartos: None,
        });
    }
    moves
}

fn get_moves_general(
    node: &Node,
    _tablebase: Option<&TablebaseIndex>,
    depth: usize,
) -> Vec<MoveCandidate> {
    let mut moves = Vec::with_capacity(16 * 16);
    let mut empty_squares_mask = !node.board.occupancy;
    let mut non_terminal_boards = Vec::with_capacity(16);
    let available_pieces_mask = compute_unused_pieces_mask(node);

    'next_square: while empty_squares_mask != 0 {
        let square_mask = lowest_bit(empty_squares_mask);
        let square_index = square_mask.trailing_zeros() as u8;
        empty_squares_mask ^= square_mask;

        let board_after_placement =
            crate::core::evaluation::place(&node.board, node.piece_to_place, square_mask);
        let line_masks = crate::core::LINE_MASKS_INDEX[square_index as usize];

        let mut quartos: Vec<Quarto> = Vec::with_capacity(3);
        for &line_mask in line_masks {
            if check_line_mask(&board_after_placement, line_mask) {
                let quarto_squares = extract_square_indices(line_mask);
                let quarto_attributes = get_quarto_attribute(&board_after_placement, line_mask);
                for quarto_attribute in quarto_attributes {
                    quartos.push(Quarto {
                        intersection: quarto_squares,
                        attribute: quarto_attribute,
                    });
                }
            }
        }

        if !quartos.is_empty() || depth == 16 {
            let terminal_node = Node {
                board: board_after_placement,
                piece_to_place: 0,
            };
            let canonical_node = canonicalize(&terminal_node);
            moves.push(MoveCandidate {
                square: Some(square_index),
                piece: None,
                quartos: if quartos.is_empty() {
                    None
                } else {
                    Some(quartos)
                },
                resulting_node: terminal_node,
                canonical_node,
            });
            continue 'next_square;
        }

        non_terminal_boards.push(IntermediateBoard {
            board: board_after_placement,
            square_placed: square_index,
        });
    }

    let mut remaining_pieces_mask = available_pieces_mask;
    while remaining_pieces_mask != 0 {
        let piece_mask = lowest_bit(remaining_pieces_mask);
        let piece_index = piece_mask.trailing_zeros() as u8;
        remaining_pieces_mask ^= piece_mask;

        for intermediate in &non_terminal_boards {
            let resulting_node = Node {
                board: intermediate.board,
                piece_to_place: piece_index,
            };
            let canonical_node = canonicalize(&resulting_node);

            moves.push(MoveCandidate {
                square: Some(intermediate.square_placed),
                piece: Some(piece_index),
                quartos: None,
                resulting_node,
                canonical_node,
            });
        }
    }

    moves
}

fn extract_square_indices(line_mask: u16) -> [u8; 4] {
    let mut squares = [0u8; 4];
    let mut mask = line_mask;
    let mut index = 0;

    while mask != 0 {
        let square_mask = lowest_bit(mask);
        mask ^= square_mask;
        squares[index] = square_mask.trailing_zeros() as u8;
        index += 1;
    }

    squares
}

fn get_quarto_attribute(board: &Board, line_mask: u16) -> Vec<String> {
    // Bit encoding:
    // bit 0 (value 1): Hollow (1) vs Solid (0)
    // bit 1 (value 2): Short (1) vs Tall (0)
    // bit 2 (value 4): Square (1) vs Round (0)
    // bit 3 (value 8): Dark (1) vs Light (0)
    let mut attributes = Vec::new();

    for attr in 0..4 {
        let attr_mask = board.attribute_masks[attr];
        if (attr_mask & line_mask) == line_mask {
            attributes.push(match attr {
                0 => "hollow".to_string(),
                1 => "short".to_string(),
                2 => "square".to_string(),
                3 => "dark".to_string(),
                _ => unreachable!(),
            });
        } else if (attr_mask & line_mask) == 0 {
            attributes.push(match attr {
                0 => "solid".to_string(),
                1 => "tall".to_string(),
                2 => "round".to_string(),
                3 => "light".to_string(),
                _ => unreachable!(),
            });
        }
    }

    attributes
}

#[inline(always)]
fn lowest_bit(mask: u16) -> u16 {
    mask & mask.wrapping_neg()
}
