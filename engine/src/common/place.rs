use crate::common::board::Board;
use crate::common::line_masks::{
    COL_0, COL_1, COL_2, COL_3, DIAG_ANTI, DIAG_MAIN, ROW_0, ROW_1, ROW_2, ROW_3,
};

use crate::common::check_line_mask::check_line_mask;

pub const LINE_MASKS_00: [u16; 3] = [ROW_0, COL_0, DIAG_MAIN];
pub const LINE_MASKS_01: [u16; 2] = [ROW_0, COL_1];
pub const LINE_MASKS_02: [u16; 2] = [ROW_0, COL_2];
pub const LINE_MASKS_03: [u16; 3] = [ROW_0, COL_3, DIAG_ANTI];

pub const LINE_MASKS_04: [u16; 2] = [ROW_1, COL_0];
pub const LINE_MASKS_05: [u16; 3] = [ROW_1, COL_1, DIAG_MAIN];
pub const LINE_MASKS_06: [u16; 3] = [ROW_1, COL_2, DIAG_ANTI];
pub const LINE_MASKS_07: [u16; 2] = [ROW_1, COL_3];

pub const LINE_MASKS_08: [u16; 2] = [ROW_2, COL_0];
pub const LINE_MASKS_09: [u16; 3] = [ROW_2, COL_1, DIAG_ANTI];
pub const LINE_MASKS_10: [u16; 3] = [ROW_2, COL_2, DIAG_MAIN];
pub const LINE_MASKS_11: [u16; 2] = [ROW_2, COL_3];

pub const LINE_MASKS_12: [u16; 3] = [ROW_3, COL_0, DIAG_ANTI];
pub const LINE_MASKS_13: [u16; 2] = [ROW_3, COL_1];
pub const LINE_MASKS_14: [u16; 2] = [ROW_3, COL_2];
pub const LINE_MASKS_15: [u16; 3] = [ROW_3, COL_3, DIAG_MAIN];

pub const LINE_MASKS_INDEX: [&[u16]; 16] = [
    &LINE_MASKS_00,
    &LINE_MASKS_01,
    &LINE_MASKS_02,
    &LINE_MASKS_03,
    &LINE_MASKS_04,
    &LINE_MASKS_05,
    &LINE_MASKS_06,
    &LINE_MASKS_07,
    &LINE_MASKS_08,
    &LINE_MASKS_09,
    &LINE_MASKS_10,
    &LINE_MASKS_11,
    &LINE_MASKS_12,
    &LINE_MASKS_13,
    &LINE_MASKS_14,
    &LINE_MASKS_15,
];

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
