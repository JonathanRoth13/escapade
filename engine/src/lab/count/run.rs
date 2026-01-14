use std::thread;

use crate::{common::MachineSpecs, lab::count::worker::worker_root};

use anyhow::Result;

pub fn handle(layer: usize, workers_opt: Option<u32>) -> anyhow::Result<()> {
    // Probe machine specs
    let specs = MachineSpecs::probe();

    // Choose worker count.
    let workers: u32 = match workers_opt {
        Some(w) => w.max(1),
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

    let mut handles = Vec::with_capacity(workers as usize);
    for worker_id in 0..workers {
        let h = thread::spawn(move || worker_root(worker_id, workers, layer));
        handles.push(h);
    }

    let mut total_positions = 0usize;
    let mut canonical_positions = 0usize;
    let mut canonical_non_terminal = 0usize;

    for (i, h) in handles.into_iter().enumerate() {
        match h.join() {
            Ok(stats) => {
                total_positions += stats.total_positions;
                canonical_positions += stats.canonical_positions;
                canonical_non_terminal += stats.canonical_non_terminal;
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

    println!("========================================");
    println!("Layer {} position counts:", layer);
    println!("========================================");
    println!("Total positions:           {}", total_positions);
    println!("Canonical positions:       {}", canonical_positions);
    println!("Canonical non-terminal:    {}", canonical_non_terminal);
    println!("========================================");

    Result::Ok(())
}
