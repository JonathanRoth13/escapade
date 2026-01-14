mod board;
mod canon;
mod canonical_transformations;
mod check_line_mask;
mod evaluate;
mod hashing;
mod line_masks;
mod machine_specs;
mod occupancy_masks;
mod place;
mod ply;
mod ply_display;
mod random_ply;
mod record;
mod transform;
mod validation;

pub use board::{Board, ROOT_BOARD};
pub use canon::{canonicalize_ply, is_ply_canonical_under_attribute_relabeling_only};
pub use canonical_transformations::CANONICAL_TRANSFORMATIONS;
pub use check_line_mask::check_line_mask;
pub use evaluate::{
    compute_unused_pieces_mask, evaluate, evaluate_with_remaining_pieces, outcome_to_sort_score,
};
pub use hashing::{
    hash_bucket_bytes, hash_bucket_ply, hash_phi_bytes, hash_shard_ply, hash_worker_ply,
};
pub use line_masks::LINE_MASKS;
pub use machine_specs::MachineSpecs;
pub use occupancy_masks::INDEX as OCCUPANCY_MASKS;
pub use place::LINE_MASKS_INDEX;
pub use place::{check_after_place, place};
pub use ply::{LAYER_0_SENTINEL, LAYER_1_CANONICAL, Ply};
pub use ply_display::{format_ply, format_ply_hex, parse_ply, pretty_print_ply};
pub use random_ply::generate_random_ply;
pub use record::Record;
pub use transform::{Transformation, apply_u16, is_canonical_occupancy};
pub use validation::validate_ply;
