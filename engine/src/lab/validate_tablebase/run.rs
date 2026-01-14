use super::worker::validate_positions;
use crate::common::{MachineSpecs, format_ply_hex};
use crate::tablebase::TablebaseIndex;
use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

pub fn run(
    layer: usize,
    tablebase_dir: PathBuf,
    samples: usize,
    workers_opt: Option<u32>,
    seed: Option<u64>,
) -> Result<()> {
    println!("Validating tablebase for layer {}", layer);
    println!("Samples: {}", samples);

    // Determine number of workers
    let num_workers = match workers_opt {
        Some(w) => w,
        None => {
            let specs = MachineSpecs::probe();
            #[cfg(target_os = "macos")]
            let count = specs
                .mac_perf_cores
                .unwrap_or(specs.cpu_cores_physical as u32);
            #[cfg(not(target_os = "macos"))]
            let count = specs.cpu_cores_logical as u32;
            count
        }
    };
    println!("Workers: {}", num_workers);

    let rng_seed = seed.unwrap_or(0x5EED_C0DE_CAFE_BABE);
    println!("Seed: 0x{:016X}\n", rng_seed);

    // Load tablebase from directory
    println!("Loading tablebase from {}...", tablebase_dir.display());
    let tablebase = TablebaseIndex::load_from_dir(&tablebase_dir)?;

    // Get available layers
    let available_layers = tablebase.available_layers();

    // Show what layers were loaded
    let loaded_layers: Vec<usize> = (0..18).filter(|&i| available_layers[i]).collect();
    println!("Loaded layers: {:?}", loaded_layers);

    // Check if the target layer is available
    if layer > 16 || !available_layers[layer] {
        anyhow::bail!("Tablebase for layer {} not found in directory", layer);
    }

    println!("Tablebase loaded successfully\n");

    let tablebase = Arc::new(tablebase);

    // Spawn workers
    let mut handles = Vec::new();

    for worker_id in 0..num_workers {
        let tablebase_ref = Arc::clone(&tablebase);

        let handle = thread::spawn(move || {
            validate_positions(
                worker_id,
                num_workers,
                layer,
                samples,
                rng_seed,
                &tablebase_ref,
            )
        });

        handles.push(handle);
    }

    // Collect results
    let mut all_errors = Vec::new();
    let mut total_samples = 0usize;
    let mut total_original_nanos = 0u64;
    let mut total_tablebase_nanos = 0u64;

    for (i, handle) in handles.into_iter().enumerate() {
        let (errors, count, orig_nanos, tb_nanos) = match handle.join() {
            Ok(result) => result,
            Err(_) => {
                eprintln!(
                    "FATAL: Worker {} panicked - aborting without saving state",
                    i
                );
                std::process::abort();
            }
        };
        all_errors.extend(errors);
        total_samples += count;
        total_original_nanos += orig_nanos;
        total_tablebase_nanos += tb_nanos;
    }

    // Write all errors to file for analysis
    if !all_errors.is_empty() {
        let error_file = PathBuf::from(format!("validation_errors_layer{}.csv", layer));
        let file = File::create(&error_file)?;
        let mut writer = BufWriter::new(file);

        // Write header
        writeln!(writer, "ply_hex,eval_outcome,tb_outcome")?;

        // Write all errors
        for error in &all_errors {
            writeln!(
                writer,
                "{},{},{}",
                format_ply_hex(&error.ply),
                error.expected,
                error.actual
            )?;
        }

        writer.flush()?;
        println!(
            "Wrote {} errors to: {}\n",
            all_errors.len(),
            error_file.display()
        );
    }

    // Report results
    println!("═══════════════════════════════════════════════════════════");
    println!("VALIDATION RESULTS");
    println!("═══════════════════════════════════════════════════════════");
    println!("Total samples validated: {}", total_samples);
    println!("Errors found: {}", all_errors.len());

    if !all_errors.is_empty() {
        println!("\n⚠️  VALIDATION FAILURES:");
        for (i, error) in all_errors.iter().take(10).enumerate() {
            println!(
                "  [{}] {} - Evaluation: {}, Tablebase: {}",
                i + 1,
                format_ply_hex(&error.ply),
                error.expected,
                error.actual
            );
        }
        if all_errors.len() > 10 {
            println!("  ... and {} more errors", all_errors.len() - 10);
        }

        // Analyze error patterns
        analyze_errors(&all_errors);
    } else {
        println!("✓ All validations passed!");
    }

    // Performance comparison
    println!("\n═══════════════════════════════════════════════════════════");
    println!("PERFORMANCE COMPARISON");
    println!("═══════════════════════════════════════════════════════════");

    let avg_original_ns = total_original_nanos / total_samples as u64;
    let avg_tablebase_ns = total_tablebase_nanos / total_samples as u64;

    println!("Original evaluation:");
    println!("  Average: {} ns", avg_original_ns);
    println!(
        "  Total:   {:.3} ms",
        total_original_nanos as f64 / 1_000_000.0
    );

    println!("\nTablebase lookup:");
    println!("  Average: {} ns", avg_tablebase_ns);
    println!(
        "  Total:   {:.3} ms",
        total_tablebase_nanos as f64 / 1_000_000.0
    );

    if avg_tablebase_ns < avg_original_ns {
        let speedup = avg_original_ns as f64 / avg_tablebase_ns as f64;
        println!("\n✓ Tablebase is {:.2}x faster", speedup);
    } else {
        let slowdown = avg_tablebase_ns as f64 / avg_original_ns as f64;
        println!("\n⚠️  Tablebase is {:.2}x slower", slowdown);
    }

    if !all_errors.is_empty() {
        anyhow::bail!(
            "Validation failed: {} errors found out of {} samples",
            all_errors.len(),
            total_samples
        );
    }

    Ok(())
}

fn analyze_errors(errors: &[super::worker::ValidationError]) {
    let total = errors.len();

    // Categorize errors
    let mut same_winner_diff_depth = 0;
    let mut different_winner = 0;
    let mut eval_draw_tb_win = 0;
    let mut eval_win_tb_draw = 0;
    let mut eval_odd_tb_even = 0;
    let mut eval_even_tb_odd = 0;

    // Track outcome pairs
    let mut outcome_pairs: HashMap<(u8, u8), usize> = HashMap::new();

    for error in errors {
        let eval = error.expected;
        let tb = error.actual;

        *outcome_pairs.entry((eval, tb)).or_insert(0) += 1;

        let eval_parity = eval % 2;
        let tb_parity = tb % 2;

        if eval == 15 || tb == 15 {
            // Draw involved
            if eval == 15 {
                eval_draw_tb_win += 1;
            } else {
                eval_win_tb_draw += 1;
            }
        } else if eval_parity != tb_parity {
            // Different winner
            different_winner += 1;
            if eval_parity == 1 {
                eval_odd_tb_even += 1;
            } else {
                eval_even_tb_odd += 1;
            }
        } else {
            // Same winner, different depth
            same_winner_diff_depth += 1;
        }
    }

    // Print analysis
    println!("\n═══════════════════════════════════════════════════════════");
    println!("ERROR PATTERN ANALYSIS");
    println!("═══════════════════════════════════════════════════════════");

    println!(
        "\n1. Same winner, different depth: {} ({:.1}%)",
        same_winner_diff_depth,
        100.0 * same_winner_diff_depth as f64 / total as f64
    );
    println!("   (Both agree on winner, but disagree on when game ends)");

    println!(
        "\n2. Different winner: {} ({:.1}%)",
        different_winner,
        100.0 * different_winner as f64 / total as f64
    );
    println!(
        "   - Eval says White wins (odd), TB says Black wins (even): {}",
        eval_odd_tb_even
    );
    println!(
        "   - Eval says Black wins (even), TB says White wins (odd): {}",
        eval_even_tb_odd
    );

    println!(
        "\n3. Draw vs Win discrepancies: {} ({:.1}%)",
        eval_draw_tb_win + eval_win_tb_draw,
        100.0 * (eval_draw_tb_win + eval_win_tb_draw) as f64 / total as f64
    );
    println!(
        "   - Eval says DRAW (15), TB says WIN: {}",
        eval_draw_tb_win
    );
    println!(
        "   - Eval says WIN, TB says DRAW (15): {}",
        eval_win_tb_draw
    );

    // Show most common outcome pairs
    let mut pairs_vec: Vec<_> = outcome_pairs.iter().collect();
    pairs_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("\n═══════════════════════════════════════════════════════════");
    println!("TOP 10 OUTCOME PAIRS");
    println!("═══════════════════════════════════════════════════════════");
    println!(
        "{:<20} {:<20} {:<10} {:<10}",
        "Eval Outcome", "TB Outcome", "Count", "% of errors"
    );
    println!("───────────────────────────────────────────────────────────");

    for ((eval, tb), count) in pairs_vec.iter().take(10) {
        let eval_str = outcome_description(*eval);
        let tb_str = outcome_description(*tb);
        let pct = 100.0 * **count as f64 / total as f64;
        println!(
            "{:<20} {:<20} {:<10} {:<10.1}%",
            eval_str, tb_str, **count, pct
        );
    }

    // Check outcome frequency
    println!("\n═══════════════════════════════════════════════════════════");
    println!("OUTCOME FREQUENCY ANALYSIS");
    println!("═══════════════════════════════════════════════════════════");

    let mut eval_counts: HashMap<u8, usize> = HashMap::new();
    let mut tb_counts: HashMap<u8, usize> = HashMap::new();

    for error in errors {
        *eval_counts.entry(error.expected).or_insert(0) += 1;
        *tb_counts.entry(error.actual).or_insert(0) += 1;
    }

    println!("Outcomes seen in EVAL: {:?}", {
        let mut keys: Vec<_> = eval_counts.keys().copied().collect();
        keys.sort();
        keys
    });

    println!("Outcomes seen in TB:   {:?}", {
        let mut keys: Vec<_> = tb_counts.keys().copied().collect();
        keys.sort();
        keys
    });

    // Find outcomes that appear in one but not the other
    let eval_only: Vec<u8> = eval_counts
        .keys()
        .filter(|k| !tb_counts.contains_key(k))
        .copied()
        .collect();

    let tb_only: Vec<u8> = tb_counts
        .keys()
        .filter(|k| !eval_counts.contains_key(k))
        .copied()
        .collect();

    if !eval_only.is_empty() {
        println!("\nOutcomes ONLY in eval: {:?}", eval_only);
    }

    if !tb_only.is_empty() {
        println!("Outcomes ONLY in TB: {:?}", tb_only);
    }
}

fn outcome_description(outcome: u8) -> String {
    if outcome == 15 {
        "DRAW (15)".to_string()
    } else if outcome == 255 {
        "MISSING (255)".to_string()
    } else {
        let winner = if outcome % 2 == 1 { "W" } else { "B" };
        let depth = 17 - outcome;
        format!("{} wins @ {} ({})", winner, depth, outcome)
    }
}
