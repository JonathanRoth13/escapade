use crate::common::{format_ply_hex, generate_random_ply};
use anyhow::Result;
use rand::SeedableRng;
use rand::rngs::StdRng;

pub fn run(layer: usize) -> Result<()> {
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    let mut rng = StdRng::seed_from_u64(seed);
    let ply = generate_random_ply(layer, &mut rng);
    let ply_string = format_ply_hex(&ply);

    println!("{}", ply_string);

    Ok(())
}
