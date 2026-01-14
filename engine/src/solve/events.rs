use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SolveEvent {
    RunStart {
        layer: u32,
        workers: u32,
        shard_bits: u8,
        #[serde(with = "system_time_serde")]
        timestamp: SystemTime,
    },

    RunResume {
        layer: u32,
        workers: u32,
        shard_bits: u8,
        #[serde(with = "system_time_serde")]
        timestamp: SystemTime,
    },

    WorkerStart {
        worker_id: u16,
        #[serde(with = "system_time_serde")]
        timestamp: SystemTime,
    },

    MaskComplete {
        worker_id: u16,
        mask_index: usize,
        positions: u64,
        #[serde(with = "system_time_serde")]
        timestamp: SystemTime,
    },

    ShardSpill {
        worker_id: u16,
        shard_id: u16,
        mask_index: usize,
        bytes_written: usize,
        #[serde(with = "system_time_serde")]
        timestamp: SystemTime,
    },

    WorkerEnd {
        worker_id: u16,
        total_positions: u64,
        #[serde(with = "system_time_serde")]
        timestamp: SystemTime,
    },

    RunEnd {
        total_positions: u64,
        #[serde(with = "system_time_serde")]
        timestamp: SystemTime,
    },
}

/// Custom serde for SystemTime (serialize as Unix timestamp)
mod system_time_serde {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = time
            .duration_since(UNIX_EPOCH)
            .map_err(serde::ser::Error::custom)?;
        serializer.serialize_u64(duration.as_secs())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(UNIX_EPOCH + std::time::Duration::from_secs(secs))
    }
}
