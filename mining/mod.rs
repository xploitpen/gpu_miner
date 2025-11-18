// src/mining/mod.rs
// developed by bekmen_my
pub trait MinerBackend {
    fn name(&self) -> &'static str;
    fn init(&mut self) -> anyhow::Result<()>;
    fn hash(&self, input: &[u8]) -> [u8; 32]; // placeholder per algorithm
}

#[cfg(feature = "cuda")] pub mod cuda;
#[cfg(feature = "opencl")] pub mod opencl;
#[cfg(feature = "vulkan")] pub mod vulkan;
#[cfg(feature = "gles")] pub mod gles;
#[cfg(feature = "cpu")] pub mod cpu;
