use indicatif::{ProgressBar, ProgressStyle};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tracing::info;

use crate::transfer::engine::TransferMessage;

pub struct ProgressReporter {
    bar: ProgressBar,
    start_time: Instant,
    last_update: Instant,
    total_bytes: u64,
    bytes_transferred: u64,
    update_interval: Duration,
}

impl ProgressReporter {
    pub fn new(total_bytes: u64, update_interval_secs: u64) -> Self {
        let bar = ProgressBar::new(total_bytes);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
                .unwrap()
                .progress_chars("#>-")
        );
        
        Self {
            bar,
            start_time: Instant::now(),
            last_update: Instant::now(),
            total_bytes,
            bytes_transferred: 0,
            update_interval: Duration::from_secs(update_interval_secs),
        }
    }
    
    pub async fn run(&mut self, mut rx: mpsc::Receiver<TransferMessage>) {
        while let Some(message) = rx.recv().await {
            match message {
                TransferMessage::Progress { bytes_transferred, total_bytes: _ } => {
                    self.bytes_transferred += bytes_transferred;
                    
                    let now = Instant::now();
                    if now.duration_since(self.last_update) >= self.update_interval {
                        self.update_progress();
                        self.last_update = now;
                    }
                }
                TransferMessage::Checksum { algorithm, value } => {
                    info!("Checksum {}: {:x?}", algorithm, value);
                }
                TransferMessage::Complete => {
                    self.bar.finish_with_message("Transfer completed");
                    break;
                }
                TransferMessage::Error(err) => {
                    self.bar.abandon_with_message(format!("Transfer failed: {}", err));
                    break;
                }
            }
        }
    }
    
    fn update_progress(&self) {
        self.bar.set_position(self.bytes_transferred);
        
        let elapsed = self.start_time.elapsed();
        let rate = if elapsed.as_secs() > 0 {
            self.bytes_transferred / elapsed.as_secs()
        } else {
            0
        };
        
        let percentage = if self.total_bytes > 0 {
            (self.bytes_transferred as f64 / self.total_bytes as f64) * 100.0
        } else {
            0.0
        };
        
        if log::log_enabled!(log::Level::Info) {
            info!(
                "Progress: {:.1}% ({} KB/s)",
                percentage,
                rate / 1024
            );
        }
    }
}