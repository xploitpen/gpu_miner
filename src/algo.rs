// src/algo.rs
// developed by bekmen_my
pub trait JobHandler {
    fn process_job(&mut self, job: &[u8]) -> Vec<u8>; // placeholder
}
