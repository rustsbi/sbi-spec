#![no_std]

// §3
pub mod binary;
// §4
pub mod base;
// §5
pub mod legacy;
// §6
pub mod time;
// §7
pub mod spi;
// §8
pub mod rfnc;
// §9
pub mod hsm;
// §10
pub mod srst;
// §11
pub mod pmu;

/// Converts SBI EID from str.
const fn eid_from_str(name: &str) -> i32 {
    match *name.as_bytes() {
        [a] => a as _,
        [a, b] => (a as i32) << 8 | b as i32,
        [a, b, c] => (a as i32) << 16 | (b as i32) << 8 | c as i32,
        [a, b, c, d] => (a as i32) << 24 | (b as i32) << 16 | (c as i32) << 8 | d as i32,
        _ => unreachable!(),
    }
}
