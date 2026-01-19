use crate::core::{Board, DEPTH_0_SENTINEL, LINE_MASKS, Node, check_line_mask};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    /// The same piece ID appears multiple times on the board
    DuplicatePieceId,

    /// An attribute mask has bits set outside the occupancy mask
    AttributeMaskOutOfBounds,

    /// Multiple non-intersecting quartos exist on the board
    NonIntersectingQuartos,

    /// The piece_to_place is already on the board
    PieceToPlaceAlreadyUsed,

    /// Terminal position must have piece_to_place = 0
    TerminalPositionInvalidPiece,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::DuplicatePieceId => {
                write!(f, "The same piece ID appears multiple times on the board")
            }
            ValidationError::AttributeMaskOutOfBounds => {
                write!(f, "Attribute mask has bits set outside occupancy")
            }
            ValidationError::NonIntersectingQuartos => {
                write!(f, "Board has multiple non-intersecting quartos")
            }
            ValidationError::PieceToPlaceAlreadyUsed => {
                write!(f, "The piece_to_place is already on the board")
            }
            ValidationError::TerminalPositionInvalidPiece => {
                write!(f, "Terminal position must have piece_to_place = 0")
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// Check if a board has multiple quartos that dont share a common cell
fn check_quarto_intersection(board: &Board) -> (bool, bool) {
    let mut there_exists_at_least_one_quarto = false;
    let mut intersection_mask: u16 = 0;

    for mask in LINE_MASKS {
        if check_line_mask(board, mask) {
            if there_exists_at_least_one_quarto {
                intersection_mask &= mask;
                if intersection_mask == 0 {
                    return (false, false);
                }
            } else {
                there_exists_at_least_one_quarto = true;
                intersection_mask = mask;
            }
        }
    }
    (true, there_exists_at_least_one_quarto)
}

/// Validate that a board's structure is consistent
pub fn validate_board(board: &Board) -> Result<(), ValidationError> {
    // Check that attribute masks are subsets of occupancy
    for attr_mask in &board.attribute_masks {
        if (attr_mask & !board.occupancy) != 0 {
            return Err(ValidationError::AttributeMaskOutOfBounds);
        }
    }

    // Check for duplicate piece IDs
    let mut used_pieces: u16 = 0;
    let mut remaining_occupied = board.occupancy;

    while remaining_occupied != 0 {
        let cell_bit = remaining_occupied & remaining_occupied.wrapping_neg();
        remaining_occupied ^= cell_bit;

        let mut piece_id: u8 = 0;
        for (i, &attr_mask) in board.attribute_masks.iter().enumerate() {
            if (attr_mask & cell_bit) != 0 {
                piece_id |= 1u8 << i;
            }
        }

        let piece_bit = 1u16 << piece_id;
        if (used_pieces & piece_bit) != 0 {
            return Err(ValidationError::DuplicatePieceId);
        }
        used_pieces |= piece_bit;
    }

    // Check for non-intersecting quartos
    let (is_valid, _has_quarto) = check_quarto_intersection(board);
    if !is_valid {
        return Err(ValidationError::NonIntersectingQuartos);
    }

    Ok(())
}

/// Validate a node (board + piece_to_place)
pub fn validate_node(node: &Node) -> Result<(), ValidationError> {
    if *node == DEPTH_0_SENTINEL {
        return Ok(());
    }

    validate_board(&node.board)?;

    // Check if this is a terminal position (has a quarto)
    let has_quarto = LINE_MASKS
        .iter()
        .any(|&mask| check_line_mask(&node.board, mask));

    if has_quarto {
        // Terminal positions must have piece_to_place = 0
        if node.piece_to_place != 0 {
            return Err(ValidationError::TerminalPositionInvalidPiece);
        }
    } else {
        // Non-terminal: check that piece_to_place is not already used on the board
        let mut remaining_occupied = node.board.occupancy;

        while remaining_occupied != 0 {
            let cell_bit = remaining_occupied & remaining_occupied.wrapping_neg();
            remaining_occupied ^= cell_bit;

            let mut piece_id: u8 = 0;
            for (i, &attr_mask) in node.board.attribute_masks.iter().enumerate() {
                if (attr_mask & cell_bit) != 0 {
                    piece_id |= 1u8 << i;
                }
            }

            if piece_id == node.piece_to_place {
                return Err(ValidationError::PieceToPlaceAlreadyUsed);
            }
        }
    }

    Ok(())
}
