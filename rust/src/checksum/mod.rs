// Checksum algorithms implementation

use anyhow::Result;

pub trait Checksum: Send + Sync {
    fn update(&mut self, data: &[u8]);
    fn finalize(self) -> Vec<u8>;
    fn name(&self) -> &'static str;
}

pub mod md5;
pub mod crc32;
pub mod adler32;

pub enum ChecksumType {
    MD5,
    CRC32,
    Adler32,
}

pub fn create_checksum(checksum_type: ChecksumType) -> Box<dyn Checksum> {
    match checksum_type {
        ChecksumType::MD5 => Box::new(md5::MD5Checksum::new()),
        ChecksumType::CRC32 => Box::new(crc32::CRC32Checksum::new()),
        ChecksumType::Adler32 => Box::new(adler32::Adler32Checksum::new()),
    }
}