use crate::core::{Node, hash_shard_node};
use anyhow::{Result, anyhow};
use std::ffi::OsStr;
use std::path::Path;

use super::file::TablebaseFile;

struct DepthData {
    shard_bits: u8,
    files: Vec<TablebaseFile>, // length = 2^shard_bits, indexed by shard_id
}

pub struct TablebaseIndex {
    depths: [Option<DepthData>; 18],
}

impl TablebaseIndex {
    /// Load all tablebase files from directory
    pub fn load_from_dir(path: &Path) -> Result<Self> {
        Self::load_from_dir_impl(path, false)
    }

    /// Load all tablebase files from directory (silent mode)
    pub fn load_from_dir_silent(path: &Path) -> Result<Self> {
        Self::load_from_dir_impl(path, true)
    }

    fn load_from_dir_impl(path: &Path, silent: bool) -> Result<Self> {
        let mut temp_depths: [Option<(u8, Vec<TablebaseFile>)>; 18] = [const { None }; 18];

        for entry_result in std::fs::read_dir(path)? {
            let entry = entry_result?;
            let file_path = entry.path();

            if file_path.extension() != Some(OsStr::new("bin")) {
                continue;
            }

            let file = match TablebaseFile::open(&file_path) {
                Ok(f) => f,
                Err(e) => {
                    if !silent {
                        eprintln!(
                            "Warning: Skipping invalid tablebase file {}: {}",
                            file_path.display(),
                            e
                        );
                    }
                    continue;
                }
            };

            let depth = file.depth as usize;

            match &mut temp_depths[depth] {
                Some((existing_shard_bits, files)) => {
                    if *existing_shard_bits != file.shard_bits {
                        return Err(anyhow!(
                            "Inconsistent shard_bits for depth {}: found {} and {}",
                            depth,
                            existing_shard_bits,
                            file.shard_bits
                        ));
                    }
                    if files.iter().any(|f| f.shard_id == file.shard_id) {
                        return Err(anyhow!(
                            "Duplicate tablebase file for depth {} shard {}",
                            depth,
                            file.shard_id
                        ));
                    }
                    files.push(file);
                }
                None => {
                    temp_depths[depth] = Some((file.shard_bits, vec![file]));
                }
            }
        }

        let mut depths: [Option<DepthData>; 18] = [const { None }; 18];

        for (depth_idx, temp_data) in temp_depths.into_iter().enumerate() {
            if let Some((shard_bits, files_vec)) = temp_data {
                let num_shards = 1usize << shard_bits;

                // Create array of correct size, initially None
                let mut files: Vec<Option<TablebaseFile>> = (0..num_shards).map(|_| None).collect();

                // Place files at correct shard_id positions
                for file in files_vec {
                    let shard_id = file.shard_id as usize;
                    if shard_id >= num_shards {
                        return Err(anyhow!(
                            "Invalid shard_id {} for depth {} (shard_bits={})",
                            shard_id,
                            depth_idx,
                            shard_bits
                        ));
                    }
                    files[shard_id] = Some(file);
                }

                // Check that all shards are present
                let files: Vec<TablebaseFile> = files
                    .into_iter()
                    .enumerate()
                    .map(|(idx, opt)| {
                        opt.ok_or_else(|| {
                            anyhow!(
                                "Missing shard {} for depth {} (need all {} shards)",
                                idx,
                                depth_idx,
                                num_shards
                            )
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;

                depths[depth_idx] = Some(DepthData { shard_bits, files });
            }
        }

        Ok(Self { depths })
    }

    pub fn available_depths(&self) -> [bool; 18] {
        let mut available = [false; 18];
        for (idx, depth) in self.depths.iter().enumerate() {
            available[idx] = depth.is_some();
        }
        available
    }

    /// Calculate total memory usage of all loaded tablebase files
    pub fn memory_usage(&self) -> usize {
        self.depths
            .iter()
            .filter_map(|depth| depth.as_ref())
            .flat_map(|depth_data| &depth_data.files)
            .map(|file| file.size())
            .sum()
    }

    pub fn query(&self, node: &Node) -> Option<u8> {
        let depth = node.board.occupancy.count_ones() as usize + 1;

        if depth > 16 {
            return None;
        }

        // Get depth data (returns None if no tablebase for this depth)
        let depth_data = self.depths[depth].as_ref()?;

        // Determine shard ID
        let shard_id = if depth_data.shard_bits == 0 {
            0
        } else {
            let shard_hash = hash_shard_node(node);
            (shard_hash >> (64 - depth_data.shard_bits)) as usize
        };

        // Get file for shard_id (returns None if out of bounds)
        let file = depth_data.files.get(shard_id)?;

        Some(file.query(node))
    }
}
