// src/detect.rs
// developed by bekmen_my
#[cfg(feature = "cuda")]
pub fn compiled_backend() -> String { "cuda".into() }
#[cfg(all(not(feature = "cuda"), feature = "opencl"))]
pub fn compiled_backend() -> String { "opencl".into() }
#[cfg(all(not(feature = "cuda"), not(feature = "opencl"), feature = "vulkan"))]
pub fn compiled_backend() -> String { "vulkan".into() }
#[cfg(all(not(feature = "cuda"), not(feature = "opencl"), not(feature = "vulkan"), feature = "gles"))]
pub fn compiled_backend() -> String { "gles".into() }
#[cfg(all(not(feature = "cuda"), not(feature = "opencl"), not(feature = "vulkan"), not(feature = "gles")))]
pub fn compiled_backend() -> String { "cpu".into() }
