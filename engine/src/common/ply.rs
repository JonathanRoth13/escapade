use crate::common::board::Board;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ply {
    pub board: Board,
    pub piece_to_place: u8,
}

impl Ply {
    #[inline]
    pub fn to_bytes(self) -> [u8; 11] {
        let p = u16::to_be_bytes(self.board.occupancy);
        let a3 = u16::to_be_bytes(self.board.attribute_masks[3]);
        let a2 = u16::to_be_bytes(self.board.attribute_masks[2]);
        let a1 = u16::to_be_bytes(self.board.attribute_masks[1]);
        let a0 = u16::to_be_bytes(self.board.attribute_masks[0]);
        [
            p[0],
            p[1],
            a3[0],
            a3[1],
            a2[0],
            a2[1],
            a1[0],
            a1[1],
            a0[0],
            a0[1],
            self.piece_to_place << 4, // Store in upper 4 bits
        ]
    }

    #[inline]
    pub fn from_bytes(x: &[u8; 11]) -> Self {
        let occupancy = u16::from_be_bytes([x[0], x[1]]);
        let a3 = u16::from_be_bytes([x[2], x[3]]);
        let a2 = u16::from_be_bytes([x[4], x[5]]);
        let a1 = u16::from_be_bytes([x[6], x[7]]);
        let a0 = u16::from_be_bytes([x[8], x[9]]);
        let piece_to_place = x[10] >> 4; // Extract from upper 4 bits

        Self {
            board: Board {
                occupancy,
                attribute_masks: [a0, a1, a2, a3],
            },
            piece_to_place,
        }
    }

    pub fn from_byte_slice(bytes: &[u8]) -> Self {
        assert!(
            bytes.len() == 11,
            "expected 11 bytes for Ply, got {}",
            bytes.len()
        );

        let occupancy = u16::from_be_bytes([bytes[0], bytes[1]]);
        let a3 = u16::from_be_bytes([bytes[2], bytes[3]]);
        let a2 = u16::from_be_bytes([bytes[4], bytes[5]]);
        let a1 = u16::from_be_bytes([bytes[6], bytes[7]]);
        let a0 = u16::from_be_bytes([bytes[8], bytes[9]]);
        let piece_to_place = bytes[10] >> 4; // Extract from upper 4 bits

        Self {
            board: Board {
                occupancy,
                attribute_masks: [a0, a1, a2, a3],
            },
            piece_to_place,
        }
    }
}

impl From<Ply> for [u8; 11] {
    #[inline]
    fn from(p: Ply) -> Self {
        p.to_bytes()
    }
}

impl From<&Ply> for [u8; 11] {
    #[inline]
    fn from(p: &Ply) -> Self {
        p.to_bytes()
    }
}

impl From<[u8; 11]> for Ply {
    #[inline]
    fn from(x: [u8; 11]) -> Self {
        Ply::from_bytes(&x)
    }
}

impl From<&[u8; 11]> for Ply {
    #[inline]
    fn from(x: &[u8; 11]) -> Self {
        Ply::from_bytes(x)
    }
}

impl From<&[u8]> for Ply {
    #[inline]
    fn from(x: &[u8]) -> Self {
        Ply::from_byte_slice(x)
    }
}

/// Used to represent the root position.
pub const LAYER_0_SENTINEL: Ply = Ply {
    board: Board {
        occupancy: 0xFFFF,
        attribute_masks: [0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF],
    },
    piece_to_place: 0xFF,
};

/// The only canonical move in layer 1
pub const LAYER_1_CANONICAL: Ply = Ply {
    board: Board {
        occupancy: 0,
        attribute_masks: [0, 0, 0, 0],
    },
    piece_to_place: 0,
};
