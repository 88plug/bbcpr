use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::BbcpError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferState {
    pub transfer_id: String,
    pub source_path: String,
    pub destination_path: String,
    pub total_size: u64,
    pub bytes_transferred: u64,
    pub chunk_states: HashMap<u32, ChunkState>,
    pub checksum: Option<String>,
    pub timestamp: u64,
    pub streams: u32,
    pub compression_level: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkState {
    pub chunk_id: u32,
    pub start_offset: u64,
    pub end_offset: u64,
    pub bytes_completed: u64,
    pub checksum: Option<String>,
    pub completed: bool,
}

impl TransferState {
    pub fn new(
        source: &str,
        destination: &str,
        total_size: u64,
        streams: u32,
        compression_level: Option<u8>,
    ) -> Self {
        let transfer_id = generate_transfer_id(source, destination);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            transfer_id,
            source_path: source.to_string(),
            destination_path: destination.to_string(),
            total_size,
            bytes_transferred: 0,
            chunk_states: HashMap::new(),
            checksum: None,
            timestamp,
            streams,
            compression_level,
        }
    }

    pub fn initialize_chunks(&mut self) {
        let chunk_size = self.total_size / self.streams as u64;
        let mut remaining = self.total_size;

        for i in 0..self.streams {
            let start_offset = i as u64 * chunk_size;
            let current_chunk_size = if i == self.streams - 1 {
                remaining
            } else {
                chunk_size.min(remaining)
            };
            let end_offset = start_offset + current_chunk_size;

            let chunk_state = ChunkState {
                chunk_id: i,
                start_offset,
                end_offset,
                bytes_completed: 0,
                checksum: None,
                completed: false,
            };

            self.chunk_states.insert(i, chunk_state);
            remaining = remaining.saturating_sub(current_chunk_size);
        }
    }

    pub fn update_chunk_progress(&mut self, chunk_id: u32, bytes_completed: u64) {
        if let Some(chunk) = self.chunk_states.get_mut(&chunk_id) {
            chunk.bytes_completed = bytes_completed;
            chunk.completed = bytes_completed >= (chunk.end_offset - chunk.start_offset);
        }
        self.recalculate_total_progress();
    }

    pub fn mark_chunk_complete(&mut self, chunk_id: u32, checksum: Option<String>) {
        if let Some(chunk) = self.chunk_states.get_mut(&chunk_id) {
            chunk.completed = true;
            chunk.checksum = checksum;
        }
        self.recalculate_total_progress();
    }

    pub fn is_complete(&self) -> bool {
        self.chunk_states.values().all(|chunk| chunk.completed)
    }

    pub fn get_completion_percentage(&self) -> f64 {
        if self.total_size == 0 {
            return 100.0;
        }
        (self.bytes_transferred as f64 / self.total_size as f64) * 100.0
    }

    pub fn get_incomplete_chunks(&self) -> Vec<u32> {
        self.chunk_states
            .iter()
            .filter(|(_, chunk)| !chunk.completed)
            .map(|(id, _)| *id)
            .collect()
    }

    fn recalculate_total_progress(&mut self) {
        self.bytes_transferred = self.chunk_states
            .values()
            .map(|chunk| chunk.bytes_completed)
            .sum();
    }

    pub fn save_to_disk(&self) -> Result<(), BbcpError> {
        let state_dir = get_state_directory()?;
        fs::create_dir_all(&state_dir)?;

        let state_file = state_dir.join(format!("{}.json", self.transfer_id));
        let state_json = serde_json::to_string_pretty(self)?;
        fs::write(state_file, state_json)?;

        Ok(())
    }

    pub fn load_from_disk(transfer_id: &str) -> Result<Option<Self>, BbcpError> {
        let state_dir = get_state_directory()?;
        let state_file = state_dir.join(format!("{}.json", transfer_id));

        if !state_file.exists() {
            return Ok(None);
        }

        let state_json = fs::read_to_string(state_file)?;
        let state: TransferState = serde_json::from_str(&state_json)?;
        Ok(Some(state))
    }

    pub fn delete_from_disk(&self) -> Result<(), BbcpError> {
        let state_dir = get_state_directory()?;
        let state_file = state_dir.join(format!("{}.json", self.transfer_id));

        if state_file.exists() {
            fs::remove_file(state_file)?;
        }

        Ok(())
    }

    pub fn find_existing_transfer(source: &str, destination: &str) -> Result<Option<Self>, BbcpError> {
        let transfer_id = generate_transfer_id(source, destination);
        Self::load_from_disk(&transfer_id)
    }

    pub fn list_all_transfers() -> Result<Vec<Self>, BbcpError> {
        let state_dir = get_state_directory()?;
        
        if !state_dir.exists() {
            return Ok(Vec::new());
        }

        let mut transfers = Vec::new();
        for entry in fs::read_dir(state_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(state_json) = fs::read_to_string(&path) {
                    if let Ok(state) = serde_json::from_str::<TransferState>(&state_json) {
                        transfers.push(state);
                    }
                }
            }
        }

        Ok(transfers)
    }
}

fn generate_transfer_id(source: &str, destination: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    source.hash(&mut hasher);
    destination.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

fn get_state_directory() -> Result<PathBuf, BbcpError> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| BbcpError::IoError("Could not determine home directory".into()))?;
    
    Ok(home_dir.join(".bbcpr").join("transfers"))
}

pub fn cleanup_old_transfers(max_age_days: u64) -> Result<usize, BbcpError> {
    let transfers = TransferState::list_all_transfers()?;
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    let max_age_seconds = max_age_days * 24 * 60 * 60;
    let mut cleaned_count = 0;

    for transfer in transfers {
        if current_time.saturating_sub(transfer.timestamp) > max_age_seconds {
            transfer.delete_from_disk()?;
            cleaned_count += 1;
        }
    }

    Ok(cleaned_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_state_creation() {
        let mut state = TransferState::new(
            "/source/file.txt",
            "/dest/file.txt",
            1000,
            4,
            Some(5),
        );

        assert_eq!(state.total_size, 1000);
        assert_eq!(state.streams, 4);
        assert_eq!(state.compression_level, Some(5));
        assert_eq!(state.bytes_transferred, 0);

        state.initialize_chunks();
        assert_eq!(state.chunk_states.len(), 4);
    }

    #[test]
    fn test_chunk_progress_updates() {
        let mut state = TransferState::new(
            "/source/file.txt",
            "/dest/file.txt",
            1000,
            4,
            None,
        );
        state.initialize_chunks();

        state.update_chunk_progress(0, 100);
        assert_eq!(state.bytes_transferred, 100);
        assert_eq!(state.get_completion_percentage(), 10.0);

        state.mark_chunk_complete(0, Some("abc123".to_string()));
        assert!(state.chunk_states[&0].completed);
    }

    #[test]
    fn test_transfer_id_generation() {
        let id1 = generate_transfer_id("/a", "/b");
        let id2 = generate_transfer_id("/a", "/b");
        let id3 = generate_transfer_id("/a", "/c");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }
}