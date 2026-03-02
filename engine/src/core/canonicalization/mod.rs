mod canonical_transformations;
#[allow(clippy::module_inception)]
mod canonicalization;
mod transform;

pub use canonical_transformations::CANONICAL_TRANSFORMATIONS;
pub use canonicalization::{canonicalize, is_node_canonical_under_attribute_relabeling_only};
