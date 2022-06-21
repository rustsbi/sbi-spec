//! Chapter 4. Base Extension (EID #0x10)

pub const EID_BASE: usize = 0x10;
pub use fid::*;

/// §4.8
mod fid {
    /// §4.1
    pub const GET_SPEC_VERSION: usize = 0x0;
    /// §4.2
    pub const GET_SBI_IMPL_ID: usize = 0x1;
    /// §4.3
    pub const GET_SBI_IMPL_VERSION: usize = 0x2;
    /// §4.4
    pub const PROBE_EXTENSION: usize = 0x3;
    /// §4.5
    pub const GET_MVENDORID: usize = 0x4;
    /// §4.6
    pub const GET_MARCHID: usize = 0x5;
    /// §4.7
    pub const GET_MIMPID: usize = 0x6;
}

/// §4.9
pub mod impl_id {
    pub const IMPL_BBL: usize = 0;
    pub const IMPL_OPEN_SBI: usize = 1;
    pub const IMPL_XVISOR: usize = 2;
    pub const IMPL_KVM: usize = 3;
    pub const IMPL_RUST_SBI: usize = 4;
    pub const IMPL_DIOSIX: usize = 5;
    pub const IMPL_COFFER: usize = 6;
}
