use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

#[derive(Default, Debug)]
pub struct ShardBuffer {
    buffer: Vec<u8>, // multiples of 11 bytes
}

/// Maximum retry attempts for transient I/O errors
const MAX_RETRIES: usize = 10;

/// Initial backoff delay before first retry
const INITIAL_BACKOFF_MS: u64 = 100;

/// Maximum backoff delay between retries
const MAX_BACKOFF_MS: u64 = 5_000;

/// Chunk size for writing buffered data to disk
const WRITE_CHUNK_MAX: usize = 1024 * 1024; // 1 MiB

#[inline]
fn is_retryable_io(error: &io::Error) -> bool {
    // Common transient I/O conditions on macOS/posix:
    // EIO=5, EINTR=4, EAGAIN=35, ETIMEDOUT=60, ENXIO=6
    matches!(
        error.raw_os_error(),
        Some(5) | Some(4) | Some(35) | Some(60) | Some(6)
    )
}

/// Append data to file with retry on transient I/O failures
fn append_with_retry(path: &Path, data: &[u8]) -> io::Result<()> {
    if data.is_empty() {
        return Ok(());
    }

    let mut attempt = 0usize;
    let mut backoff = Duration::from_millis(INITIAL_BACKOFF_MS);
    let max_backoff = Duration::from_millis(MAX_BACKOFF_MS);
    let mut last_error: Option<io::Error> = None;
    let mut offset: usize = 0;

    'attempts: while attempt <= MAX_RETRIES {
        let open_res = OpenOptions::new().create(true).append(true).open(path);
        let mut file = match open_res {
            Ok(mut file) => {
                // Seek to avoid duplicating data on partial write retry
                if offset > 0 {
                    use std::io::Seek;
                    file.seek(std::io::SeekFrom::Start(offset as u64))?;
                }
                file
            }
            Err(e) => {
                if is_retryable_io(&e) {
                    last_error = Some(e);
                    attempt += 1;
                    sleep(backoff);
                    backoff = (backoff * 2).min(max_backoff);
                    continue 'attempts;
                } else {
                    return Err(e);
                }
            }
        };

        while offset < data.len() {
            let end = (offset + WRITE_CHUNK_MAX).min(data.len());
            if let Err(e) = file.write_all(&data[offset..end]) {
                if is_retryable_io(&e) {
                    last_error = Some(e);
                    attempt += 1;
                    sleep(backoff);
                    backoff = (backoff * 2).min(max_backoff);
                    drop(file);
                    continue 'attempts;
                } else {
                    return Err(e);
                }
            }
            offset = end;
        }

        return Ok(());
    }

    Err(last_error.unwrap_or_else(|| io::Error::other("append_with_retry: exhausted retries")))
}

impl ShardBuffer {
    #[inline]
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(1024 * 1024),
        }
    }

    /// Append an 11-byte key to the buffer
    #[inline(always)]
    pub fn append_fast(&mut self, key: &[u8; 11]) {
        self.buffer.extend_from_slice(key);
    }

    /// Write buffer to disk with retry and fsync
    pub fn finalize(&mut self, path: &Path) -> io::Result<()> {
        debug_assert!(
            self.buffer.len().is_multiple_of(11),
            "buffer not multiple of 11, len={}",
            self.buffer.len()
        );
        if self.buffer.is_empty() {
            return Ok(());
        }

        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        append_with_retry(path, &self.buffer)?;

        let file = OpenOptions::new().create(true).append(true).open(path)?;
        file.sync_all()?;

        self.buffer.clear();
        Ok(())
    }

    #[inline]
    pub fn buffered_len(&self) -> usize {
        self.buffer.len()
    }
}
