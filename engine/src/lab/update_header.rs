use anyhow::{Result, anyhow};
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

const MAGIC: [u8; 4] = *b"ESCQ";
const VERSION: u8 = 1;
const LAYER_OFFSET: u64 = 5;
const SHARD_ID_OFFSET: u64 = 6;
const SHARD_BITS_OFFSET: u64 = 7;

pub fn run(index_path: PathBuf, layer: u8, shard_id: u8, shard_bits: u8) -> Result<()> {
    if layer > 16 {
        return Err(anyhow!("Invalid layer: {} (must be 0-16)", layer));
    }

    if shard_bits > 16 {
        return Err(anyhow!("Invalid shard_bits: {} (must be 0-16)", shard_bits));
    }

    let num_shards = 1u16 << shard_bits;
    if shard_id >= num_shards as u8 {
        return Err(anyhow!(
            "Invalid shard_id: {} (must be < {} for shard_bits={})",
            shard_id,
            num_shards,
            shard_bits
        ));
    }

    // Open file for reading and writing
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&index_path)?;

    // Read and validate header
    let mut header = [0u8; 8];
    file.read_exact(&mut header)?;

    // Validate magic
    if header[0..4] != MAGIC {
        return Err(anyhow!(
            "Invalid magic number: {:?} (expected {:?})",
            std::str::from_utf8(&header[0..4]).unwrap_or("<invalid>"),
            std::str::from_utf8(&MAGIC).unwrap()
        ));
    }

    // Validate version
    if header[4] != VERSION {
        return Err(anyhow!(
            "Unsupported version: {} (expected {})",
            header[4],
            VERSION
        ));
    }

    let old_layer = header[5];
    let old_shard_id = header[6];
    let old_shard_bits = header[7];

    // Update layer
    file.seek(SeekFrom::Start(LAYER_OFFSET))?;
    file.write_all(&[layer])?;

    // Update shard_id
    file.seek(SeekFrom::Start(SHARD_ID_OFFSET))?;
    file.write_all(&[shard_id])?;

    // Update shard_bits
    file.seek(SeekFrom::Start(SHARD_BITS_OFFSET))?;
    file.write_all(&[shard_bits])?;

    file.flush()?;

    println!(
        "{}: layer {} -> {}, shard_id {} -> {}, shard_bits {} -> {}",
        index_path.display(),
        old_layer,
        layer,
        old_shard_id,
        shard_id,
        old_shard_bits,
        shard_bits
    );

    Ok(())
}
