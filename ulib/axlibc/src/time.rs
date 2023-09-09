use arceos_posix_api::{sys_clock_gettime, sys_nanosleep};
use core::ffi::c_int;

use crate::{ctypes, utils::e};

/// Get clock time since booting
#[no_mangle]
pub unsafe extern "C" fn clock_gettime(clk: ctypes::clockid_t, ts: *mut ctypes::timespec) -> c_int {
    e(sys_clock_gettime(clk, ts))
}

/// Set clock time since booting
#[no_mangle]
pub unsafe extern "C" fn ax_clock_settime(ts: *mut ctypes::timespec) -> c_int {
    ax_call_body!(ax_clock_settime, {
        if ts.is_null() {
            return Err(LinuxError::EFAULT);
        }
        let now = axhal::time::current_time().into();
        unsafe { *ts = now };
        debug!("ax_clock_settime: {}.{:09}s", now.tv_sec, now.tv_nsec);
        Ok(0)
    })
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
