use std::cmp::{Ordering, PartialEq, PartialOrd};

pub const ROOT_BOARD: Board = Board {
    occupancy: 0,
    attribute_masks: [0, 0, 0, 0],
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Board {
    pub occupancy: u16,
    pub attribute_masks: [u16; 4],
}

impl Board {
    /// Serialize the board to 10 big-endian bytes: occupancy + 4 attribute masks.
    #[inline]
    pub fn to_bytes(self) -> [u8; 10] {
        let p = u16::to_be_bytes(self.occupancy);
        let a3 = u16::to_be_bytes(self.attribute_masks[3]);
        let a2 = u16::to_be_bytes(self.attribute_masks[2]);
        let a1 = u16::to_be_bytes(self.attribute_masks[1]);
        let a0 = u16::to_be_bytes(self.attribute_masks[0]);
        [p[0], p[1], a3[0], a3[1], a2[0], a2[1], a1[0], a1[1], a0[0], a0[1]]
    }

    /// Deserialize a board from 10 big-endian bytes.
    #[inline]
    pub fn from_bytes(x: &[u8; 10]) -> Self {
        Self {
            occupancy: u16::from_be_bytes([x[0], x[1]]),
            attribute_masks: [
                u16::from_be_bytes([x[8], x[9]]),
                u16::from_be_bytes([x[6], x[7]]),
                u16::from_be_bytes([x[4], x[5]]),
                u16::from_be_bytes([x[2], x[3]]),
            ],
        }
    }

    /// Extract the piece ID at a given square bitmask.
    /// The square must be a single set bit within the occupancy mask.
    #[inline(always)]
    pub fn piece_at(&self, square: u16) -> u8 {
        let mut piece_id: u8 = 0;
        for (i, &attr_mask) in self.attribute_masks.iter().enumerate() {
            if (attr_mask & square) != 0 {
                piece_id |= 1u8 << i;
            }
        }
        piece_id
    }
}

impl Ord for Board {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        (
            self.occupancy,
            self.attribute_masks[3],
            self.attribute_masks[2],
            self.attribute_masks[1],
            self.attribute_masks[0],
        )
            .cmp(&(
                other.occupancy,
                other.attribute_masks[3],
                other.attribute_masks[2],
                other.attribute_masks[1],
                other.attribute_masks[0],
            ))
    }
}

impl PartialOrd for Board {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
