use super::events::SolveEvent;
use crossbeam_channel::Receiver;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};
use std::time::{Duration, Instant};

/// Event logger for writing solve events to JSONL and console
pub struct EventLogger {
    receiver: Receiver<SolveEvent>,
    log_file: BufWriter<File>,
    global_total: Arc<AtomicU64>,
    expected_total: u64,
    poll_interval: Duration,
    start_time: Instant,
    enumeration_done: bool,
}

impl EventLogger {
    /// Create event logger that writes to file and tracks progress
    pub fn new(
        receiver: Receiver<SolveEvent>,
        log_path: PathBuf,
        global_total: Arc<AtomicU64>,
        expected_total: u64,
        poll_interval: Duration,
    ) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)?;

        Ok(Self {
            receiver,
            log_file: BufWriter::new(file),
            global_total,
            expected_total,
            poll_interval,
            start_time: Instant::now(),
            enumeration_done: false,
        })
    }

    /// Run event loop until channel is closed
    pub fn run(mut self) -> std::io::Result<()> {
        let mut last_progress_display = Instant::now();

        loop {
            match self.receiver.recv_timeout(self.poll_interval) {
                Ok(event) => {
                    self.update_console_state(&event);

                    let json = serde_json::to_string(&event).map_err(|e| {
                        std::io::Error::other(format!("Failed to serialize event: {}", e))
                    })?;
                    writeln!(self.log_file, "{}", json)?;
                    self.log_file.flush()?;

                    if !self.enumeration_done
                        && last_progress_display.elapsed() >= self.poll_interval
                    {
                        self.display_progress();
                        last_progress_display = Instant::now();
                    }
                }
                Err(crossbeam_channel::RecvTimeoutError::Timeout) => {
                    if !self.enumeration_done {
                        self.display_progress();
                        last_progress_display = Instant::now();
                    }
                }
                Err(crossbeam_channel::RecvTimeoutError::Disconnected) => {
                    break;
                }
            }
        }

        self.log_file.flush()?;
        Ok(())
    }

    /// Update console state based on event
    fn update_console_state(&mut self, event: &SolveEvent) {
        match event {
            SolveEvent::RunStart { layer, .. } => {
                println!("----- Begin Solve (Layer {}) -----", layer);
            }
            SolveEvent::RunResume { layer, .. } => {
                println!("----- Resume Solve (Layer {}) -----", layer);
            }
            SolveEvent::WorkerStart { .. } => {}
            SolveEvent::MaskComplete { .. } => {}
            SolveEvent::ShardSpill { .. } => {}
            SolveEvent::WorkerEnd { .. } => {}
            SolveEvent::RunEnd { .. } => {
                self.enumeration_done = true;
                self.display_progress();
                println!("----- End Solve -----");
            }
        }
    }

    /// Display progress update by polling global counter
    fn display_progress(&self) {
        let current = self.global_total.load(Ordering::Relaxed);
        let percent = if self.expected_total > 0 {
            (current as f64 / self.expected_total as f64) * 100.0
        } else {
            0.0
        };

        let elapsed = self.start_time.elapsed();
        let duration_str = format_duration(elapsed);

        let expected_str = format_number(self.expected_total);
        let width = expected_str.len();

        let current_str = format_number(current);
        let current_padded = format!("{:>width$}", current_str, width = width);

        let percent_str = format!("{:>6.2}", percent);

        println!(
            "Progress: {} / {} ({}%) | Elapsed: {}",
            current_padded, expected_str, percent_str, duration_str
        );
    }
}

fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

fn format_duration(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
