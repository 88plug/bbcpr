use crate::checksum::Checksum;

pub struct MD5Checksum {
    hasher: md5::Md5,
}

impl MD5Checksum {
    pub fn new() -> Self {
        Self {
            hasher: md5::Md5::new(),
        }
    }
}

impl Checksum for MD5Checksum {
    fn update(&mut self, data: &[u8]) {
        use md5::Digest;
        self.hasher.update(data);
    }

    fn finalize(self) -> Vec<u8> {
        use md5::Digest;
        self.hasher.finalize().to_vec()
    }

    fn name(&self) -> &'static str {
        "MD5"
    }
}