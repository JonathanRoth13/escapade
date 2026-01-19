use crate::core::bitboard::line_masks::{check_line_mask, LINE_MASKS_INDEX};
use crate::core::bitboard::Board;

#[inline(always)]
pub fn place(board: &Board, attribute: u8, cell: u16) -> Board {
    let occupancy = board.occupancy | cell;
    let mut attribute_masks = board.attribute_masks;

    if (attribute & 1) != 0 {
        attribute_masks[0] |= cell;
    }
    if (attribute & 2) != 0 {
        attribute_masks[1] |= cell;
    }
    if (attribute & 4) != 0 {
        attribute_masks[2] |= cell;
    }
    if (attribute & 8) != 0 {
        attribute_masks[3] |= cell;
    }

    Board {
        occupancy,
        attribute_masks,
    }
}

#[inline]
pub fn check_after_place(board: &Board, attribute: u8, cell: u16) -> (Board, bool) {
    let next_board: Board = place(board, attribute, cell);
    let line_masks: &[u16] = LINE_MASKS_INDEX[cell.trailing_zeros() as usize];
    for mask in line_masks {
        if check_line_mask(&next_board, *mask) {
            return (next_board, true);
        }
    }
    (next_board, false)
}
