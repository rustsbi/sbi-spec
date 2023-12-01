//! Chapter 15. Nested Acceleration Extension (EID #0x4E41434C "NACL").

/// Extension ID for Nested Acceleration Extension.
pub const EID_NACL: usize = crate::eid_from_str("NACL") as _;
pub use fid::*;

/// Declared in § 15.15.
mod fid {
    /// Function ID to probe a nested acceleration feature.
    ///
    /// Declared in §15.5.
    pub const PROBE_FEATURE: usize = 0;
    /// Function ID to set and enable the shared memory for nested acceleration on the calling hart.
    ///
    /// Declared in §15.6.
    pub const SET_SHMEM: usize = 1;
    /// Function ID to synchronize CSRs in the nested acceleration shared memory.
    ///
    /// Declared in §15.7.
    pub const SYNC_CSR: usize = 2;
    /// Function ID to synchronize HFENCEs in the nested acceleration shared memory.
    ///
    /// Declared in §15.8.
    pub const SYNC_HFENCE: usize = 3;
    /// Function ID to synchronize CSRs and HFENCEs in the nested acceleration shared memory and emulate the SRET instruction.
    ///
    /// Declared in §15.9.
    pub const SYNC_SRET: usize = 4;
}

/// Nested Acceleration Feature ID.
///
/// Declared in §15.
pub mod feature_id {
    /// Feature ID for the synchronize CSR feature.
    ///
    /// Declared in §15.1.
    pub const SYNC_CSR: usize = 0;
    /// Feature ID for the synchronize HFENCE feature.
    ///
    /// Declared in §15.2.
    pub const SYNC_HFENCE: usize = 1;
    /// Feature ID for the synchronize SRET feature.
    ///
    /// Declared in §15.3.
    pub const SYNC_SRET: usize = 2;
    /// Feature ID for the autoswap CSR feature.
    ///
    /// Declared in §15.4.
    pub const AUTOSWAP_CSR: usize = 3;
}
