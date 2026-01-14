use crate::common::{Ply, hash_shard_ply};
use anyhow::{Result, anyhow};
use std::ffi::OsStr;
use std::path::Path;

use super::file::TablebaseFile;

struct LayerData {
    shard_bits: u8,
    files: Vec<TablebaseFile>, // length = 2^shard_bits, indexed by shard_id
}

pub struct TablebaseIndex {
    layers: [Option<LayerData>; 18], // indexed by layer (0-17)
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
        let mut temp_layers: [Option<(u8, Vec<TablebaseFile>)>; 18] = [const { None }; 18];

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

            let layer = file.layer as usize;

            match &mut temp_layers[layer] {
                Some((existing_shard_bits, files)) => {
                    if *existing_shard_bits != file.shard_bits {
                        return Err(anyhow!(
                            "Inconsistent shard_bits for layer {}: found {} and {}",
                            layer,
                            existing_shard_bits,
                            file.shard_bits
                        ));
                    }
                    if files.iter().any(|f| f.shard_id == file.shard_id) {
                        return Err(anyhow!(
                            "Duplicate tablebase file for layer {} shard {}",
                            layer,
                            file.shard_id
                        ));
                    }
                    files.push(file);
                }
                None => {
                    temp_layers[layer] = Some((file.shard_bits, vec![file]));
                }
            }
        }

        let mut layers: [Option<LayerData>; 18] = [const { None }; 18];

        for (layer_idx, temp_data) in temp_layers.into_iter().enumerate() {
            if let Some((shard_bits, files_vec)) = temp_data {
                let num_shards = 1usize << shard_bits;

                // Create array of correct size, initially None
                let mut files: Vec<Option<TablebaseFile>> = (0..num_shards).map(|_| None).collect();

                // Place files at correct shard_id positions
                for file in files_vec {
                    let shard_id = file.shard_id as usize;
                    if shard_id >= num_shards {
                        return Err(anyhow!(
                            "Invalid shard_id {} for layer {} (shard_bits={})",
                            shard_id,
                            layer_idx,
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
                                "Missing shard {} for layer {} (need all {} shards)",
                                idx,
                                layer_idx,
                                num_shards
                            )
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;

                layers[layer_idx] = Some(LayerData { shard_bits, files });
            }
        }

        Ok(Self { layers })
    }

    pub fn available_layers(&self) -> [bool; 18] {
        let mut available = [false; 18];
        for (idx, layer) in self.layers.iter().enumerate() {
            available[idx] = layer.is_some();
        }
        available
    }

    /// Calculate total memory usage of all loaded tablebase files
    pub fn memory_usage(&self) -> usize {
        self.layers
            .iter()
            .filter_map(|layer| layer.as_ref())
            .flat_map(|layer_data| &layer_data.files)
            .map(|file| file.size())
            .sum()
    }

    // todo - helper function that canonicalizes
    pub fn query(&self, ply: &Ply) -> Option<u8> {
        let layer = ply.board.occupancy.count_ones() as usize + 1;

        if layer > 16 {
            return None;
        }

        // Get layer data (returns None if no tablebase for this layer)
        let layer_data = self.layers[layer].as_ref()?;

        // Determine shard ID
        let shard_id = if layer_data.shard_bits == 0 {
            0
        } else {
            let shard_hash = hash_shard_ply(ply);
            (shard_hash >> (64 - layer_data.shard_bits)) as usize
        };

        // Get file for shard_id (returns None if out of bounds)
        let file = layer_data.files.get(shard_id)?;

        // Query the file
        Some(file.query(ply))
    }
}
