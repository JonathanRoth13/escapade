use crate::core::{Node, hash_bucket_node, hash_phi_bytes};
use anyhow::{Result, anyhow};
use memmap2::Mmap;
use std::fs::File;
use std::path::Path;

const MAGIC: [u8; 4] = *b"ESCQ";
const VERSION: u8 = 1;
const HEADER_SIZE: usize = 18;

pub struct TablebaseFile {
    mmap: Mmap,

    // Header fields (parsed from first 34 bytes)
    pub depth: u8,
    pub shard_id: u8,
    pub shard_bits: u8,

    // Cached derived values (avoid recomputing)
    bucket_mask: u64,
    slot_mask: u64,
    outcomes_offset: usize,
}

impl TablebaseFile {
    /// Open and mmap a tablebase file
    pub fn open(path: &Path) -> Result<Self> {
        // Open file
        let file = File::open(path)
            .map_err(|e| anyhow!("Failed to open tablebase file {}: {}", path.display(), e))?;

        // Memory-map the file
        let mmap = unsafe {
            Mmap::map(&file)
                .map_err(|e| anyhow!("Failed to mmap tablebase file {}: {}", path.display(), e))?
        };

        // Parse header (inline, matches index/mod.rs writing format)
        if mmap.len() < HEADER_SIZE {
            return Err(anyhow!(
                "File too small: {} bytes (expected at least {})",
                mmap.len(),
                HEADER_SIZE
            ));
        }

        let mut offset = 0;

        // Magic (4 bytes)
        let magic = [mmap[0], mmap[1], mmap[2], mmap[3]];
        offset += 4;
        if magic != MAGIC {
            return Err(anyhow!(
                "Invalid magic number: {:?} (expected {:?})",
                std::str::from_utf8(&magic).unwrap_or("<invalid>"),
                std::str::from_utf8(&MAGIC).unwrap()
            ));
        }

        // Version (1 byte)
        let version = mmap[offset];
        offset += 1;
        if version != VERSION {
            return Err(anyhow!(
                "Unsupported version: {} (expected {})",
                version,
                VERSION
            ));
        }

        // Depth (1 byte)
        let depth = mmap[offset];
        offset += 1;
        if depth > 16 {
            return Err(anyhow!("Invalid depth: {} (must be 0-16)", depth));
        }

        // Shard ID (1 byte)
        let shard_id = mmap[offset];
        offset += 1;

        // Shard bits (1 byte)
        let shard_bits = mmap[offset];
        offset += 1;

        // Validate shard_id is within range (num_shards = 2^shard_bits)
        let num_shards = 1u16 << shard_bits;
        if shard_id >= num_shards as u8 {
            return Err(anyhow!(
                "Invalid shard_id: {} (must be < {} for shard_bits={})",
                shard_id,
                num_shards,
                shard_bits
            ));
        }

        // Bucket bits (1 byte)
        let bucket_bits = mmap[offset];
        offset += 1;

        // Slot bits (1 byte)
        let slot_bits = mmap[offset];
        offset += 1;

        // Num keys (8 bytes, little-endian) - not currently used
        let _num_keys = u64::from_le_bytes([
            mmap[offset],
            mmap[offset + 1],
            mmap[offset + 2],
            mmap[offset + 3],
            mmap[offset + 4],
            mmap[offset + 5],
            mmap[offset + 6],
            mmap[offset + 7],
        ]);
        offset += 8;

        assert_eq!(offset, HEADER_SIZE);

        // Calculate num_buckets and num_slots from bits
        let num_buckets = 1u64 << bucket_bits;
        let num_slots = 1u64 << slot_bits;

        // Verify file size
        let sigma_table_size = (num_buckets * 2) as usize;
        let outcomes_table_size = num_slots.div_ceil(2) as usize;
        let expected_size = HEADER_SIZE + sigma_table_size + outcomes_table_size;

        if mmap.len() != expected_size {
            return Err(anyhow!(
                "File size mismatch: {} bytes (expected {})",
                mmap.len(),
                expected_size
            ));
        }

        // Calculate cached values
        let bucket_mask = (1u64 << bucket_bits) - 1;
        let slot_mask = (1u64 << slot_bits) - 1;
        let outcomes_offset = HEADER_SIZE + sigma_table_size;

        Ok(Self {
            mmap,
            depth,
            shard_id,
            shard_bits,
            bucket_mask,
            slot_mask,
            outcomes_offset,
        })
    }

    /// Query outcome for a position (4-bit value 0-15)
    pub fn query(&self, node: &Node) -> u8 {
        let key = node.to_bytes();

        // 1. Hash (bucket) to get bucket_id
        let hash_bucket = hash_bucket_node(node);
        let bucket_id = hash_bucket & self.bucket_mask;

        // 2. Read sigma value (u16) for this bucket
        let sigma = self.read_sigma(bucket_id);

        // 3. Hash (phi) with sigma to get slot index
        let hash_phi = hash_phi_bytes(&key, sigma as u64);
        let slot = hash_phi & self.slot_mask;

        // 4. Read 4-bit outcome from slot
        self.read_outcome(slot)
    }

    /// Read sigma value (u16) for a bucket
    fn read_sigma(&self, bucket_id: u64) -> u16 {
        let offset = HEADER_SIZE + (bucket_id as usize * 2);
        u16::from_le_bytes([self.mmap[offset], self.mmap[offset + 1]])
    }

    /// Read 4-bit outcome from a slot
    fn read_outcome(&self, slot: u64) -> u8 {
        let byte_offset = self.outcomes_offset + (slot as usize / 2);
        let byte = self.mmap[byte_offset];

        if slot.is_multiple_of(2) {
            byte >> 4 // Upper 4 bits
        } else {
            byte & 0x0F // Lower 4 bits
        }
    }

    /// Get file size in bytes
    pub fn size(&self) -> usize {
        self.mmap.len()
    }
}

impl std::fmt::Debug for TablebaseFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TablebaseFile")
            .field("depth", &self.depth)
            .field("shard_id", &self.shard_id)
            .field("shard_bits", &self.shard_bits)
            .field("size", &self.size())
            .finish()
    }
}
