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
