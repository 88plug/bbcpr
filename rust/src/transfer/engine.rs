use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tracing::{debug, info, warn};

use crate::checksum::Checksum;
use crate::network::Connection;
use crate::transfer::{TransferOptions, state::{TransferState, ChunkState}};

pub struct TransferEngine {
    options: TransferOptions,
    source_path: PathBuf,
    destination_path: PathBuf,
}

#[derive(Debug)]
pub enum TransferMessage {
    Progress { bytes_transferred: u64, total_bytes: u64 },
    Checksum { algorithm: String, value: Vec<u8> },
    Complete,
    Error(String),
    Resumed { previous_bytes: u64 },
}

impl TransferEngine {
    pub fn new(source: PathBuf, destination: PathBuf, options: TransferOptions) -> Self {
        Self {
            options,
            source_path: source,
            destination_path: destination,
        }
    }

    pub async fn transfer<C: Connection + 'static>(
        &self,
        mut connection: C,
        progress_tx: mpsc::Sender<TransferMessage>,
    ) -> Result<()> {
        info!("Starting transfer from {:?} to {:?}", self.source_path, self.destination_path);
        
        // Connect to remote if needed
        connection.connect().await
            .context("Failed to establish connection")?;

        // Get file metadata
        let metadata = tokio::fs::metadata(&self.source_path).await
            .context("Failed to read source file metadata")?;
        let total_size = metadata.len();

        // Check for existing transfer state
        let mut transfer_state = if self.options.resume {
            self.load_or_create_transfer_state(total_size).await?
        } else {
            self.create_new_transfer_state(total_size).await?
        };

        // Save initial state to disk
        transfer_state.save_to_disk()
            .context("Failed to save transfer state")?;

        // Check if we're resuming
        if transfer_state.bytes_transferred > 0 {
            info!("Resuming transfer from {} bytes ({:.1}% complete)", 
                  transfer_state.bytes_transferred, 
                  transfer_state.get_completion_percentage());
            
            let _ = progress_tx.send(TransferMessage::Resumed { 
                previous_bytes: transfer_state.bytes_transferred 
            }).await;
        }

        // Start parallel transfer streams for incomplete chunks only
        let incomplete_chunks = transfer_state.get_incomplete_chunks();
        let mut handles: Vec<JoinHandle<Result<()>>> = Vec::new();

        for chunk_id in incomplete_chunks {
            if let Some(chunk_state) = transfer_state.chunk_states.get(&chunk_id) {
                let handle = self.spawn_transfer_stream(
                    chunk_id,
                    chunk_state,
                    &transfer_state,
                    progress_tx.clone(),
                );
                handles.push(handle);
            }
        }

        // If no incomplete chunks, we're already done
        if handles.is_empty() {
            info!("Transfer already complete, cleaning up state");
            if self.options.cleanup_on_success {
                transfer_state.delete_from_disk()
                    .context("Failed to cleanup transfer state")?;
            }
            let _ = progress_tx.send(TransferMessage::Complete).await;
            return Ok(());
        }

        // Wait for all streams to complete
        for handle in handles {
            handle.await
                .context("Transfer stream panicked")?
                .context("Transfer stream failed")?;
        }

        // Final verification and cleanup
        if self.options.checksum {
            self.verify_transfer_checksum(&transfer_state).await?;
        }

        // Clean up state file on successful completion
        if self.options.cleanup_on_success {
            transfer_state.delete_from_disk()
                .context("Failed to cleanup transfer state")?;
        }

        // Send completion message
        let _ = progress_tx.send(TransferMessage::Complete).await;
        
        info!("Transfer completed successfully");
        Ok(())
    }

    async fn load_or_create_transfer_state(&self, total_size: u64) -> Result<TransferState> {
        let source_str = self.source_path.to_string_lossy();
        let dest_str = self.destination_path.to_string_lossy();

        // Try to load existing state
        if let Some(existing_state) = TransferState::find_existing_transfer(&source_str, &dest_str)? {
            // Validate that the existing state matches current parameters
            if existing_state.total_size == total_size 
                && existing_state.streams == self.options.streams 
                && existing_state.compression_level == self.options.compress {
                
                info!("Found existing transfer state, resuming from {:.1}% complete",
                      existing_state.get_completion_percentage());
                return Ok(existing_state);
            } else {
                warn!("Existing transfer state incompatible with current parameters, starting fresh");
                existing_state.delete_from_disk()?;
            }
        }

        // Create new state
        self.create_new_transfer_state(total_size).await
    }

    async fn create_new_transfer_state(&self, total_size: u64) -> Result<TransferState> {
        let source_str = self.source_path.to_string_lossy();
        let dest_str = self.destination_path.to_string_lossy();

        let mut state = TransferState::new(
            &source_str,
            &dest_str,
            total_size,
            self.options.streams,
            self.options.compress,
        );

        state.initialize_chunks();
        Ok(state)
    }

    fn spawn_transfer_stream(
        &self,
        chunk_id: u32,
        chunk_state: &ChunkState,
        transfer_state: &TransferState,
        progress_tx: mpsc::Sender<TransferMessage>,
    ) -> JoinHandle<Result<()>> {
        let source_path = self.source_path.clone();
        let dest_path = self.destination_path.clone();
        let buffer_size = self.options.buffer_size;
        let chunk_state = chunk_state.clone();
        let transfer_id = transfer_state.transfer_id.clone();
        
        tokio::spawn(async move {
            // Calculate actual transfer range (accounting for already completed bytes)
            let start_offset = chunk_state.start_offset + chunk_state.bytes_completed;
            let end_offset = chunk_state.end_offset;
            let remaining_bytes = end_offset - start_offset;

            if remaining_bytes == 0 {
                debug!("Chunk {} already complete, skipping", chunk_id);
                return Ok(());
            }

            debug!("Stream {} resuming transfer of bytes {}-{} (remaining: {})", 
                   chunk_id, start_offset, end_offset, remaining_bytes);
            
            use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
            use std::io::SeekFrom;

            // Open source file
            let mut source_file = tokio::fs::File::open(&source_path).await
                .context("Failed to open source file")?;

            // Seek to the correct position
            source_file.seek(SeekFrom::Start(start_offset)).await
                .context("Failed to seek in source file")?;

            // Open/create destination file for this chunk
            let mut dest_file = tokio::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(&dest_path).await
                .context("Failed to open destination file")?;

            // Seek to the correct position in destination
            dest_file.seek(SeekFrom::Start(start_offset)).await
                .context("Failed to seek in destination file")?;

            // Transfer data
            let mut buffer = vec![0u8; buffer_size];
            let mut bytes_transferred = 0u64;
            let mut total_chunk_bytes = chunk_state.bytes_completed;

            while bytes_transferred < remaining_bytes {
                let to_read = buffer_size.min((remaining_bytes - bytes_transferred) as usize);
                let bytes_read = source_file.read(&mut buffer[..to_read]).await
                    .context("Failed to read from source file")?;

                if bytes_read == 0 {
                    break; // EOF
                }

                dest_file.write_all(&buffer[..bytes_read]).await
                    .context("Failed to write to destination file")?;

                bytes_transferred += bytes_read as u64;
                total_chunk_bytes += bytes_read as u64;

                // Update progress periodically
                if bytes_transferred % (buffer_size as u64 * 10) == 0 {
                    // Load current state, update, and save
                    if let Ok(Some(mut state)) = TransferState::load_from_disk(&transfer_id) {
                        state.update_chunk_progress(chunk_id, total_chunk_bytes);
                        let _ = state.save_to_disk();
                        
                        let _ = progress_tx.send(TransferMessage::Progress {
                            bytes_transferred: state.bytes_transferred,
                            total_bytes: state.total_size,
                        }).await;
                    }
                }
            }

            // Flush and sync the destination file
            dest_file.flush().await
                .context("Failed to flush destination file")?;
            dest_file.sync_all().await
                .context("Failed to sync destination file")?;

            // Mark chunk as complete
            if let Ok(Some(mut state)) = TransferState::load_from_disk(&transfer_id) {
                state.mark_chunk_complete(chunk_id, None); // TODO: Add chunk-level checksums
                let _ = state.save_to_disk();
                
                let _ = progress_tx.send(TransferMessage::Progress {
                    bytes_transferred: state.bytes_transferred,
                    total_bytes: state.total_size,
                }).await;
            }

            debug!("Stream {} completed successfully", chunk_id);
            Ok(())
        })
    }

    async fn verify_transfer_checksum(&self, _transfer_state: &TransferState) -> Result<()> {
        // TODO: Implement comprehensive checksum verification
        // This would compare source and destination file checksums
        debug!("Checksum verification not yet implemented");
        Ok(())
    }

    pub async fn list_pending_transfers() -> Result<Vec<TransferState>> {
        TransferState::list_all_transfers()
            .context("Failed to list transfer states")
    }

    pub async fn resume_transfer(transfer_id: &str) -> Result<Option<TransferState>> {
        TransferState::load_from_disk(transfer_id)
            .context("Failed to load transfer state")
    }

    pub async fn cancel_transfer(transfer_id: &str) -> Result<bool> {
        if let Some(state) = TransferState::load_from_disk(transfer_id)? {
            state.delete_from_disk()
                .context("Failed to delete transfer state")?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn cleanup_old_transfers(max_age_days: u64) -> Result<usize> {
        crate::transfer::state::cleanup_old_transfers(max_age_days)
            .context("Failed to cleanup old transfers")
    }
}