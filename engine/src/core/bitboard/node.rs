use super::board::Board;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node {
    pub board: Board,
    pub piece_to_place: u8,
}

impl Node {
    #[inline]
    pub fn to_bytes(self) -> [u8; 11] {
        let b = self.board.to_bytes();
        [
            b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9],
            self.piece_to_place << 4, // Store in upper 4 bits
        ]
    }

    #[inline]
    pub fn from_bytes(x: &[u8; 11]) -> Self {
        Self {
            board: Board::from_bytes(x[0..10].try_into().unwrap()),
            piece_to_place: x[10] >> 4, // Extract from upper 4 bits
        }
    }

    pub fn from_byte_slice(bytes: &[u8]) -> Self {
        assert!(
            bytes.len() == 11,
            "expected 11 bytes for Node, got {}",
            bytes.len()
        );
        Self {
            board: Board::from_bytes(bytes[0..10].try_into().unwrap()),
            piece_to_place: bytes[10] >> 4, // Extract from upper 4 bits
        }
    }
}

impl From<Node> for [u8; 11] {
    #[inline]
    fn from(p: Node) -> Self {
        p.to_bytes()
    }
}

impl From<&Node> for [u8; 11] {
    #[inline]
    fn from(p: &Node) -> Self {
        p.to_bytes()
    }
}

impl From<[u8; 11]> for Node {
    #[inline]
    fn from(x: [u8; 11]) -> Self {
        Node::from_bytes(&x)
    }
}

impl From<&[u8; 11]> for Node {
    #[inline]
    fn from(x: &[u8; 11]) -> Self {
        Node::from_bytes(x)
    }
}

impl From<&[u8]> for Node {
    #[inline]
    fn from(x: &[u8]) -> Self {
        Node::from_byte_slice(x)
    }
}

/// Used to represent the root position.
pub const DEPTH_0_SENTINEL: Node = Node {
    board: Board {
        occupancy: 0xFFFF,
        attribute_masks: [0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF],
    },
    piece_to_place: 0xFF,
};

/// The only canonical move in depth 1
pub const DEPTH_1_CANONICAL: Node = Node {
    board: Board {
        occupancy: 0,
        attribute_masks: [0, 0, 0, 0],
    },
    piece_to_place: 0,
};
