use crate::checksum::Checksum;

pub struct Adler32Checksum {
    hasher: adler::Adler32,
}

impl Adler32Checksum {
    pub fn new() -> Self {
        Self {
            hasher: adler::Adler32::new(),
        }
    }
}

impl Checksum for Adler32Checksum {
    fn update(&mut self, data: &[u8]) {
        self.hasher.write_slice(data);
    }

    fn finalize(self) -> Vec<u8> {
        self.hasher.finish().to_le_bytes().to_vec()
    }

    fn name(&self) -> &'static str {
        "Adler32"
    }
}