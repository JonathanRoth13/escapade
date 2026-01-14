use crate::common::board::Board;
use crate::common::ply::Ply;

/// Serialization format (11 bytes):
/// - Bytes 0-1: Occupancy mask (u16, big-endian)
/// - Bytes 2-9: Four attribute masks (4 × u16, big-endian)
/// - Byte 10: Upper 4 bits = piece_to_place, Lower 4 bits = outcome
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Record {
    pub ply: Ply,
    pub outcome: u8, // Only lower 4 bits used (0-15)
}

impl Record {
    #[inline]
    pub fn to_bytes(self) -> [u8; 11] {
        let p = u16::to_be_bytes(self.ply.board.occupancy);
        let a3 = u16::to_be_bytes(self.ply.board.attribute_masks[3]);
        let a2 = u16::to_be_bytes(self.ply.board.attribute_masks[2]);
        let a1 = u16::to_be_bytes(self.ply.board.attribute_masks[1]);
        let a0 = u16::to_be_bytes(self.ply.board.attribute_masks[0]);

        // Pack piece_to_place (upper 4 bits) and outcome (lower 4 bits) into one byte
        let packed = (self.ply.piece_to_place << 4) | (self.outcome & 0x0F);

        [
            p[0], p[1], a3[0], a3[1], a2[0], a2[1], a1[0], a1[1], a0[0], a0[1], packed,
        ]
    }

    #[inline]
    pub fn from_bytes(x: &[u8; 11]) -> Self {
        let packed = x[10];
        let piece_to_place = packed >> 4; // Upper 4 bits
        let outcome = packed & 0x0F; // Lower 4 bits

        let occupancy = u16::from_be_bytes([x[0], x[1]]);
        let a3 = u16::from_be_bytes([x[2], x[3]]);
        let a2 = u16::from_be_bytes([x[4], x[5]]);
        let a1 = u16::from_be_bytes([x[6], x[7]]);
        let a0 = u16::from_be_bytes([x[8], x[9]]);

        Self {
            ply: Ply {
                board: Board {
                    occupancy,
                    attribute_masks: [a0, a1, a2, a3],
                },
                piece_to_place,
            },
            outcome,
        }
    }

    /// Deserialize a Record from a byte slice.
    ///
    /// This function accepts a variable-length slice and validates it's exactly 11 bytes.
    /// For fixed-size arrays, use `from_bytes()` or the `From<[u8; 11]>` trait instead.
    ///
    /// # Panics
    /// Panics if the slice length is not exactly 11 bytes.
    pub fn from_byte_slice(bytes: &[u8]) -> Self {
        assert!(
            bytes.len() == 11,
            "expected 11 bytes for Record, got {}",
            bytes.len()
        );

        let packed = bytes[10];
        let piece_to_place = packed >> 4; // Upper 4 bits
        let outcome = packed & 0x0F; // Lower 4 bits

        let occupancy = u16::from_be_bytes([bytes[0], bytes[1]]);
        let a3 = u16::from_be_bytes([bytes[2], bytes[3]]);
        let a2 = u16::from_be_bytes([bytes[4], bytes[5]]);
        let a1 = u16::from_be_bytes([bytes[6], bytes[7]]);
        let a0 = u16::from_be_bytes([bytes[8], bytes[9]]);

        Self {
            ply: Ply {
                board: Board {
                    occupancy,
                    attribute_masks: [a0, a1, a2, a3],
                },
                piece_to_place,
            },
            outcome,
        }
    }
}

impl From<Record> for [u8; 11] {
    #[inline]
    fn from(r: Record) -> Self {
        r.to_bytes()
    }
}

impl From<&Record> for [u8; 11] {
    #[inline]
    fn from(r: &Record) -> Self {
        r.to_bytes()
    }
}

impl From<[u8; 11]> for Record {
    #[inline]
    fn from(x: [u8; 11]) -> Self {
        Record::from_bytes(&x)
    }
}

impl From<&[u8; 11]> for Record {
    #[inline]
    fn from(x: &[u8; 11]) -> Self {
        Record::from_bytes(x)
    }
}

impl From<&[u8]> for Record {
    #[inline]
    fn from(x: &[u8]) -> Self {
        Record::from_byte_slice(x)
    }
}
