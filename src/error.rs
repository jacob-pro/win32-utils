use windows::core::Error;
use windows::Win32::Foundation::{SetLastError, ERROR_SUCCESS};

#[inline]
/// Use to wrap fallible Win32 functions
/// First calls SetLastError(0)
/// And then after checks GetLastError()
pub fn check_error<F, R>(mut f: F) -> windows::core::Result<R>
where
    F: FnMut() -> R,
{
    unsafe {
        SetLastError(ERROR_SUCCESS);
        let result = f();
        let error = Error::from_win32();
        if error == Error::OK {
            Ok(result)
        } else {
            Err(error)
        }
    }
}