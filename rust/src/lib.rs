// bbcpr library - Core functionality
// Copyright (C) 2025 Andrew Mello <andrew@88plug.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

pub mod checksum;
pub mod error;
pub mod network;
pub mod platform;
pub mod transfer;

pub use crate::error::{BbcprError, Result};

/// bbcpr version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default number of parallel streams
pub const DEFAULT_STREAMS: u32 = 4;

/// Default buffer size (128KB)
pub const DEFAULT_BUFFER_SIZE: usize = 128 * 1024;

/// Default window size (128KB)
pub const DEFAULT_WINDOW_SIZE: usize = 128 * 1024;