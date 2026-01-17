const DOMAIN_SHARD: u64 = 0x7f754f191742d00f;
const DOMAIN_BUCKET: u64 = 0x97b750923ceb3ffd;
const DOMAIN_PHI_1: u64 = 0x0f825a9f1ecf8543;
const DOMAIN_PHI_2: u64 = 0x3136e519e64f64f1;
const DOMAIN_WORKER: u64 = 0x97b750923ceb3ffd;

use xxhash_rust::xxh3::xxh3_64_with_seed;

use crate::common::Ply;

/// Hash for shard selection
pub fn hash_shard_ply(ply: &Ply) -> u64 {
    xxh3_64_with_seed(&ply.to_bytes(), DOMAIN_SHARD)
}

/// Hash for MPH bucket selection within a shard
pub fn hash_bucket_bytes(key: &[u8; 11]) -> u64 {
    xxh3_64_with_seed(key, DOMAIN_BUCKET)
}

/// Hash for MPH bucket selection within a shard
pub fn hash_bucket_ply(ply: &Ply) -> u64 {
    hash_bucket_bytes(&ply.to_bytes())
}

/// MPH phi function hash
pub fn hash_phi_bytes(key: &[u8; 11], l: u64) -> u64 {
    xxh3_64_with_seed(key, DOMAIN_PHI_1)
        .wrapping_add(l.wrapping_mul(xxh3_64_with_seed(key, DOMAIN_PHI_2) | 1))
}

/// Hash for worker partitioning during solve/count
pub fn hash_worker_ply(ply: &Ply) -> u64 {
    xxh3_64_with_seed(&ply.to_bytes(), DOMAIN_WORKER)
}
