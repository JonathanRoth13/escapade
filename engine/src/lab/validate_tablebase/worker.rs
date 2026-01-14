use crate::common::{Ply, canonicalize_ply, evaluate, generate_random_ply};
use crate::tablebase::TablebaseIndex;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::time::Instant;

/// Validation result for a single position
#[derive(Debug)]
pub struct ValidationError {
    pub ply: Ply,
    pub expected: u8, // From original evaluation
    pub actual: u8,   // From tablebase
}

/// Worker that validates tablebase results
pub fn validate_positions(
    worker_id: u32,
    total_workers: u32,
    layer: usize,
    target_samples: usize,
    rng_seed: u64,
    tablebase: &TablebaseIndex,
) -> (Vec<ValidationError>, usize, u64, u64) {
    let samples_per_worker = target_samples.div_ceil(total_workers as usize);
    let worker_seed = rng_seed.wrapping_add(worker_id as u64);
    let mut rng = StdRng::seed_from_u64(worker_seed);

    let mut errors = Vec::new();
    let mut total_original_nanos = 0u64;
    let mut total_tablebase_nanos = 0u64;

    for _ in 0..samples_per_worker {
        let ply = generate_random_ply(layer, &mut rng);

        // Evaluate with original method
        let start = Instant::now();
        let expected = evaluate(&ply, None);
        let original_nanos = start.elapsed().as_nanos() as u64;
        total_original_nanos += original_nanos;

        // Evaluate with tablebase method (canonicalize first)
        let start = Instant::now();
        let canonical_ply = canonicalize_ply(&ply);
        let actual_opt = tablebase.query(&canonical_ply);
        let tablebase_nanos = start.elapsed().as_nanos() as u64;
        total_tablebase_nanos += tablebase_nanos;

        // Check for mismatch (if lookup succeeded)
        if let Some(actual) = actual_opt {
            if expected != actual {
                errors.push(ValidationError {
                    ply,
                    expected,
                    actual,
                });
            }
        } else {
            // Tablebase lookup failed - this shouldn't happen for the target layer
            errors.push(ValidationError {
                ply,
                expected,
                actual: 255, // Sentinel value indicating lookup failure
            });
        }
    }

    (
        errors,
        samples_per_worker,
        total_original_nanos,
        total_tablebase_nanos,
    )
}
