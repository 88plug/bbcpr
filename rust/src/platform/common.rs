use crate::error::Result;
use std::path::Path;

pub trait FileSystem {
    fn get_file_size(path: &Path) -> Result<u64>;
    fn get_available_space(path: &Path) -> Result<u64>;
    fn set_permissions(path: &Path, mode: u32) -> Result<()>;
    fn sync_file(path: &Path) -> Result<()>;
    fn get_block_size(path: &Path) -> Result<u64>;
}

pub struct FileMetadata {
    pub size: u64,
    pub modified: u64,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
}

pub fn get_metadata(path: &Path) -> Result<FileMetadata> {
    let metadata = std::fs::metadata(path)?;
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        Ok(FileMetadata {
            size: metadata.len(),
            modified: metadata.mtime() as u64,
            mode: metadata.mode(),
            uid: metadata.uid(),
            gid: metadata.gid(),
        })
    }
    
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        Ok(FileMetadata {
            size: metadata.len(),
            modified: metadata.last_write_time(),
            mode: 0o644, // Default permissions on Windows
            uid: 0,
            gid: 0,
        })
    }
}