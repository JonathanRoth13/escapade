use crate::common::Board;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Transformation {
    Identity,
    Rotate90,
    Rotate180,
    Rotate270,
    ReflectHorizontal,
    ReflectVertical,
    ReflectDiagonalMain, // NW-SE diagonal (positions 0, 5, 10, 15)
    ReflectDiagonalAnti, // NE-SW diagonal (positions 3, 6, 9, 12)
}

impl Transformation {
    #[inline(always)]
    pub fn index(self) -> usize {
        match self {
            Transformation::Identity => 0,
            Transformation::Rotate90 => 1,
            Transformation::Rotate180 => 2,
            Transformation::Rotate270 => 3,
            Transformation::ReflectHorizontal => 4,
            Transformation::ReflectVertical => 5,
            Transformation::ReflectDiagonalMain => 6,
            Transformation::ReflectDiagonalAnti => 7,
        }
    }
}

const PERMUTATIONS: [[u8; 16]; 8] = [
    // Identity
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    // Rotation 90° clockwise
    [12, 8, 4, 0, 13, 9, 5, 1, 14, 10, 6, 2, 15, 11, 7, 3],
    // Rotation 180° clockwise
    [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
    // Rotation 270° clockwise
    [3, 7, 11, 15, 2, 6, 10, 14, 1, 5, 9, 13, 0, 4, 8, 12],
    // Horizontal reflection
    [12, 13, 14, 15, 8, 9, 10, 11, 4, 5, 6, 7, 0, 1, 2, 3],
    // Vertical reflection
    [3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12],
    // Diagonal NW–SE reflection
    [0, 4, 8, 12, 1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15],
    // Diagonal NE–SW reflection
    [15, 11, 7, 3, 14, 10, 6, 2, 13, 9, 5, 1, 12, 8, 4, 0],
];

#[inline(always)]
fn apply_permutation_u16(x: u16, perm: &[u8; 16]) -> u16 {
    let mut y: u16 = 0;
    y |= (x >> perm[0]) & 1;
    y |= ((x >> perm[1]) & 1) << 1;
    y |= ((x >> perm[2]) & 1) << 2;
    y |= ((x >> perm[3]) & 1) << 3;
    y |= ((x >> perm[4]) & 1) << 4;
    y |= ((x >> perm[5]) & 1) << 5;
    y |= ((x >> perm[6]) & 1) << 6;
    y |= ((x >> perm[7]) & 1) << 7;
    y |= ((x >> perm[8]) & 1) << 8;
    y |= ((x >> perm[9]) & 1) << 9;
    y |= ((x >> perm[10]) & 1) << 10;
    y |= ((x >> perm[11]) & 1) << 11;
    y |= ((x >> perm[12]) & 1) << 12;
    y |= ((x >> perm[13]) & 1) << 13;
    y |= ((x >> perm[14]) & 1) << 14;
    y |= ((x >> perm[15]) & 1) << 15;
    y
}

/// Apply a D₄ transform to a `Board` (position and the 4 attribute planes)
#[inline(always)]
pub fn apply(board: &Board, t: Transformation) -> Board {
    let perm = &PERMUTATIONS[t.index()];
    Board {
        occupancy: apply_permutation_u16(board.occupancy, perm),
        attribute_masks: [
            apply_permutation_u16(board.attribute_masks[0], perm),
            apply_permutation_u16(board.attribute_masks[1], perm),
            apply_permutation_u16(board.attribute_masks[2], perm),
            apply_permutation_u16(board.attribute_masks[3], perm),
        ],
    }
}
