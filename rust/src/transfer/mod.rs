// File transfer engine

use anyhow::Result;
use std::path::Path;

pub mod engine;
pub mod progress;
pub mod state;
pub mod stream;

pub struct TransferOptions {
    pub streams: u32,
    pub buffer_size: usize,
    pub window_size: usize,
    pub compress: Option<u8>,
    pub checksum: bool,
    pub preserve: bool,
    pub force: bool,
    pub resume: bool,
    pub cleanup_on_success: bool,
}