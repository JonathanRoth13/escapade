// ┏━━━┯━━━┯━━━┯━━━┓
// ┃   │   │   │   ┃
// ┠───┼───┼───┼───┨
// ┃   │   │   │   ┃
// ┠───┼───┼───┼───┨
// ┃   │   │   │   ┃
// ┠───┼───┼───┼───┨
// ┃ X │ X │ X │ X ┃
// ┗━━━┷━━━┷━━━┷━━━┛
pub const ROW_0: u16 = 0x000F;

// ┏━━━┯━━━┯━━━┯━━━┓
// ┃   │   │   │   ┃
// ┠───┼───┼───┼───┨
// ┃   │   │   │   ┃
// ┠───┼───┼───┼───┨
// ┃ X │ X │ X │ X ┃
// ┠───┼───┼───┼───┨
// ┃   │   │   │   ┃
// ┗━━━┷━━━┷━━━┷━━━┛
pub const ROW_1: u16 = 0x00F0;

// ┏━━━┯━━━┯━━━┯━━━┓
// ┃   │   │   │   ┃
// ┠───┼───┼───┼───┨
// ┃ X │ X │ X │ X ┃
// ┠───┼───┼───┼───┨
// ┃   │   │   │   ┃
// ┠───┼───┼───┼───┨
// ┃   │   │   │   ┃
// ┗━━━┷━━━┷━━━┷━━━┛
pub const ROW_2: u16 = 0x0F00;

// ┏━━━┯━━━┯━━━┯━━━┓
// ┃ X │ X │ X │ X ┃
// ┠───┼───┼───┼───┨
// ┃   │   │   │   ┃
// ┠───┼───┼───┼───┨
// ┃   │   │   │   ┃
// ┠───┼───┼───┼───┨
// ┃   │   │   │   ┃
// ┗━━━┷━━━┷━━━┷━━━┛
pub const ROW_3: u16 = 0xF000;

// ┏━━━┯━━━┯━━━┯━━━┓
// ┃ X │   │   │   ┃
// ┠───┼───┼───┼───┨
// ┃ X │   │   │   ┃
// ┠───┼───┼───┼───┨
// ┃ X │   │   │   ┃
// ┠───┼───┼───┼───┨
// ┃ X │   │   │   ┃
// ┗━━━┷━━━┷━━━┷━━━┛
pub const COL_0: u16 = 0x1111;

// ┏━━━┯━━━┯━━━┯━━━┓
// ┃   │ X │   │   ┃
// ┠───┼───┼───┼───┨
// ┃   │ X │   │   ┃
// ┠───┼───┼───┼───┨
// ┃   │ X │   │   ┃
// ┠───┼───┼───┼───┨
// ┃   │ X │   │   ┃
// ┗━━━┷━━━┷━━━┷━━━┛
pub const COL_1: u16 = 0x2222;

// ┏━━━┯━━━┯━━━┯━━━┓
// ┃   │   │ X │   ┃
// ┠───┼───┼───┼───┨
// ┃   │   │ X │   ┃
// ┠───┼───┼───┼───┨
// ┃   │   │ X │   ┃
// ┠───┼───┼───┼───┨
// ┃   │   │ X │   ┃
// ┗━━━┷━━━┷━━━┷━━━┛
pub const COL_2: u16 = 0x4444;

// ┏━━━┯━━━┯━━━┯━━━┓
// ┃   │   │   │ X ┃
// ┠───┼───┼───┼───┨
// ┃   │   │   │ X ┃
// ┠───┼───┼───┼───┨
// ┃   │   │   │ X ┃
// ┠───┼───┼───┼───┨
// ┃   │   │   │ X ┃
// ┗━━━┷━━━┷━━━┷━━━┛
pub const COL_3: u16 = 0x8888;

// ┏━━━┯━━━┯━━━┯━━━┓
// ┃   │   │   │ X ┃
// ┠───┼───┼───┼───┨
// ┃   │   │ X │   ┃
// ┠───┼───┼───┼───┨
// ┃   │ X │   │   ┃
// ┠───┼───┼───┼───┨
// ┃ X │   │   │   ┃
// ┗━━━┷━━━┷━━━┷━━━┛
pub const DIAG_MAIN: u16 = 0x8421;

// ┏━━━┯━━━┯━━━┯━━━┓
// ┃ X │   │   │   ┃
// ┠───┼───┼───┼───┨
// ┃   │ X │   │   ┃
// ┠───┼───┼───┼───┨
// ┃   │   │ X │   ┃
// ┠───┼───┼───┼───┨
// ┃   │   │   │ X ┃
// ┗━━━┷━━━┷━━━┷━━━┛
pub const DIAG_ANTI: u16 = 0x1248;

pub const LINE_MASKS: [u16; 10] = [
    ROW_0, ROW_1, ROW_2, ROW_3, COL_0, COL_1, COL_2, COL_3, DIAG_MAIN, DIAG_ANTI,
];

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

use super::Board;

#[inline(always)]
pub fn check_line_mask(board: &Board, line_mask: u16) -> bool {
    if (board.occupancy & line_mask) != line_mask {
        return false;
    }

    let a0 = board.attribute_masks[0] & line_mask;
    if a0 == 0 || a0 == line_mask {
        return true;
    }

    let a1 = board.attribute_masks[1] & line_mask;
    if a1 == 0 || a1 == line_mask {
        return true;
    }

    let a2 = board.attribute_masks[2] & line_mask;
    if a2 == 0 || a2 == line_mask {
        return true;
    }

    let a3 = board.attribute_masks[3] & line_mask;
    if a3 == 0 || a3 == line_mask {
        return true;
    }

    false
}
