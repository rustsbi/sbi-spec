//! RISC-V SBI Specification structure and constant definitions.
//!
//! This crate adapts to RISC-V SBI Specification verion 2.0-rc1.
//! It provides structures in Rust semantics and best practices to simplify
//! designs of RISC-V SBI ecosystem, both implementation and applications.
//!
//! You may find it convenient to use this library in a vast range of packages,
//! from operating system kernels, hypervisors, to SBI bare metal implementations.
//! This crate is `no_std` compatible and does not need dymanic memory allocation,
//! which make it suitable for embedded development.
//!
//! Although this library is dedicated to RISC-V architecture, it does not limit
//! which build target the dependents should compile into. For example, you are
//! developing a RISC-V emulator on platforms other than RISC-V, the emulator
//! designed on other platforms can still make use of `sbi-spec` structures to
//! provide necessary features the emulated RISC-V environment would make use of.
#![no_std]
#![deny(missing_docs, unsafe_code, unstable_features)]

// §3
pub mod binary;
// §4
pub mod base;
// §5
#[cfg(feature = "legacy")]
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
// §12
pub mod dbcn;
// §13
pub mod susp;
// §14
pub mod cppc;
// §15
pub mod nacl;
// §16
pub mod sta;

/// Converts SBI EID from str.
const fn eid_from_str(name: &str) -> i32 {
    match *name.as_bytes() {
        [a] => i32::from_be_bytes([0, 0, 0, a]),
        [a, b] => i32::from_be_bytes([0, 0, a, b]),
        [a, b, c] => i32::from_be_bytes([0, a, b, c]),
        [a, b, c, d] => i32::from_be_bytes([a, b, c, d]),
        _ => unreachable!(),
    }
}

/// Checks during compilation, and provides an item list for developers.
#[cfg(test)]
mod tests {
    use static_assertions::{
        assert_eq_align, assert_eq_size, assert_fields, assert_impl_all, const_assert_eq,
    };
    // §3
    #[test]
    fn test_binary() {
        use crate::binary::*;
        assert_eq_align!(SbiRet, usize);
        assert_eq_size!(SbiRet, [usize; 2]);
        assert_fields!(SbiRet: error);
        assert_fields!(SbiRet: value);
        assert_impl_all!(SbiRet: Copy, Clone, PartialEq, Eq, core::fmt::Debug);

        const_assert_eq!(0, RET_SUCCESS as isize);
        const_assert_eq!(-1, RET_ERR_FAILED as isize);
        const_assert_eq!(-2, RET_ERR_NOT_SUPPORTED as isize);
        const_assert_eq!(-3, RET_ERR_INVALID_PARAM as isize);
        const_assert_eq!(-4, RET_ERR_DENIED as isize);
        const_assert_eq!(-5, RET_ERR_INVALID_ADDRESS as isize);
        const_assert_eq!(-6, RET_ERR_ALREADY_AVAILABLE as isize);
        const_assert_eq!(-7, RET_ERR_ALREADY_STARTED as isize);
        const_assert_eq!(-8, RET_ERR_ALREADY_STOPPED as isize);
    }
    // §4
    #[test]
    fn test_base() {
        use crate::base::*;
        const_assert_eq!(0x10, EID_BASE);
        const_assert_eq!(0, GET_SBI_SPEC_VERSION);
        const_assert_eq!(1, GET_SBI_IMPL_ID);
        const_assert_eq!(2, GET_SBI_IMPL_VERSION);
        const_assert_eq!(3, PROBE_EXTENSION);
        const_assert_eq!(4, GET_MVENDORID);
        const_assert_eq!(5, GET_MARCHID);
        const_assert_eq!(6, GET_MIMPID);
        const_assert_eq!(0, impl_id::BBL);
        const_assert_eq!(1, impl_id::OPEN_SBI);
        const_assert_eq!(2, impl_id::XVISOR);
        const_assert_eq!(3, impl_id::KVM);
        const_assert_eq!(4, impl_id::RUST_SBI);
        const_assert_eq!(5, impl_id::DIOSIX);
        const_assert_eq!(6, impl_id::COFFER);
    }
    // §5
    #[cfg(feature = "legacy")]
    #[test]
    fn test_legacy() {
        use crate::legacy::*;
        const_assert_eq!(0, LEGACY_SET_TIMER);
        const_assert_eq!(1, LEGACY_CONSOLE_PUTCHAR);
        const_assert_eq!(2, LEGACY_CONSOLE_GETCHAR);
        const_assert_eq!(3, LEGACY_CLEAR_IPI);
        const_assert_eq!(4, LEGACY_SEND_IPI);
        const_assert_eq!(5, LEGACY_REMOTE_FENCE_I);
        const_assert_eq!(6, LEGACY_REMOTE_SFENCE_VMA);
        const_assert_eq!(7, LEGACY_REMOTE_SFENCE_VMA_ASID);
        const_assert_eq!(8, LEGACY_SHUTDOWN);
    }
    // §6
    #[test]
    fn test_time() {
        use crate::time::*;
        const_assert_eq!(0x54494D45, EID_TIME);
        const_assert_eq!(0, SET_TIMER);
    }
    // §7
    #[test]
    fn test_spi() {
        use crate::spi::*;
        const_assert_eq!(0x735049, EID_SPI);
        const_assert_eq!(0, SEND_IPI);
    }
    // §8
    #[test]
    fn test_rfnc() {
        use crate::rfnc::*;
        const_assert_eq!(0x52464E43, EID_RFNC);
        const_assert_eq!(0, REMOTE_FENCE_I);
        const_assert_eq!(1, REMOTE_SFENCE_VMA);
        const_assert_eq!(2, REMOTE_SFENCE_VMA_ASID);
        const_assert_eq!(3, REMOTE_HFENCE_GVMA_VMID);
        const_assert_eq!(4, REMOTE_HFENCE_GVMA);
        const_assert_eq!(5, REMOTE_HFENCE_VVMA_ASID);
        const_assert_eq!(6, REMOTE_HFENCE_VVMA);
    }
    // §9
    #[test]
    fn test_hsm() {
        use crate::hsm::*;
        const_assert_eq!(0x48534D, EID_HSM);
        const_assert_eq!(0, HART_STATE_STARTED);
        const_assert_eq!(1, HART_STATE_STOPPED);
        const_assert_eq!(2, HART_STATE_START_PENDING);
        const_assert_eq!(3, HART_STATE_STOP_PENDING);
        const_assert_eq!(4, HART_STATE_SUSPENDED);
        const_assert_eq!(5, HART_STATE_SUSPEND_PENDING);
        const_assert_eq!(6, HART_STATE_RESUME_PENDING);
        const_assert_eq!(0x0000_0000, HART_SUSPEND_TYPE_RETENTIVE);
        const_assert_eq!(0x8000_0000, HART_SUSPEND_TYPE_NON_RETENTIVE);
        const_assert_eq!(0, HART_START);
        const_assert_eq!(1, HART_STOP);
        const_assert_eq!(2, HART_GET_STATUS);
        const_assert_eq!(3, HART_SUSPEND);
    }
    // §10
    #[test]
    fn test_srst() {
        use crate::srst::*;
        const_assert_eq!(0x53525354, EID_SRST);
        const_assert_eq!(0, RESET_TYPE_SHUTDOWN);
        const_assert_eq!(1, RESET_TYPE_COLD_REBOOT);
        const_assert_eq!(2, RESET_TYPE_WARM_REBOOT);
        const_assert_eq!(0, RESET_REASON_NO_REASON);
        const_assert_eq!(1, RESET_REASON_SYSTEM_FAILURE);
        const_assert_eq!(0, SYSTEM_RESET);
    }
    // §11
    #[test]
    fn test_pmu() {
        use crate::pmu::*;
        const_assert_eq!(0x504D55, EID_PMU);
        const_assert_eq!(0, PMU_NUM_COUNTERS);
        const_assert_eq!(1, PMU_COUNTER_GET_INFO);
        const_assert_eq!(2, PMU_COUNTER_CONFIG_MATCHING);
        const_assert_eq!(3, PMU_COUNTER_START);
        const_assert_eq!(4, PMU_COUNTER_STOP);
        const_assert_eq!(5, PMU_COUNTER_FW_READ);
        const_assert_eq!(6, PMU_COUNTER_FW_READ_HI);

        const_assert_eq!(0, event_idx_type::PMU_EVENT_TYPE_HW);
        const_assert_eq!(1, event_idx_type::PMU_EVENT_TYPE_HW_CACHE);
        const_assert_eq!(2, event_idx_type::PMU_EVENT_TYPE_HW_RAW);
        const_assert_eq!(15, event_idx_type::PMU_EVENT_TYPE_FW);

        const_assert_eq!(0, hw_event_code::PMU_HW_NO_EVENT);
        const_assert_eq!(1, hw_event_code::PMU_HW_CPU_CYCLES);
        const_assert_eq!(2, hw_event_code::PMU_HW_INSTRUCTIONS);
        const_assert_eq!(3, hw_event_code::PMU_HW_CACHE_REFERENCES);
        const_assert_eq!(4, hw_event_code::PMU_HW_CACHE_MISSES);
        const_assert_eq!(5, hw_event_code::PMU_HW_BRANCH_INSTRUCTIONS);
        const_assert_eq!(6, hw_event_code::PMU_HW_BRANCH_MISSES);
        const_assert_eq!(7, hw_event_code::PMU_HW_BUS_CYCLES);
        const_assert_eq!(8, hw_event_code::PMU_HW_STALLED_CYCLES_FRONTEND);
        const_assert_eq!(9, hw_event_code::PMU_HW_STALLED_CYCLES_BACKEND);
        const_assert_eq!(10, hw_event_code::PMU_HW_REF_CPU_CYCLES);

        const_assert_eq!(0, hw_cache_id::PMU_HW_CACHE_L1D);
        const_assert_eq!(1, hw_cache_id::PMU_HW_CACHE_L1I);
        const_assert_eq!(2, hw_cache_id::PMU_HW_CACHE_LL);
        const_assert_eq!(3, hw_cache_id::PMU_HW_CACHE_DTLB);
        const_assert_eq!(4, hw_cache_id::PMU_HW_CACHE_ITLB);
        const_assert_eq!(5, hw_cache_id::PMU_HW_CACHE_BPU);
        const_assert_eq!(6, hw_cache_id::PMU_HW_CACHE_NODE);

        const_assert_eq!(0, hw_cache_op_id::PMU_HW_CACHE_OP_READ);
        const_assert_eq!(1, hw_cache_op_id::PMU_HW_CACHE_OP_WRITE);
        const_assert_eq!(2, hw_cache_op_id::PMU_HW_CACHE_OP_PREFETCH);

        const_assert_eq!(0, hw_cache_op_result_id::PMU_HW_CACHE_RESULT_ACCESS);
        const_assert_eq!(1, hw_cache_op_result_id::PMU_HW_CACHE_RESULT_MISS);

        const_assert_eq!(0, fw_event_code::PMU_FW_MISALIGNED_LOAD);
        const_assert_eq!(1, fw_event_code::PMU_FW_MISALIGNED_STORE);
        const_assert_eq!(2, fw_event_code::PMU_FW_ACCESS_LOAD);
        const_assert_eq!(3, fw_event_code::PMU_FW_ACCESS_STORE);
        const_assert_eq!(4, fw_event_code::PMU_FW_ILLEGAL_INSN);
        const_assert_eq!(5, fw_event_code::PMU_FW_SET_TIMER);
        const_assert_eq!(6, fw_event_code::PMU_FW_IPI_SENT);
        const_assert_eq!(7, fw_event_code::PMU_FW_IPI_RECEIVED);
        const_assert_eq!(8, fw_event_code::PMU_FW_FENCE_I_SENT);
        const_assert_eq!(9, fw_event_code::PMU_FW_FENCE_I_RECEIVED);
        const_assert_eq!(10, fw_event_code::PMU_FW_SFENCE_VMA_SENT);
        const_assert_eq!(11, fw_event_code::PMU_FW_SFENCE_VMA_RECEIVED);
        const_assert_eq!(12, fw_event_code::PMU_FW_SFENCE_VMA_ASID_SENT);
        const_assert_eq!(13, fw_event_code::PMU_FW_SFENCE_VMA_ASID_RECEIVED);
        const_assert_eq!(14, fw_event_code::PMU_FW_HFENCE_GVMA_SENT);
        const_assert_eq!(15, fw_event_code::PMU_FW_HFENCE_GVMA_RECEIVED);
        const_assert_eq!(16, fw_event_code::PMU_FW_HFENCE_GVMA_VMID_SENT);
        const_assert_eq!(17, fw_event_code::PMU_FW_HFENCE_GVMA_VMID_RECEIVED);
        const_assert_eq!(18, fw_event_code::PMU_FW_HFENCE_VVMA_SENT);
        const_assert_eq!(19, fw_event_code::PMU_FW_HFENCE_VVMA_RECEIVED);
        const_assert_eq!(20, fw_event_code::PMU_FW_HFENCE_VVMA_ASID_SENT);
        const_assert_eq!(21, fw_event_code::PMU_FW_HFENCE_VVMA_ASID_RECEIVED);
        const_assert_eq!(65535, fw_event_code::PMU_FW_PLATFORM);
    }
    // §12
    #[test]
    fn test_dbcn() {
        use crate::dbcn::*;
        const_assert_eq!(0x4442434E, EID_DBCN);
        const_assert_eq!(0, CONSOLE_WRITE);
        const_assert_eq!(1, CONSOLE_READ);
        const_assert_eq!(2, CONSOLE_WRITE_BYTE);
    }
    // §13
    #[test]
    fn test_susp() {
        use crate::susp::*;
        const_assert_eq!(0x53555350, EID_SUSP);
        const_assert_eq!(0, SUSPEND);
    }
    // §14
    #[test]
    fn test_cppc() {
        use crate::cppc::*;
        const_assert_eq!(0x43505043, EID_CPPC);
        const_assert_eq!(0, PROBE);
        const_assert_eq!(1, READ);
        const_assert_eq!(2, READ_HI);
        const_assert_eq!(3, WRITE);
    }
    // §15
    #[test]
    fn test_nacl() {
        use crate::nacl::*;
        const_assert_eq!(0x4E41434C, EID_NACL);
        const_assert_eq!(0, PROBE_FEATURE);
        const_assert_eq!(1, SET_SHMEM);
        const_assert_eq!(2, SYNC_CSR);
        const_assert_eq!(3, SYNC_HFENCE);
        const_assert_eq!(4, SYNC_SRET);

        const_assert_eq!(0, feature_id::SYNC_CSR);
        const_assert_eq!(1, feature_id::SYNC_HFENCE);
        const_assert_eq!(2, feature_id::SYNC_SRET);
        const_assert_eq!(3, feature_id::AUTOSWAP_CSR);
    }
    #[test]
    fn test_sta() {
        use crate::sta::*;
        const_assert_eq!(0x535441, EID_STA);
        const_assert_eq!(0, SET_SHMEM);
    }
}
