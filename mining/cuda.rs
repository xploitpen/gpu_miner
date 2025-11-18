// src/mining/cuda.rs
// developed by bekmen_my
use super::MinerBackend;
pub struct CudaBackend;
impl MinerBackend for CudaBackend {
    fn name(&self) -> &'static str { "cuda" }
    fn init(&mut self) -> anyhow::Result<()> { /* init CUDA */ Ok(()) }
    fn hash(&self, _input: &[u8]) -> [u8; 32] { [0u8; 32] }
}
