// src/mining/gles.rs
// developed by bekmen_my
use super::MinerBackend;
pub struct GlesBackend;
impl MinerBackend for GlesBackend {
    fn name(&self) -> &'static str { "gles" }
    fn init(&mut self) -> anyhow::Result<()> { /* init GL ES */ Ok(()) }
    fn hash(&self, _input: &[u8]) -> [u8; 32] { [0u8; 32] }
}
