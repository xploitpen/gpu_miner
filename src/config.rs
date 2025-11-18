// src/config.rs
// developed by bekmen_my
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MinerConfig {
    pub algo: String,           // "monero" | "blake2b" | "fisheye" | "duino"
    pub pool_host: String,
    pub pool_port: u16,
    pub rig_name: Option<String>,
    pub backend: String,        // "cuda" | "opencl" | "vulkan" | "gles" | "cpu"
    pub dev_fee_interval: u32,  // e.g., 100 (every 100 accepted shares)
}
