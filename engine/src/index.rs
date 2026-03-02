use crate::core::{MachineSpecs, hash_bucket_bytes, hash_phi_bytes};
use anyhow::Result;
use bitvec::prelude::*;
use memmap2::MmapOptions;
use num_format::{Locale, ToFormattedString};
use rayon::prelude::*;
use std::fs::{self, File};
use std::io::{BufReader, Read, Seek, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

/// Extract the key portion from an 11-byte record by masking out the outcome bits
#[inline]
fn record_to_key(record: &[u8; 11]) -> [u8; 11] {
    let mut key = *record;
    // Byte 10: upper 4 bits = piece_to_place (keep), lower 4 bits = outcome (mask out)
    key[10] &= 0xF0; // Keep only upper 4 bits
    key
}

#[allow(clippy::too_many_arguments)]
pub fn run(
    depth: u32,
    shard_path: PathBuf,
    index_path: PathBuf,
    shard_id: u8,
    shard_bits: u8,
    target_bucket_size: u64,
    reserve_os: f64,
    workers_opt: Option<u32>,
) -> Result<()> {
    // Probe machine specs
    let specs = MachineSpecs::probe();
    let available_memory = specs.available_memory(reserve_os);

    // Verify shard file exists
    if !shard_path.exists() {
        anyhow::bail!("Shard file not found: {}", shard_path.display());
    }

    // Get shard size
    let metadata = fs::metadata(&shard_path)?;
    let shard_size = metadata.len();
    let num_keys = shard_size / 11;

    if shard_size % 11 != 0 {
        anyhow::bail!("Shard size {} is not divisible by 11", shard_size);
    }

    // Calculate r = 2^⌈log2(n/t)⌉
    let ratio = num_keys as f64 / target_bucket_size as f64;
    let log2_ratio = ratio.log2();
    let bucket_bits = log2_ratio.ceil() as u32;
    let num_buckets: u64 = 1u64 << bucket_bits;

    // Calculate m = number of slots
    // we multiply by 2 to give us at most a load factor of .5
    // this is a deviation from the CHD algorithm
    let slots_needed = (num_keys as f64 * 2.0).ceil() as u64;
    let log2_slots = (slots_needed as f64).log2();
    let slot_bits = log2_slots.ceil() as u32;
    let num_slots: u64 = 1u64 << slot_bits;

    // Calculate max workers based on Pass 2 memory (most restrictive)
    // Pass 2 shared memory: keys + bucket_histogram + bucket_offsets
    // Per-worker memory: worker_offsets (reuses local_histograms memory from Pass 1)
    let shared_memory = (num_keys * 11) +       // keys array
        (num_buckets * 8) +                     // bucket_histogram
        (num_buckets * 8);                      // bucket_offsets
    let per_worker_memory = num_buckets * 8;    // worker_offsets / local_histogram
    let memory_for_workers = available_memory.saturating_sub(shared_memory);
    let max_workers = if per_worker_memory > 0 {
        (memory_for_workers / per_worker_memory).max(1) as u32
    } else {
        u32::MAX
    };

    // Get CPU-based worker count
    let cpu_workers = {
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
    };

    let workers: u32 = match workers_opt {
        Some(w) => w.min(max_workers),
        None => cpu_workers.min(max_workers),
    };

    rayon::ThreadPoolBuilder::new()
        .num_threads(workers as usize)
        .build_global()
        .ok(); // Ignore error if already initialized

    println!("═══════════════════════════════════════════════════════════");
    println!("              INDEX GENERATION STARTING");
    println!("═══════════════════════════════════════════════════════════");
    println!(
        "Shard size          : {} bytes ({} records)",
        shard_size, num_keys
    );
    println!("Target bucket size  : {}", target_bucket_size);
    println!("Number of buckets   : 2^{} = {}", bucket_bits, num_buckets);
    println!(
        "Number of slots     : 2^{} = {} (load factor: {:.2}%)",
        slot_bits,
        num_slots,
        100.0 * num_keys as f64 / num_slots as f64
    );
    println!("Workers             : {}", workers);
    println!("═══════════════════════════════════════════════════════════\n");

    let mut bucket_histogram: Vec<usize> = vec![0; num_buckets as usize];
    let mut bucket_offsets: Vec<usize> = vec![0; num_buckets as usize];

    let bucket_mask: u64 = (1u64 << bucket_bits) - 1;
    let slot_mask: u64 = (1u64 << slot_bits) - 1;

    let shard_start = Instant::now();

    // ===== STEP 1: Build Histogram =====
    println!("(1/6) Building histogram...");
    let step1_start = Instant::now();

    let file = File::open(&shard_path)?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    #[cfg(target_os = "macos")]
    unsafe {
        libc::madvise(mmap.as_ptr() as *mut _, mmap.len(), libc::MADV_SEQUENTIAL);
    }

    let records_per_worker = (num_keys as usize).div_ceil(workers as usize);
    let global_histogram = Mutex::new(&mut bucket_histogram);

    // Store local histograms for Step 2
    let local_histograms: Vec<Vec<usize>> = (0..workers)
        .into_par_iter()
        .map(|worker_id| {
            let start_record = (worker_id as usize) * records_per_worker;
            let end_record = ((worker_id as usize + 1) * records_per_worker).min(num_keys as usize);

            if start_record >= num_keys as usize {
                return vec![0usize; num_buckets as usize];
            }

            let mut local_histogram = vec![0usize; num_buckets as usize];

            for record_idx in start_record..end_record {
                let offset = record_idx * 11;
                let mut record = [0u8; 11];
                record.copy_from_slice(&mmap[offset..offset + 11]);

                let key = record_to_key(&record);
                let hash = hash_bucket_bytes(&key);
                let bucket_id = (hash & bucket_mask) as usize;
                local_histogram[bucket_id] += 1;
            }

            let mut global = global_histogram.lock().unwrap();
            for (bucket_id, &count) in local_histogram.iter().enumerate() {
                global[bucket_id] += count;
            }
            drop(global);

            local_histogram
        })
        .collect();

    drop(mmap);
    drop(file);

    let step1_ms = step1_start.elapsed().as_millis() as u64;
    println!("      Complete ({:.2} sec)\n", step1_ms as f64 / 1000.0);

    // Compute bucket offsets
    let mut cumulative = 0usize;
    for bucket_id in 0..num_buckets as usize {
        bucket_offsets[bucket_id] = cumulative;
        cumulative += bucket_histogram[bucket_id];
    }

    let non_empty_buckets = bucket_histogram.iter().filter(|&&c| c > 0).count();

    // Compute per-worker offsets for each bucket
    let mut worker_offsets: Vec<Vec<usize>> = vec![vec![0; num_buckets as usize]; workers as usize];
    for (bucket_id, &bucket_offset) in bucket_offsets.iter().enumerate() {
        let mut cumulative = bucket_offset;
        for worker_id in 0..workers as usize {
            worker_offsets[worker_id][bucket_id] = cumulative;
            cumulative += local_histograms[worker_id][bucket_id];
        }
    }
    drop(local_histograms);

    let mut keys: Vec<[u8; 11]> = Vec::with_capacity(num_keys as usize);

    // ===== STEP 2: Scatter Keys =====
    println!("(2/6) Scattering keys...");
    let step2_start = Instant::now();

    // Progress tracking
    let progress_counter = Arc::new(AtomicU64::new(0));
    let progress_done = Arc::new(AtomicBool::new(false));
    let progress_counter_clone = progress_counter.clone();
    let progress_done_clone = progress_done.clone();

    // Spawn progress reporter thread
    let reporter_handle = thread::spawn(move || {
        let start = Instant::now();

        loop {
            thread::sleep(std::time::Duration::from_secs(2));

            if progress_done_clone.load(Ordering::Relaxed) {
                break;
            }

            let current = progress_counter_clone.load(Ordering::Relaxed);
            let percent = (current as f64 / num_keys as f64) * 100.0;
            let elapsed = start.elapsed();
            let hh = elapsed.as_secs() / 3600;
            let mm = (elapsed.as_secs() / 60) % 60;
            let ss = elapsed.as_secs() % 60;

            let current_str = current.to_formatted_string(&Locale::en);

            eprintln!(
                "      [Scattering Keys] t={hh:02}:{mm:02}:{ss:02} total={current_str:>16} ({percent:>6.2}%)",
            );
        }
    });

    let keys_ptr_addr = keys.as_mut_ptr() as usize;
    let shard_path_clone = shard_path.clone();
    let progress_counter_worker = progress_counter.clone();

    // Consume worker_offsets so each worker takes ownership without cloning
    worker_offsets.into_par_iter().enumerate().for_each(
        |(worker_id, mut local_write_positions)| {
            let start_record = worker_id * records_per_worker;
            let end_record = ((worker_id + 1) * records_per_worker).min(num_keys as usize);

            if start_record >= num_keys as usize {
                return;
            }

            let records_this_worker = end_record - start_record;

            // Each worker opens its own file handle with buffered reads
            let file = File::open(&shard_path_clone).expect("Failed to open shard file");
            let mut reader = BufReader::with_capacity(8 * 1024 * 1024, file); // 8 MB buffer
            let start_offset = (start_record * 11) as u64;
            reader
                .seek(std::io::SeekFrom::Start(start_offset))
                .expect("Failed to seek");

            let ptr = keys_ptr_addr as *mut [u8; 11];

            // Update progress every 10 million records
            let report_interval = 10_000_000;
            let mut local_count = 0usize;

            for _ in 0..records_this_worker {
                let mut record = [0u8; 11];
                reader
                    .read_exact(&mut record)
                    .expect("Failed to read record");

                let key = record_to_key(&record);
                let hash = hash_bucket_bytes(&key);
                let bucket_id = (hash & bucket_mask) as usize;

                let pos = local_write_positions[bucket_id];
                unsafe {
                    ptr.add(pos).write(key);
                }
                local_write_positions[bucket_id] += 1;

                local_count += 1;
                if local_count >= report_interval {
                    progress_counter_worker.fetch_add(local_count as u64, Ordering::Relaxed);
                    local_count = 0;
                }
            }

            // Flush remaining count
            if local_count > 0 {
                progress_counter_worker.fetch_add(local_count as u64, Ordering::Relaxed);
            }
        },
    );

    // Set the length of keys vector now that all writes are complete
    unsafe {
        keys.set_len(num_keys as usize);
    }

    // Signal reporter thread to stop
    progress_done.store(true, Ordering::Relaxed);
    if reporter_handle.join().is_err() {
        eprintln!("FATAL: Reporter thread panicked - aborting without saving state");
        std::process::abort();
    }

    let step2_ms = step2_start.elapsed().as_millis() as u64;
    println!("      Complete ({:.2} sec)\n", step2_ms as f64 / 1000.0);

    // ===== STEP 3: Sort Buckets =====
    println!("(3/6) Sorting buckets...");
    let step3_start = Instant::now();

    let mut bucket_indices: Vec<usize> = (0..num_buckets as usize).collect();
    bucket_indices.par_sort_by_key(|&i| (std::cmp::Reverse(bucket_histogram[i]), i));

    let step3_ms = step3_start.elapsed().as_millis() as u64;
    println!("      Complete ({:.2} sec)\n", step3_ms as f64 / 1000.0);

    // Initialize data structures for Step 4 (derive sigma)
    let mut sigma: Vec<u16> = vec![0u16; num_buckets as usize];
    let mut occupied = bitvec![0; num_slots as usize];

    // ===== STEP 4: Derive Sigma =====
    println!("(4/6) Deriving sigma...");

    let step4_start = Instant::now();
    let report_every_buckets = (non_empty_buckets / 200).max(1);
    let mut next_mark_buckets = report_every_buckets;
    let mut processed_buckets = 0;

    let mut candidate_occupied = bitvec![0; num_slots as usize];
    let mut candidate_list = Vec::new();

    for &bucket_id in bucket_indices.iter() {
        let bucket_size = bucket_histogram[bucket_id];

        if bucket_size == 0 {
            break;
        }

        let start = bucket_offsets[bucket_id];
        let end = start + bucket_size;
        let bucket_keys = &keys[start..end];

        let mut l: u16 = 0;
        const MAX_DISPLACEMENT: u16 = (1 << 14) - 1; // 16383 (14 bits)

        'search: loop {
            if l > MAX_DISPLACEMENT {
                eprintln!(
                    "ERROR: Cannot find displacement for bucket {} (size: {}). Tried {} displacements.",
                    bucket_id, bucket_size, l
                );
                eprintln!("Occupied slots: {} / {}", occupied.count_ones(), num_slots);
                anyhow::bail!("Failed to find displacement value for bucket {}", bucket_id);
            }

            // Clear only the slots we touched in previous attempt
            for &slot_index in &candidate_list {
                candidate_occupied.set(slot_index, false);
            }
            candidate_list.clear();

            for key in bucket_keys {
                let hash = hash_phi_bytes(key, l as u64);
                let slot_index = (hash & slot_mask) as usize;

                if slot_index >= occupied.len() {
                    eprintln!(
                        "ERROR: slot_index {} >= occupied.len() {}",
                        slot_index,
                        occupied.len()
                    );
                    anyhow::bail!("Slot index out of bounds");
                }

                // Check both previous buckets AND intra-bucket collisions
                if occupied[slot_index] || candidate_occupied[slot_index] {
                    l += 1;
                    continue 'search;
                }

                candidate_occupied.set(slot_index, true);
                candidate_list.push(slot_index);
            }

            // Success - mark all candidate slots as permanently occupied
            for &slot_index in &candidate_list {
                occupied.set(slot_index, true);
            }

            sigma[bucket_id] = l;
            break;
        }

        processed_buckets += 1;

        if processed_buckets >= next_mark_buckets {
            let elapsed = step4_start.elapsed();
            let hh = elapsed.as_secs() / 3600;
            let mm = (elapsed.as_secs() / 60) % 60;
            let ss = elapsed.as_secs() % 60;
            let pct = 100.0 * processed_buckets as f64 / non_empty_buckets as f64;
            let count_str = processed_buckets.to_formatted_string(&Locale::en);
            eprintln!(
                "      [Derive Sigma] t={hh:02}:{mm:02}:{ss:02} total={count_str:>16} ({pct:>6.2}%)",
            );
            next_mark_buckets =
                ((processed_buckets / report_every_buckets) + 1) * report_every_buckets;
        }
    }

    let step4_ms = step4_start.elapsed().as_millis() as u64;
    println!("      Complete ({:.2} sec)\n", step4_ms as f64 / 1000.0);

    // ===== STEP 5: Populate Outcomes =====
    println!("(5/6) Populating outcomes...");

    let step5_start = Instant::now();
    let outcome_bytes = num_slots.div_ceil(2);
    let mut outcomes: Vec<u8> = vec![0u8; outcome_bytes as usize];

    let file = File::open(&shard_path)?;
    let mut reader = BufReader::with_capacity(256 * 1024, file);
    let mut buffer = [0u8; 11];
    let mut record_count = 0u64;
    let report_every_records = (num_keys / 200).max(1);
    let mut next_mark_records = report_every_records;

    loop {
        match reader.read_exact(&mut buffer) {
            Ok(()) => {
                let key = record_to_key(&buffer);
                let outcome = buffer[10] & 0x0F;

                let hash_bucket = hash_bucket_bytes(&key);
                let bucket_idx = (hash_bucket & bucket_mask) as usize;

                let l = sigma[bucket_idx];

                let hash_phi = hash_phi_bytes(&key, l as u64);
                let slot_idx = (hash_phi & slot_mask) as usize;

                let byte_idx = slot_idx / 2;
                if slot_idx.is_multiple_of(2) {
                    outcomes[byte_idx] = (outcomes[byte_idx] & 0x0F) | (outcome << 4);
                } else {
                    outcomes[byte_idx] = (outcomes[byte_idx] & 0xF0) | outcome;
                }

                record_count += 1;

                if record_count >= next_mark_records {
                    let elapsed = step5_start.elapsed();
                    let hh = elapsed.as_secs() / 3600;
                    let mm = (elapsed.as_secs() / 60) % 60;
                    let ss = elapsed.as_secs() % 60;
                    let pct = 100.0 * record_count as f64 / num_keys as f64;
                    let count_str = record_count.to_formatted_string(&Locale::en);
                    eprintln!(
                        "      [Populate Outcomes] t={hh:02}:{mm:02}:{ss:02} total={count_str:>16} ({pct:>6.2}%)",
                    );
                    next_mark_records =
                        ((record_count / report_every_records) + 1) * report_every_records;
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e.into()),
        }
    }

    let step5_ms = step5_start.elapsed().as_millis() as u64;
    println!("      Complete ({:.2} sec)\n", step5_ms as f64 / 1000.0);

    // ===== STEP 6: Write Index File =====
    println!("(6/6) Writing index file...");

    let step6_start = Instant::now();

    // Create output directory if it doesn't exist
    if let Some(parent) = index_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(&index_path)?;
    let mut writer = std::io::BufWriter::with_capacity(4 * 1024 * 1024, file);

    // Write 18-byte header
    const HEADER_SIZE: usize = 18;
    let mut header = [0u8; HEADER_SIZE];
    let mut offset = 0;

    // Magic (4 bytes)
    header[offset..offset + 4].copy_from_slice(b"ESCQ");
    offset += 4;

    // Version (1 byte)
    header[offset] = 1;
    offset += 1;

    // Depth (1 byte)
    header[offset] = depth as u8;
    offset += 1;

    // Shard ID (1 byte)
    header[offset] = shard_id;
    offset += 1;

    // Shard bits (1 byte)
    header[offset] = shard_bits;
    offset += 1;

    // Bucket bits (1 byte)
    header[offset] = bucket_bits as u8;
    offset += 1;

    // Slot bits (1 byte)
    header[offset] = slot_bits as u8;
    offset += 1;

    // Num keys (8 bytes, little-endian)
    header[offset..offset + 8].copy_from_slice(&num_keys.to_le_bytes());
    offset += 8;

    assert_eq!(offset, HEADER_SIZE);
    writer.write_all(&header)?;

    // Write sigma array (u16 little-endian)
    let sigma_bytes = num_buckets * 2;
    for &displacement in &sigma {
        writer.write_all(&displacement.to_le_bytes())?;
    }

    // Write outcomes array (4-bit packed)
    writer.write_all(&outcomes)?;

    writer.flush()?;

    let step6_ms = step6_start.elapsed().as_millis() as u64;
    let total_ms = shard_start.elapsed().as_millis() as u64;

    let total_file_size = HEADER_SIZE as u64 + sigma_bytes + outcome_bytes;

    println!("      Complete ({:.2} sec)\n", step6_ms as f64 / 1000.0);
    println!("═══════════════════════════════════════════════════════════");
    println!("              INDEX GENERATION COMPLETE");
    println!("═══════════════════════════════════════════════════════════");
    println!("Output file : {}", index_path.display());
    println!(
        "File size   : {} bytes ({:.2} GiB)",
        total_file_size,
        total_file_size as f64 / (1024.0 * 1024.0 * 1024.0)
    );
    println!();
    println!("Timing Summary:");
    println!(
        "  (1/6) Build histogram   : {:>8.2} sec ({:>5.1}%)",
        step1_ms as f64 / 1000.0,
        100.0 * step1_ms as f64 / total_ms as f64
    );
    println!(
        "  (2/6) Scatter keys      : {:>8.2} sec ({:>5.1}%)",
        step2_ms as f64 / 1000.0,
        100.0 * step2_ms as f64 / total_ms as f64
    );
    println!(
        "  (3/6) Sort buckets      : {:>8.2} sec ({:>5.1}%)",
        step3_ms as f64 / 1000.0,
        100.0 * step3_ms as f64 / total_ms as f64
    );
    println!(
        "  (4/6) Derive sigma      : {:>8.2} sec ({:>5.1}%)",
        step4_ms as f64 / 1000.0,
        100.0 * step4_ms as f64 / total_ms as f64
    );
    println!(
        "  (5/6) Populate outcomes : {:>8.2} sec ({:>5.1}%)",
        step5_ms as f64 / 1000.0,
        100.0 * step5_ms as f64 / total_ms as f64
    );
    println!(
        "  (6/6) Write index file  : {:>8.2} sec ({:>5.1}%)",
        step6_ms as f64 / 1000.0,
        100.0 * step6_ms as f64 / total_ms as f64
    );
    println!("  ─────────────────────────────────────────────────");
    println!(
        "  Total                   : {:>8.2} sec",
        total_ms as f64 / 1000.0
    );
    println!("═══════════════════════════════════════════════════════════\n");

    Ok(())
}
