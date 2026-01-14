use crate::common::{
    Board, LAYER_0_SENTINEL, Ply, canonicalize_ply, compute_unused_pieces_mask, evaluate,
    format_ply, format_ply_hex, outcome_to_sort_score,
};
use crate::common::{LINE_MASKS, check_line_mask};
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
    resulting_ply: Ply,
    canonical_ply: Ply,
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

fn number_to_word(n: usize) -> &'static str {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        _ => unreachable!(),
    }
}

pub fn format_outcome(outcome: u8, is_player_first: bool, current_layer: usize) -> String {
    if outcome == 15 {
        return "The game ends in a draw.".to_string();
    }

    let first_player_wins = outcome % 2 == 1;

    let player_wins = if is_player_first {
        first_player_wins
    } else {
        !first_player_wins
    };

    let winner = if player_wins { "player" } else { "opponent" };

    let layer_that_the_game_ends_on = 18 - outcome as usize;

    let moves_remaining = layer_that_the_game_ends_on - (current_layer + 1);

    if moves_remaining == 0 {
        return format!("The {} wins.", winner);
    }

    let move_word = if moves_remaining == 1 {
        "move"
    } else {
        "moves"
    };

    format!(
        "The {} wins in {} {}, on move {}.",
        winner,
        number_to_word(moves_remaining),
        move_word,
        layer_that_the_game_ends_on
    )
}

pub fn analyze(ply: &Ply, tablebase: Option<&TablebaseIndex>) -> String {
    // Calculate layer
    let layer = if *ply == LAYER_0_SENTINEL {
        0
    } else {
        ply.board.occupancy.count_ones() as usize + 1
    };

    // Always from current player's perspective
    let is_player_first: bool = layer % 2 == 0;
    let is_players_turn: bool = true;

    // Format input ply
    let ply_grid = format_ply(ply);
    let ply_hex = format_ply_hex(ply);

    // Canonicalize and format
    let canonical_ply = canonicalize_ply(ply);
    let canon_grid = format_ply(&canonical_ply);
    let canon_hex = format_ply_hex(&canonical_ply);



    if layer > 4 {
        // Check if this is a terminal position (skip for LAYER_0_SENTINEL)
        let mut quarto: Vec<Quarto> = Vec::with_capacity(3);

        for mask in &LINE_MASKS {
            if check_line_mask(&ply.board, *mask) {
                let quarto_squares = extract_square_indices(*mask);
                let quarto_attributes = get_quarto_attribute(&ply.board, *mask);
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

    let move_candidates: Vec<MoveCandidate> = match layer {
        0 => get_moves_layer_0(),
        _ => get_moves_general(ply, tablebase, layer),
    };

    // Convert move_candidates to orbits by grouping by canonical_ply
    let mut orbits = Vec::new();
    let mut processed = Vec::new();

    for candidate in &move_candidates {
        // Skip if we've already processed this canonical ply
        if processed.contains(&candidate.canonical_ply) {
            continue;
        }
        processed.push(candidate.canonical_ply);

        // Collect all candidates with this canonical ply and sort by resulting_ply
        let mut candidates_in_orbit: Vec<&MoveCandidate> = move_candidates
            .iter()
            .filter(|c| c.canonical_ply == candidate.canonical_ply)
            .collect();

        candidates_in_orbit.sort_by(|a, b| a.resulting_ply.cmp(&b.resulting_ply));

        let moves: Vec<Move> = candidates_in_orbit
            .iter()
            .map(|c| {
                // Determine whose turn it is from perspective
                let actor = if is_players_turn {
                    "player"
                } else {
                    "opponent"
                };

                Move {
                    square: c.square,
                    piece: c.piece,
                    hex: format_ply_hex(&c.resulting_ply),
                    quarto: c.quartos.clone(),
                }
            })
            .collect();

        let canon_hex = format_ply_hex(&candidate.canonical_ply);

        // Check if this is a terminal position (has quarto)
        let outcome = if candidate.quartos.is_some() {
            // Terminal position: game ends after this move
            (17 - layer) as u8
        } else {
            evaluate(&candidate.canonical_ply, tablebase)
        };

        orbits.push(Orbit {
            canon_hex,
            outcome,
            moves,
        });
    }

    // Sort orbits before returning
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

fn get_moves_layer_0() -> Vec<MoveCandidate> {
    let mut moves = Vec::with_capacity(16);
    let empty_board = Board {
        occupancy: 0,
        attribute_masks: [0, 0, 0, 0],
    };

    for piece_index in 0..16 {
        moves.push(MoveCandidate {
            square: None,
            piece: Some(piece_index),
            resulting_ply: Ply {
                board: empty_board,
                piece_to_place: piece_index,
            },
            canonical_ply: Ply {
                board: empty_board,
                piece_to_place: 0,
            },
            quartos: None,
        });
    }
    moves
}

fn get_moves_general(ply: &Ply, _tablebase: Option<&TablebaseIndex>,layer:usize) -> Vec<MoveCandidate> {
    let mut moves = Vec::with_capacity(16 * 16);
    let mut empty_squares_mask = !ply.board.occupancy;
    let mut non_terminal_boards = Vec::with_capacity(16);
    let available_pieces_mask = compute_unused_pieces_mask(ply);

    // Phase 1: Try placing piece_to_place on each empty square
    'next_square: while empty_squares_mask != 0 {
        let square_mask = lowest_bit(empty_squares_mask);
        let square_index = square_mask.trailing_zeros() as u8;
        empty_squares_mask ^= square_mask;

        let board_after_placement =
            crate::common::place(&ply.board, ply.piece_to_place, square_mask);
        let line_masks = crate::common::LINE_MASKS_INDEX[square_index as usize];

        let mut quartos: Vec<Quarto> = Vec::with_capacity(3);
        for &line_mask in line_masks {
            if crate::common::check_line_mask(&board_after_placement, line_mask) {
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

        if !quartos.is_empty() {
            let terminal_ply = Ply {
                board: board_after_placement,
                piece_to_place: 0,
            };
            let canonical_ply = canonicalize_ply(&terminal_ply);
            moves.push(MoveCandidate {
                square: Some(square_index),
                piece: None,
                quartos: Some(quartos),
                resulting_ply: terminal_ply,
                canonical_ply,
            });
            continue 'next_square;
        }

        // claude, when you get the chance, combine this with the above logic block
        if layer == 16 {
            let terminal_ply = Ply {
                board: board_after_placement,
                piece_to_place: 0,
            };
            let canonical_ply = canonicalize_ply(&terminal_ply);
            moves.push(MoveCandidate {
                square: Some(square_index),
                piece: None,
                quartos: None,
                resulting_ply: terminal_ply,
                canonical_ply,
            });
            continue 'next_square;
        }

        // Non-terminal position - save for phase 2
        non_terminal_boards.push(IntermediateBoard {
            board: board_after_placement,
            square_placed: square_index,
        });
    }

    // Phase 2: For non-terminal boards, pair with each available piece
    let mut remaining_pieces_mask = available_pieces_mask;
    while remaining_pieces_mask != 0 {
        let piece_mask = lowest_bit(remaining_pieces_mask);
        let piece_index = piece_mask.trailing_zeros() as u8;
        remaining_pieces_mask ^= piece_mask;

        for intermediate in &non_terminal_boards {
            let resulting_ply = Ply {
                board: intermediate.board,
                piece_to_place: piece_index,
            };
            let canonical_ply = canonicalize_ply(&resulting_ply);

            moves.push(MoveCandidate {
                square: Some(intermediate.square_placed),
                piece: Some(piece_index),
                quartos: None,
                resulting_ply,
                canonical_ply,
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

fn get_quarto_description(intersection: &[u8; 4], attribute: &String) -> String {
    format!(
        "The pieces on squares {}, {}, {} and {} are all {}.",
        intersection[0], intersection[1], intersection[2], intersection[3], attribute
    )
    .to_string()
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
        // Check if all 4 pieces have the same value for this attribute
        if (attr_mask & line_mask) == line_mask {
            // All pieces have this bit set
            attributes.push(match attr {
                0 => "hollow".to_string(),
                1 => "short".to_string(),
                2 => "square".to_string(),
                3 => "dark".to_string(),
                _ => unreachable!(),
            });
        } else if (attr_mask & line_mask) == 0 {
            // All pieces have this bit unset
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
