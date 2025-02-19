use arceos_posix_api::{sys_clock_gettime, sys_nanosleep,sys_clock_settime};
use core::ffi::c_int;

use crate::{ctypes, utils::e};

/// Get clock time since booting
#[no_mangle]
pub unsafe extern "C" fn clock_gettime(clk: ctypes::clockid_t, ts: *mut ctypes::timespec) -> c_int {
    e(sys_clock_gettime(clk, ts))
}

/// Set clock time since booting
#[no_mangle]
pub unsafe extern "C" fn clock_settime(clk: ctypes::clockid_t, ts: *mut ctypes::timespec) -> c_int {
    e(sys_clock_settime(clk, ts))
}

/// Sleep some nanoseconds
///
/// TODO: should be woken by signals, and set errno
#[no_mangle]
pub unsafe extern "C" fn nanosleep(
    req: *const ctypes::timespec,
    rem: *mut ctypes::timespec,
) -> c_int {
    e(sys_nanosleep(req, rem))
}
