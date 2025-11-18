// src/mining/vulkan.rs
// developed by bekmen_my
use super::MinerBackend;
pub struct VulkanBackend;
impl MinerBackend for VulkanBackend {
    fn name(&self) -> &'static str { "vulkan" }
    fn init(&mut self) -> anyhow::Result<()> { /* init Vulkan */ Ok(()) }
    fn hash(&self, _input: &[u8]) -> [u8; 32] { [0u8; 32] }
}
