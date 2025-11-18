// src/mining/opencl.rs
// developed by bekmen_my
use super::MinerBackend;
pub struct OpenClBackend;
impl MinerBackend for OpenClBackend {
    fn name(&self) -> &'static str { "opencl" }
    fn init(&mut self) -> anyhow::Result<()> { /* init OpenCL */ Ok(()) }
    fn hash(&self, _input: &[u8]) -> [u8; 32] { [0u8; 32] }
}
