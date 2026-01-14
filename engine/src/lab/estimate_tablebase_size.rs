// Tablebase file format constants (must match index/mod.rs)
const HEADER_SIZE: u64 = 18;
const SIGMA_BYTES_PER_BUCKET: u64 = 2; // u16 per bucket
const DEFAULT_TARGET_BUCKET_SIZE: u64 = 16;
// Note: Outcomes use 4 bits per slot (2 slots per byte)

pub fn run(positions: u64) {
    // Calculate r = 2^⌈log2(n/t)⌉
    let ratio = positions as f64 / DEFAULT_TARGET_BUCKET_SIZE as f64;
    let bucket_bits = ratio.log2().ceil() as u32;
    let num_buckets: u64 = 1u64 << bucket_bits;

    // Calculate m = number of slots (multiply by 2 for ~50% load factor)
    let slots_needed = (positions as f64 * 2.0).ceil() as u64;
    let slot_bits = (slots_needed as f64).log2().ceil() as u32;
    let num_slots: u64 = 1u64 << slot_bits;

    // Calculate sizes (must match index/mod.rs)
    let sigma_bytes: u64 = num_buckets * SIGMA_BYTES_PER_BUCKET;
    let outcome_bytes: u64 = num_slots.div_ceil(2); // 4 bits per slot, round up
    let total_bytes: u64 = HEADER_SIZE + sigma_bytes + outcome_bytes;

    let gib = total_bytes as f64 / (1024.0 * 1024.0 * 1024.0);

    println!("Positions:      {}", positions);
    println!("Num buckets:    {} (2^{})", num_buckets, bucket_bits);
    println!(
        "Num slots:      {} (2^{}) [load factor: {:.2}%]",
        num_slots,
        slot_bits,
        100.0 * positions as f64 / num_slots as f64
    );
    println!();
    println!("Header:         {} bytes", HEADER_SIZE);
    println!(
        "Sigma table:    {} bytes ({:.2} GiB)",
        sigma_bytes,
        sigma_bytes as f64 / (1024.0 * 1024.0 * 1024.0)
    );
    println!(
        "Outcomes:       {} bytes ({:.2} GiB)",
        outcome_bytes,
        outcome_bytes as f64 / (1024.0 * 1024.0 * 1024.0)
    );
    println!("Total:          {} bytes ({:.2} GiB)", total_bytes, gib);
}
