pub mod bitboard;
pub mod canonicalization;
pub mod evaluation;

mod hashing;
mod machine_specs;
mod validation;

// Re-exports from bitboard
pub use bitboard::{
    Board, DEPTH_0_SENTINEL, DEPTH_1_CANONICAL, LINE_MASKS, LINE_MASKS_INDEX, Node,
    OCCUPANCY_MASKS, ROOT_BOARD, Record, check_line_mask,
};

// Re-exports from canonicalization
pub use canonicalization::{
    CANONICAL_TRANSFORMATIONS, canonicalize, is_node_canonical_under_attribute_relabeling_only,
};

// Re-exports from evaluation
pub use evaluation::{
    check_after_place, compute_unused_pieces_mask, evaluate, evaluate_with_remaining_pieces,
    outcome_to_sort_score,
};

// Re-exports from root modules
pub use hashing::{
    hash_bucket_bytes, hash_bucket_node, hash_phi_bytes, hash_shard_node, hash_worker_node,
};
pub use machine_specs::MachineSpecs;
pub use validation::validate_node;
