use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use super::events::SolveEvent;
use super::shard_buffer::ShardBuffer;
use anyhow::{Context, Result};
use crossbeam_channel::Sender;

/// Per-thread writer context managing multiple shard buffers.
/// Spills largest buffers to disk when memory cap is exceeded.
pub struct WorkerContext {
    pub shard_bits: u8,
    cap_worker_bytes: usize,
    tmp_dir: PathBuf,
    pub worker_id: u16,
    shards: Vec<ShardBuffer>,
    total_buf_bytes: usize,
    filename_width: usize,
    global_total: Arc<AtomicU64>,
    local_position_count: u64,
    current_mask: usize,
    event_tx: Sender<SolveEvent>,
}

impl fmt::Debug for WorkerContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WorkerContext")
            .field("shard_bits", &self.shard_bits)
            .field("cap_worker_bytes", &self.cap_worker_bytes)
            .field("tmp_dir", &self.tmp_dir)
            .field("worker_id", &self.worker_id)
            .field("shards_len", &self.shards.len())
            .field("total_buf_bytes", &self.total_buf_bytes)
            .finish()
    }
}

impl WorkerContext {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        shard_bits: u8,
        cap_worker_bytes: usize,
        tmp_dir: PathBuf,
        worker_id: u16,
        starting_mask_index: usize,
        starting_position_count: u64,
        global_total: Arc<AtomicU64>,
        event_tx: Sender<SolveEvent>,
    ) -> Self {
        let n = 1usize
            .checked_shl(shard_bits as u32)
            .expect("shard_bits too large");
        let filename_width = if n <= 1 {
            1
        } else {
            ((n - 1) as u32).ilog10() as usize + 1
        };

        Self {
            shard_bits,
            cap_worker_bytes,
            tmp_dir,
            worker_id,
            shards: (0..n).map(|_| ShardBuffer::new()).collect(),
            total_buf_bytes: 0,
            filename_width,
            global_total,
            local_position_count: starting_position_count,
            current_mask: starting_mask_index,
            event_tx,
        }
    }

    #[inline(always)]
    pub fn append(&mut self, shard_id: u16, key: &[u8; 11]) -> Result<()> {
        // Batch global counter updates every 10,000 positions to reduce contention
        self.local_position_count += 1;
        if self.local_position_count.is_multiple_of(10_000) {
            self.global_total.fetch_add(10_000, Ordering::Relaxed);
        }

        let index = shard_id as usize;
        self.shards[index].append_fast(key);
        self.total_buf_bytes += 11;

        if self.total_buf_bytes > self.cap_worker_bytes {
            self.force_spill_until_under_cap()?;
        }

        Ok(())
    }

    /// Flush all shard buffers to disk
    pub fn flush_all(&mut self) -> Result<()> {
        let remainder = self.local_position_count % 10_000;
        if remainder > 0 {
            self.global_total.fetch_add(remainder, Ordering::Relaxed);
            // Reset to last reported batch boundary to avoid double-counting across masks
            self.local_position_count -= remainder;
        }

        for shard_id in 0..self.shards.len() {
            let path = self.shard_path(shard_id as u16);
            let mut shard_buffer = std::mem::take(&mut self.shards[shard_id]);
            let bytes_written = shard_buffer.buffered_len();
            shard_buffer.finalize(&path)?;
            self.emit_event(SolveEvent::ShardSpill {
                worker_id: self.worker_id,
                shard_id: shard_id as u16,
                mask_index: self.current_mask,
                bytes_written,
                timestamp: std::time::SystemTime::now(),
            });
        }
        self.total_buf_bytes = 0;
        Ok(())
    }

    #[inline]
    pub fn shard_id_from_hash(&self, hash64: u64) -> u16 {
        if self.shard_bits == 0 {
            0
        } else {
            (hash64 >> (64 - self.shard_bits)) as u16
        }
    }

    #[inline]
    fn shard_path(&self, shard_id: u16) -> PathBuf {
        self.tmp_dir.join(shard_id.to_string()).join(format!(
            "shard={:0width$}.mask={:04}.worker={:03}.bin.tmp",
            shard_id,
            self.current_mask(),
            self.worker_id,
            width = self.filename_width
        ))
    }

    /// Spill largest buffers to disk until under memory cap
    fn force_spill_until_under_cap(&mut self) -> Result<()> {
        while self.total_buf_bytes > self.cap_worker_bytes {
            let mut victim_shard_id: Option<u16> = None;
            let mut victim_length: usize = 0;

            for (i, shard_buffer) in self.shards.iter().enumerate() {
                let buffer_length = shard_buffer.buffered_len();
                if buffer_length > victim_length {
                    victim_length = buffer_length;
                    victim_shard_id = Some(i as u16);
                }
            }

            let Some(shard_id) = victim_shard_id else {
                break;
            };
            if victim_length == 0 {
                break;
            }

            let mut shard_buffer = std::mem::take(&mut self.shards[shard_id as usize]);
            let path = self.shard_path(shard_id);
            shard_buffer.finalize(&path)?;
            self.total_buf_bytes = self.total_buf_bytes.saturating_sub(victim_length);

            self.emit_event(SolveEvent::ShardSpill {
                worker_id: self.worker_id,
                shard_id,
                mask_index: self.current_mask,
                bytes_written: victim_length,
                timestamp: std::time::SystemTime::now(),
            });
        }
        Ok(())
    }

    pub fn current_mask(&self) -> usize {
        self.current_mask
    }

    pub fn set_current_mask(&mut self, mask_index: usize) {
        self.current_mask = mask_index;
    }

    fn emit_event(&self, event: SolveEvent) {
        let _ = self.event_tx.send(event);
    }

    pub fn emit_worker_start(&self) {
        self.emit_event(SolveEvent::WorkerStart {
            worker_id: self.worker_id,
            timestamp: std::time::SystemTime::now(),
        });
    }

    pub fn emit_worker_end(&self) {
        self.emit_event(SolveEvent::WorkerEnd {
            worker_id: self.worker_id,
            total_positions: self.local_position_count,
            timestamp: std::time::SystemTime::now(),
        });
    }

    pub fn mark_mask_complete(&mut self) -> Result<()> {
        for shard_id in 0..self.shards.len() {
            let path_temporary = self.shard_path(shard_id as u16);

            // Skip if temp file doesn't exist (empty shard - no data written)
            if !path_temporary.exists() {
                continue;
            }

            let path_final = self.tmp_dir.join(shard_id.to_string()).join(format!(
                "shard={:0width$}.mask={:04}.worker={:03}.bin",
                shard_id,
                self.current_mask(),
                self.worker_id,
                width = self.filename_width
            ));
            std::fs::rename(&path_temporary, &path_final).with_context(|| {
                format!(
                    "Failed to finalize shard {} (rename {} -> {})",
                    shard_id,
                    path_temporary.display(),
                    path_final.display()
                )
            })?;
        }

        self.emit_event(SolveEvent::MaskComplete {
            worker_id: self.worker_id,
            mask_index: self.current_mask,
            positions: self.local_position_count,
            timestamp: std::time::SystemTime::now(),
        });

        Ok(())
    }
}
