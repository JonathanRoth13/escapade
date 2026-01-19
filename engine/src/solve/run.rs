use std::{
    fs::File,
    io::BufReader,
    path::PathBuf,
    sync::{Arc, atomic::AtomicU64},
    thread,
    time::SystemTime,
};

use crate::{
    core::MachineSpecs,
    solve::{
        events::SolveEvent, logger::EventLogger, worker::worker_root, worker_context::WorkerContext,
    },
    tablebase::TablebaseIndex,
};

use crossbeam_channel::bounded;
use std::fs;

/// Fraction of system memory reserved for OS and overhead
const HEADROOM_FRAC: f64 = 0.30;

/// Estimated memory for global data structures
const GLOBALS_EST: usize = 2 * 1024 * 1024 * 1024; // 2 GiB

#[derive(Debug, serde::Deserialize)]
struct ResumeConfigEntry {
    worker_id: u16,
    resume_from_mask: usize,
    starting_positions: u64,
}

/// Canonical position counts for each depth.
/// Depth 16 is not yet computed.
const DEPTH_COUNTS: [u64; 17] = [
    1,               // Depth 0: Empty board
    1,               // Depth 1: Empty board
    12,              // Depth 2
    462,             // Depth 3
    13_013,          // Depth 4
    395_640,         // Depth 5
    8_799_099,       // Depth 6
    154_965_078,     // Depth 7
    1_869_817_599,   // Depth 8
    16_039_232_376,  // Depth 9
    89_263_657_952,  // Depth 10
    327_861_202_104, // Depth 11
    706_899_182_360, // Depth 12
    895_462_653_600, // Depth 13 (peak)
    523_611_333_864, // Depth 14
    147_948_108_768, // Depth 15
    10_029_506_543,  // Depth 16
];

#[allow(clippy::too_many_arguments)]
pub fn run(
    depth: u32,
    shard_bits: u8,
    reserve_os: f64,
    workers_opt: Option<u32>,
    tb_bytes_opt: Option<u64>,
    directory: PathBuf,
    resume_config_path: Option<PathBuf>,
    tablebase_dir: Option<PathBuf>,
) -> anyhow::Result<()> {
    if depth > 16 {
        return Err(anyhow::anyhow!("Invalid depth: {} (must be 0-16)", depth));
    }

    // Load tablebase if provided (shared across all workers)
    let (tablebase, tablebase_memory): (Option<Arc<TablebaseIndex>>, usize) = match tablebase_dir {
        Some(ref path) => {
            eprintln!("Loading tablebase from {}...", path.display());
            let tb = TablebaseIndex::load_from_dir(path)?;
            let available = tb.available_depths();
            let loaded_depths: Vec<usize> = (0..18).filter(|&i| available[i]).collect();
            let tb_mem = tb.memory_usage();
            eprintln!("Loaded tablebase depths: {:?}", loaded_depths);
            eprintln!(
                "Tablebase memory usage: {:.2} GiB",
                tb_mem as f64 / (1024.0 * 1024.0 * 1024.0)
            );
            (Some(Arc::new(tb)), tb_mem)
        }
        None => (None, 0),
    };

    let total_positions = DEPTH_COUNTS[depth as usize];

    let specs = MachineSpecs::probe();
    let available_memory = specs.available_memory(reserve_os);

    let workers: u32 = match workers_opt {
        Some(worker_count) => worker_count.max(1),
        None => {
            #[cfg(target_os = "macos")]
            {
                specs
                    .mac_perf_cores
                    .unwrap_or(specs.cpu_cores_logical as u32)
                    .saturating_sub(1)
                    .max(1)
            }
            #[cfg(not(target_os = "macos"))]
            {
                (specs.cpu_cores_logical.saturating_sub(1)).max(1) as u32
            }
        }
    };

    let is_resuming = resume_config_path.is_some();
    let resume_config: Vec<(usize, u64)> = match resume_config_path {
        Some(path) => {
            let file = File::open(&path).map_err(|e| {
                anyhow::anyhow!("Failed to open resume config {}: {}", path.display(), e)
            })?;
            let reader = BufReader::new(file);
            let entries: Vec<ResumeConfigEntry> = serde_json::from_reader(reader)
                .map_err(|e| anyhow::anyhow!("Failed to parse resume config: {}", e))?;

            let mut config = vec![(0, 0u64); workers as usize];
            for entry in entries {
                if (entry.worker_id as usize) < config.len() {
                    config[entry.worker_id as usize] =
                        (entry.resume_from_mask, entry.starting_positions);
                }
            }

            eprintln!("Resume config loaded for {} workers", workers);
            config
        }
        None => vec![(0, 0u64); workers as usize],
    };

    let initial_total: u64 = resume_config.iter().map(|(_, positions)| positions).sum();
    let global_total = Arc::new(AtomicU64::new(initial_total));

    let ram_budget_bytes: usize = match tb_bytes_opt {
        Some(override_bytes) => available_memory.min(override_bytes) as usize,
        None => available_memory as usize,
    };

    let after_headroom: usize = ((ram_budget_bytes as f64) * (1.0 - HEADROOM_FRAC)) as usize;
    let globals_and_tablebase: usize = GLOBALS_EST + tablebase_memory;
    let total_for_workers: usize = after_headroom.saturating_sub(globals_and_tablebase);
    let per_worker_budget: usize = total_for_workers / workers.max(1) as usize;

    // Use 70% of budget for buffering, leaving slack for allocator overhead
    let cap_worker_bytes: usize = ((per_worker_budget as f64) * 0.70) as usize;

    fs::create_dir_all(&directory)?;

    let (event_tx, event_rx) = bounded(10_000);
    let log_path = directory.join("events.jsonl");

    if is_resuming {
        let _ = event_tx.send(SolveEvent::RunResume {
            depth,
            workers,
            shard_bits,
            timestamp: SystemTime::now(),
        });
    } else {
        let _ = event_tx.send(SolveEvent::RunStart {
            depth,
            workers,
            shard_bits,
            timestamp: SystemTime::now(),
        });
    }

    let global_total_logger = global_total.clone();
    let logger_handle = thread::spawn(move || {
        let logger = EventLogger::new(
            event_rx,
            log_path,
            global_total_logger,
            total_positions,
            std::time::Duration::from_secs(1),
        )
        .expect("Failed to create event logger");
        logger.run().expect("Logger thread failed");
    });

    let mut handles = Vec::with_capacity(workers as usize);

    for worker_id in 0..workers {
        let global_total_clone = global_total.clone();
        let temporary_directory = directory.clone();
        let event_tx_clone = event_tx.clone();
        let tablebase_clone = tablebase.clone();
        let (starting_mask, starting_positions) = resume_config[worker_id as usize];
        let handle = thread::spawn(move || -> anyhow::Result<()> {
            let mut context = WorkerContext::new(
                shard_bits,
                cap_worker_bytes,
                temporary_directory,
                worker_id as u16,
                starting_mask,
                starting_positions,
                global_total_clone,
                event_tx_clone,
            );

            worker_root(
                &mut context,
                workers,
                depth as usize,
                tablebase_clone.as_deref(),
            )
        });

        handles.push(handle);
    }

    let mut worker_errors = Vec::new();
    for (i, handle) in handles.into_iter().enumerate() {
        match handle.join() {
            Ok(Ok(())) => {}
            Ok(Err(e)) => {
                worker_errors.push(format!("Worker {}: {}", i, e));
            }
            Err(_) => {
                eprintln!(
                    "FATAL: Worker {} panicked - aborting without saving state",
                    i
                );
                std::process::abort();
            }
        }
    }

    if !worker_errors.is_empty() {
        return Err(anyhow::anyhow!(
            "Worker failures:\n  {}",
            worker_errors.join("\n  ")
        ));
    }

    let final_total = global_total.load(std::sync::atomic::Ordering::Relaxed);
    let _ = event_tx.send(SolveEvent::RunEnd {
        total_positions: final_total,
        timestamp: SystemTime::now(),
    });

    drop(event_tx);
    let _ = logger_handle.join();

    Ok(())
}
