//! Chapter 11. Performance Monitoring Unit Extension (EID #0x504D55 "PMU")

/// Extension ID for Performance Monitoring Unit extension.
pub const EID_PMU: usize = crate::eid_from_str("PMU") as _;
pub use fid::*;

/// Declared in §11.11.
mod fid {
    /// Function ID to get the number of counters, both hardware and firmware.
    ///
    /// Declared in §11.5.
    pub const PMU_NUM_COUNTERS: usize = 0;
    /// Function ID to get details about the specified counter.
    ///
    /// Declared in §11.6.
    pub const PMU_COUNTER_GET_INFO: usize = 1;
    /// Function ID to find and configure a counter from a set of counters.
    ///
    /// Declared in §11.7.
    pub const PMU_COUNTER_CONFIG_MATCHING: usize = 2;
    /// Function ID to start or enable a set of counters on the calling hart with the specified initial value.
    ///
    /// Declared in §11.8.
    pub const PMU_COUNTER_START: usize = 3;
    /// Function ID to stop or disable a set of counters on the calling hart.
    ///
    /// Declared in §11.9.
    pub const PMU_COUNTER_STOP: usize = 4;
    /// Function ID to provide the current value of a firmware counter.
    ///
    /// Declared in §11.10.
    pub const PMU_COUNTER_FW_READ: usize = 5;
    /// Function ID to provide the upper 32 bits of the current firmware counter value.
    ///
    /// Declared in §11.11.
    pub const PMU_COUNTER_FW_READ_HI: usize = 6;
}

/// PMU Event Types.
///
/// Declared in §11.
pub mod event_idx_type {
    /// Type for all hardware general events.
    ///
    /// Declared in §11.1.
    pub const PMU_EVENT_TYPE_HW: usize = 0;
    /// Type for all hardware cache events.
    ///
    /// Declared in §11.2.
    pub const PMU_EVENT_TYPE_HW_CACHE: usize = 1;
    /// Type for all hardware raw events.
    ///
    /// Declared in §11.3.
    pub const PMU_EVENT_TYPE_HW_RAW: usize = 2;
    /// Type for for all firmware events.
    ///
    /// Declared in §11.4.
    pub const PMU_EVENT_TYPE_FW: usize = 15;
}

/// Hardware General Event Codes.
///
/// Declared in §11.1.
pub mod hw_event_code {
    /// Unused event because event_idx cannot be zero
    pub const PMU_HW_NO_EVENT: usize = 0;
    /// Event for each CPU cycle
    pub const PMU_HW_CPU_CYCLES: usize = 1;
    /// Event for each completed instruction
    pub const PMU_HW_INSTRUCTIONS: usize = 2;
    /// Event for cache hit
    pub const PMU_HW_CACHE_REFERENCES: usize = 3;
    /// Event for cache miss
    pub const PMU_HW_CACHE_MISSES: usize = 4;
    /// Event for a branch instruction
    pub const PMU_HW_BRANCH_INSTRUCTIONS: usize = 5;
    /// Event for a branch misprediction
    pub const PMU_HW_BRANCH_MISSES: usize = 6;
    /// Event for each BUS cycle
    pub const PMU_HW_BUS_CYCLES: usize = 7;
    /// Event for a stalled cycle in microarchitecture frontend
    pub const PMU_HW_STALLED_CYCLES_FRONTEND: usize = 8;
    /// Event for a stalled cycle in microarchitecture backend
    pub const PMU_HW_STALLED_CYCLES_BACKEND: usize = 9;
    /// Event for each reference CPU cycle
    pub const PMU_HW_REF_CPU_CYCLES: usize = 10;
}

/// Hardware Cache Event ID
///
/// Declared in §11.2.
pub mod hw_cache_id {
    /// Level1 data cache event
    pub const PMU_HW_CACHE_L1D: usize = 0;
    /// Level1 instruction cache event
    pub const PMU_HW_CACHE_L1I: usize = 1;
    /// Last level cache event
    pub const PMU_HW_CACHE_LL: usize = 2;
    /// Data TLB event
    pub const PMU_HW_CACHE_DTLB: usize = 3;
    /// Instruction TLB event
    pub const PMU_HW_CACHE_ITLB: usize = 4;
    /// Branch predictor unit event
    pub const PMU_HW_CACHE_BPU: usize = 5;
    /// NUMA node cache event
    pub const PMU_HW_CACHE_NODE: usize = 6;
}

/// Hardware Cache Operation ID
///
/// Declared in §11.2.
pub mod hw_cache_op_id {
    /// Read cache line
    pub const PMU_HW_CACHE_OP_READ: usize = 0;
    /// Write cache line
    pub const PMU_HW_CACHE_OP_WRITE: usize = 1;
    /// Prefetch cache line
    pub const PMU_HW_CACHE_OP_PREFETCH: usize = 2;
}

/// Hardware Cache Operation Result ID
///
/// Declared in §11.2.
pub mod hw_cache_op_result_id {
    /// Cache access
    pub const PMU_HW_CACHE_RESULT_ACCESS: usize = 0;
    /// Cache miss
    pub const PMU_HW_CACHE_RESULT_MISS: usize = 1;
}

/// Firmware Event Codes.
///
/// Declared in §11.4.
pub mod fw_event_code {
    /// Misaligned load trap event
    pub const PMU_FW_MISALIGNED_LOAD: usize = 0;
    /// Misaligned store trap event
    pub const PMU_FW_MISALIGNED_STORE: usize = 1;
    /// Load access trap event
    pub const PMU_FW_ACCESS_LOAD: usize = 2;
    /// Store access trap event
    pub const PMU_FW_ACCESS_STORE: usize = 3;
    /// Illegal instruction trap event
    pub const PMU_FW_ILLEGAL_INSN: usize = 4;
    /// Set timer event
    pub const PMU_FW_SET_TIMER: usize = 5;
    /// Sent IPI to other HART event
    pub const PMU_FW_IPI_SENT: usize = 6;
    /// Received IPI from other HART event
    pub const PMU_FW_IPI_RECEIVED: usize = 7;
    /// Sent FENCE.I request to other HART event
    pub const PMU_FW_FENCE_I_SENT: usize = 8;
    /// Received FENCE.I request from other HART event
    pub const PMU_FW_FENCE_I_RECEIVED: usize = 9;
    /// Sent SFENCE.VMA request to other HART event
    pub const PMU_FW_SFENCE_VMA_SENT: usize = 10;
    /// Received SFENCE.VMA request from other HART event
    pub const PMU_FW_SFENCE_VMA_RECEIVED: usize = 11;
    /// Sent SFENCE.VMA with ASID request to other HART event
    pub const PMU_FW_SFENCE_VMA_ASID_SENT: usize = 12;
    /// Received SFENCE.VMA with ASID request from other HART event
    pub const PMU_FW_SFENCE_VMA_ASID_RECEIVED: usize = 13;
    /// Sent HFENCE.GVMA request to other HART event
    pub const PMU_FW_HFENCE_GVMA_SENT: usize = 14;
    /// Received HFENCE.GVMA request from other HART event
    pub const PMU_FW_HFENCE_GVMA_RECEIVED: usize = 15;
    /// Sent HFENCE.GVMA with VMID request to other HART event
    pub const PMU_FW_HFENCE_GVMA_VMID_SENT: usize = 16;
    /// Received HFENCE.GVMA with VMID request from other HART event
    pub const PMU_FW_HFENCE_GVMA_VMID_RECEIVED: usize = 17;
    /// Sent HFENCE.VVMA request to other HART event
    pub const PMU_FW_HFENCE_VVMA_SENT: usize = 18;
    /// Received HFENCE.VVMA request from other HART event
    pub const PMU_FW_HFENCE_VVMA_RECEIVED: usize = 19;
    /// Sent HFENCE.VVMA with ASID request to other HART event
    pub const PMU_FW_HFENCE_VVMA_ASID_SENT: usize = 20;
    /// Received HFENCE.VVMA with ASID request from other HART event
    pub const PMU_FW_HFENCE_VVMA_ASID_RECEIVED: usize = 21;
    /// RISC-V platform specific firmware events, where the `event_data` configuration (or parameter) contains the event encoding.
    pub const PMU_FW_PLATFORM: usize = 65535;
}
