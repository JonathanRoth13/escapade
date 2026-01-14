use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

/// Size of each record in bytes
const RECORD_SIZE_BYTES: usize = 11;

/// Merges all shard files from input directory into a single output file
pub fn run(input_dir: PathBuf, output: PathBuf) -> anyhow::Result<()> {
    let merge_start = Instant::now();

    // Find files
    let mut files: Vec<PathBuf> = fs::read_dir(&input_dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect();

    if files.is_empty() {
        anyhow::bail!("No files found in directory: {}", input_dir.display());
    }

    // Sort lexicographically by filename
    files.sort();

    // Create output directory if it doesn't exist
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }

    // Merge files
    let mut output_file = fs::File::create(&output)?;
    let mut total_bytes: u64 = 0;
    let mut total_records: u64 = 0;

    for path in &files {
        let data = fs::read(path)?;
        let file_bytes = data.len() as u64;

        // Verify file size is divisible by record size
        if !file_bytes.is_multiple_of(RECORD_SIZE_BYTES as u64) {
            eprintln!(
                "WARNING: File {} has size {} (not divisible by {})",
                path.display(),
                file_bytes,
                RECORD_SIZE_BYTES
            );
        }

        output_file.write_all(&data)?;
        total_bytes += file_bytes;
        total_records += file_bytes / RECORD_SIZE_BYTES as u64;
    }

    let total_seconds = merge_start.elapsed().as_secs_f64();

    // Output summary
    println!("Files merged  : {}", files.len());
    println!(
        "File size     : {} bytes ({:.2} GiB)",
        total_bytes,
        total_bytes as f64 / (1024.0 * 1024.0 * 1024.0)
    );
    println!("Total records : {}", total_records);
    println!("Time          : {:.2} sec", total_seconds);

    Ok(())
}
