use super::board::Board;
use super::node::Node;

/// Serialization format (11 bytes):
/// - Bytes 0-1: Occupancy mask (u16, big-endian)
/// - Bytes 2-9: Four attribute masks (4 × u16, big-endian)
/// - Byte 10: Upper 4 bits = piece_to_place, Lower 4 bits = outcome
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Record {
    pub node: Node,
    pub outcome: u8, // Only lower 4 bits used (0-15)
}

impl Record {
    #[inline]
    pub fn to_bytes(self) -> [u8; 11] {
        let b = self.node.board.to_bytes();
        // Pack piece_to_place (upper 4 bits) and outcome (lower 4 bits) into one byte
        let packed = (self.node.piece_to_place << 4) | (self.outcome & 0x0F);
        [
            b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], packed,
        ]
    }

    #[inline]
    pub fn from_bytes(x: &[u8; 11]) -> Self {
        Self {
            node: Node {
                board: Board::from_bytes(x[0..10].try_into().unwrap()),
                piece_to_place: x[10] >> 4, // Upper 4 bits
            },
            outcome: x[10] & 0x0F, // Lower 4 bits
        }
    }

    /// Deserialize a Record from a byte slice.
    pub fn from_byte_slice(bytes: &[u8]) -> Self {
        assert!(
            bytes.len() == 11,
            "expected 11 bytes for Record, got {}",
            bytes.len()
        );
        Self {
            node: Node {
                board: Board::from_bytes(bytes[0..10].try_into().unwrap()),
                piece_to_place: bytes[10] >> 4, // Upper 4 bits
            },
            outcome: bytes[10] & 0x0F, // Lower 4 bits
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
