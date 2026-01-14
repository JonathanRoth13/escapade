use crate::common::Board;

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
