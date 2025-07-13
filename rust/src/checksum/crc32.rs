use crate::checksum::Checksum;

pub struct CRC32Checksum {
    hasher: crc32fast::Hasher,
}

impl CRC32Checksum {
    pub fn new() -> Self {
        Self {
            hasher: crc32fast::Hasher::new(),
        }
    }
}

impl Checksum for CRC32Checksum {
    fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }

    fn finalize(self) -> Vec<u8> {
        self.hasher.finalize().to_le_bytes().to_vec()
    }

    fn name(&self) -> &'static str {
        "CRC32"
    }
}