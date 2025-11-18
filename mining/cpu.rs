// src/mining/cpu.rs
// developed by bekmen_my
use super::MinerBackend;
pub struct CpuBackend;
impl MinerBackend for CpuBackend {
    fn name(&self) -> &'static str { "cpu" }
    fn init(&mut self) -> anyhow::Result<()> { Ok(()) }
    fn hash(&self, _input: &[u8]) -> [u8; 32] { [0u8; 32] }
}
