//! Capture 3. Binary Encoding

/// SBI functions return type.
///
/// > SBI functions must return a pair of values in a0 and a1,
/// > with a0 returning an error code.
/// > This is analogous to returning the C structure `SbiRet`.
#[derive(PartialEq, Eq)]
#[repr(C)]
pub struct SbiRet {
    /// Error number
    pub error: usize,
    /// Result value
    pub value: usize,
}

pub const RET_SUCCESS: usize = 0;
pub const RET_ERR_FAILED: usize = -1isize as _;
pub const RET_ERR_NOT_SUPPORTED: usize = -2isize as _;
pub const RET_ERR_INVALID_PARAM: usize = -3isize as _;
pub const RET_ERR_DENIED: usize = -4isize as _;
pub const RET_ERR_INVALID_ADDRESS: usize = -5isize as _;
pub const RET_ERR_ALREADY_AVAILABLE: usize = -6isize as _;
pub const RET_ERR_ALREADY_STARTED: usize = -7isize as _;
pub const RET_ERR_ALREADY_STOPPED: usize = -8isize as _;

impl core::fmt::Debug for SbiRet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.error {
            RET_SUCCESS => self.value.fmt(f),
            RET_ERR_FAILED => write!(f, "<SBI call failed>"),
            RET_ERR_NOT_SUPPORTED => write!(f, "<SBI feature not supported>"),
            RET_ERR_INVALID_PARAM => write!(f, "<SBI invalid parameter>"),
            RET_ERR_DENIED => write!(f, "<SBI denied>"),
            RET_ERR_INVALID_ADDRESS => write!(f, "<SBI invalid address>"),
            RET_ERR_ALREADY_AVAILABLE => write!(f, "<SBI already available>"),
            RET_ERR_ALREADY_STARTED => write!(f, "<SBI already started>"),
            RET_ERR_ALREADY_STOPPED => write!(f, "<SBI already stopped>"),
            unknown => write!(f, "[SBI Unknown error: {unknown:#x}]"),
        }
    }
}

pub enum Error {
    Failed,
    NotSupported,
    InvalidParam,
    Denied,
    InvalidAddress,
    AlreadyAvailable,
    AlreadyStarted,
    AlreadyStopped,
    Customed(isize),
}

impl SbiRet {
    /// Converts to a [`Result`].
    pub const fn into_result(self) -> Result<usize, Error> {
        match self.error {
            RET_SUCCESS => Ok(self.value),
            RET_ERR_FAILED => Err(Error::Failed),
            RET_ERR_NOT_SUPPORTED => Err(Error::NotSupported),
            RET_ERR_INVALID_PARAM => Err(Error::InvalidParam),
            RET_ERR_DENIED => Err(Error::Denied),
            RET_ERR_INVALID_ADDRESS => Err(Error::InvalidAddress),
            RET_ERR_ALREADY_AVAILABLE => Err(Error::AlreadyAvailable),
            RET_ERR_ALREADY_STARTED => Err(Error::AlreadyStarted),
            RET_ERR_ALREADY_STOPPED => Err(Error::AlreadyStopped),
            unknown => Err(Error::Customed(unknown as _)),
        }
    }
}
