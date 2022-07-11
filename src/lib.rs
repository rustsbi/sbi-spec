#![no_std]
#![deny(warnings, unsafe_code)]

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
        [a] => i32::from_be_bytes([0, 0, 0, a]),
        [a, b] => i32::from_be_bytes([0, 0, a, b]),
        [a, b, c] => i32::from_be_bytes([0, a, b, c]),
        [a, b, c, d] => i32::from_be_bytes([a, b, c, d]),
        _ => unreachable!(),
    }
}

/// Checks during compilation, and provides an item list for developers.
mod static_test {
    use static_assertions::const_assert_eq as eq;
    // §3
    mod binary {
        use super::eq;
        use crate::binary::*;

        use static_assertions::*;
        assert_eq_align!(SbiRet, usize);
        assert_eq_size!(SbiRet, [usize; 2]);
        assert_fields!(SbiRet: error);
        assert_fields!(SbiRet: value);
        assert_impl_all!(SbiRet: Copy, Clone, PartialEq, Eq, core::fmt::Debug);

        eq!(0, RET_SUCCESS as isize);
        eq!(-1, RET_ERR_FAILED as isize);
        eq!(-2, RET_ERR_NOT_SUPPORTED as isize);
        eq!(-3, RET_ERR_INVALID_PARAM as isize);
        eq!(-4, RET_ERR_DENIED as isize);
        eq!(-5, RET_ERR_INVALID_ADDRESS as isize);
        eq!(-6, RET_ERR_ALREADY_AVAILABLE as isize);
        eq!(-7, RET_ERR_ALREADY_STARTED as isize);
        eq!(-8, RET_ERR_ALREADY_STOPPED as isize);
    }
    // §4
    mod base {
        use super::eq;
        use crate::base::*;
        eq!(0x10, EID_BASE);
        eq!(0, GET_SBI_SPEC_VERSION);
        eq!(1, GET_SBI_IMPL_ID);
        eq!(2, GET_SBI_IMPL_VERSION);
        eq!(3, PROBE_EXTENSION);
        eq!(4, GET_MVENDORID);
        eq!(5, GET_MARCHID);
        eq!(6, GET_MIMPID);
        eq!(0, impl_id::BBL);
        eq!(1, impl_id::OPEN_SBI);
        eq!(2, impl_id::XVISOR);
        eq!(3, impl_id::KVM);
        eq!(4, impl_id::RUST_SBI);
        eq!(5, impl_id::DIOSIX);
        eq!(6, impl_id::COFFER);
    }
    // §5
    mod legacy {
        use super::eq;
        use crate::legacy::*;
        eq!(0, LEGACY_SET_TIMER);
        eq!(1, LEGACY_CONSOLE_PUTCHAR);
        eq!(2, LEGACY_CONSOLE_GETCHAR);
        eq!(3, LEGACY_CLEAR_IPI);
        eq!(4, LEGACY_SEND_IPI);
        eq!(5, LEGACY_REMOTE_FENCE_I);
        eq!(6, LEGACY_REMOTE_SFENCE_VMA);
        eq!(7, LEGACY_REMOTE_SFENCE_VMA_ASID);
        eq!(8, LEGACY_SHUTDOWN);
    }
    // §6
    mod time {
        use super::eq;
        use crate::time::*;
        eq!(0x54494D45, EID_TIME);
        eq!(0, SET_TIMER);
    }
    // §7
    mod spi {
        use super::eq;
        use crate::spi::*;
        eq!(0x735049, EID_SPI);
        eq!(0, SEND_IPI);
    }
    // §8
    mod rfnc {
        use super::eq;
        use crate::rfnc::*;
        eq!(0x52464E43, EID_RFNC);
        eq!(0, REMOTE_FENCE_I);
        eq!(1, REMOTE_SFENCE_VMA);
        eq!(2, REMOTE_SFENCE_VMA_ASID);
        eq!(3, REMOTE_HFENCE_GVMA_VMID);
        eq!(4, REMOTE_HFENCE_GVMA);
        eq!(5, REMOTE_HFENCE_VVMA_ASID);
        eq!(6, REMOTE_HFENCE_VVMA);
    }
    // §9
    mod hsm {
        use super::eq;
        use crate::hsm::*;
        eq!(0x48534D, EID_HSM);
        eq!(0, HART_STATE_STARTED);
        eq!(1, HART_STATE_STOPPED);
        eq!(2, HART_STATE_START_PENDING);
        eq!(3, HART_STATE_STOP_PENDING);
        eq!(4, HART_STATE_SUSPENDED);
        eq!(5, HART_STATE_SUSPEND_PENDING);
        eq!(6, HART_STATE_RESUME_PENDING);
        eq!(0x0000_0000, HART_SUSPEND_TYPE_RETENTIVE);
        eq!(0x8000_0000, HART_SUSPEND_TYPE_NON_RETENTIVE);
        eq!(0, HART_START);
        eq!(1, HART_STOP);
        eq!(2, HART_GET_STATUS);
        eq!(3, HART_SUSPEND);
    }
    // §10
    mod srst {
        use super::eq;
        use crate::srst::*;
        eq!(0x53525354, EID_SRST);
        eq!(0, RESET_TYPE_SHUTDOWN);
        eq!(1, RESET_TYPE_COLD_REBOOT);
        eq!(2, RESET_TYPE_WARM_REBOOT);
        eq!(0, RESET_REASON_NO_REASON);
        eq!(1, RESET_REASON_SYSTEM_FAILURE);
        eq!(0, SYSTEM_RESET);
    }
    // §11
    mod pmu {
        use super::eq;
        use crate::pmu::*;
        eq!(0x504D55, EID_PMU);
        eq!(0, PMU_NUM_COUNTERS);
        eq!(1, PMU_COUNTER_GET_INFO);
        eq!(2, PMU_COUNTER_CONFIG_MATCHING);
        eq!(3, PMU_COUNTER_START);
        eq!(4, PMU_COUNTER_STOP);
        eq!(5, PMU_COUNTER_FW_READ);
    }
}
