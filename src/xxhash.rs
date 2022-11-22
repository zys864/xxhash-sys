#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
use std::{ffi::c_void, mem::MaybeUninit};

include!(concat!(env!("OUT_DIR"), "/xxhash_bindings.rs"));

#[derive(Debug, Clone)]
pub struct XxHash64 {
    state: XXH64_state_t,
}

impl XxHash64 {
    pub fn new() -> Self {
        Self::with_seed(0)
    }

    pub fn with_seed(seed: u64) -> XxHash64 {
        unsafe {
            let mut r = MaybeUninit::<XXH64_state_t>::uninit();
            let err_code = XXH64_reset(r.as_mut_ptr() as *mut XXH64_state_t, seed);
            if err_code == XXH_errorcode_XXH_ERROR{
                panic!("XXH64_reset error");
            }
            XxHash64 {
                state: r.assume_init(),
            }
        }
    }
    pub fn hash_with_seed(seed: u64, bytes: &[u8]) -> u64 {
        unsafe { XXH64(bytes.as_ptr() as *const c_void, bytes.len(), seed) }
    }
    pub fn hash(bytes: &[u8]) -> u64 {
        Self::hash_with_seed(0, bytes)
    }

    pub fn write(&mut self, bytes: &[u8]) {
        unsafe {
            let err_code = XXH64_update(
                &mut self.state,
                bytes.as_ptr() as *const c_void,
                bytes.len(),
            );
            if err_code == XXH_errorcode_XXH_ERROR{
                panic!("XXH64_update error");
            }
        }
    }

    pub fn finish(&self) -> u64 {
        unsafe { XXH64_digest(&self.state) }
    }
}
impl Drop for XxHash64{
    fn drop(&mut self) {
        unsafe{
            XXH64_freeState(&mut self.state);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quickhash() {
        let s = "123456";
        let h = XxHash64::hash(s.as_bytes());
        println!("{}", h);
    }
    #[test]
    fn test_streaming_hash() {
        let mut hasher = XxHash64::with_seed(0);
        dbg!(&hasher);
        let s = "123456";
        hasher.write(s.as_bytes());
        let r = hasher.finish();
        println!("{}", r);
    }
}
