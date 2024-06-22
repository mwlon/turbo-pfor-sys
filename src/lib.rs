// prerequisites: Install TurboPFor library libic under /usr/lib or /usr/local/lib
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn test_round_trip() {
        const n: usize = 3;
        let mut nums = vec![33_u32, 44, 77];
        let mut compressed = vec![0_u8; 1000];
        let mut recovered = vec![0_u32; n];
        unsafe {
            crate::p4nd1enc128v32(nums.as_mut_ptr(), 3, compressed.as_mut_ptr());
            crate::p4nd1dec128v32(compressed.as_mut_ptr(), n, recovered.as_mut_ptr());
        }
        assert_eq!(recovered, nums);
    }
}
