use std::ffi::{OsStr, OsString};
use std::iter::once;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use thiserror::Error;
use windows::core::PWSTR;

pub trait FromWin32Str {
    fn from_wchar_lossy(wchar: &[u16]) -> Self;
    fn from_pwstr_lossy(pwstr: PWSTR) -> Self;
}

impl FromWin32Str for String {
    fn from_wchar_lossy(wchar: &[u16]) -> Self {
        let end = wchar.iter().position(|&x| x == 0).unwrap_or(wchar.len());
        let truncated = &wchar[0..end];
        OsString::from_wide(truncated).to_string_lossy().into()
    }

    fn from_pwstr_lossy(pwstr: PWSTR) -> Self {
        unsafe {
            let len = (0..).take_while(|&i| *pwstr.0.offset(i) != 0).count();
            let slice = std::slice::from_raw_parts(pwstr.0, len);
            OsString::from_wide(slice).to_string_lossy().into()
        }
    }
}

pub trait ToWin32Str {
    fn to_wchar(&self) -> Vec<u16>;
    fn copy_to_wchar_buffer(&self, buffer: &mut [u16]) -> Result<(), WcharCopyError>;
}

#[derive(Debug, Error)]
#[error("The WCHAR encoded string exceeds the length of the buffer")]
pub struct WcharCopyError;

impl<T: AsRef<str>> ToWin32Str for T {
    fn to_wchar(&self) -> Vec<u16> {
        OsStr::new(self.as_ref())
            .encode_wide()
            .chain(once(0))
            .collect()
    }

    fn copy_to_wchar_buffer(&self, buffer: &mut [u16]) -> Result<(), WcharCopyError> {
        let mut wide = self.to_wchar();
        // It is ok to drop the terminating character
        if wide.len() > buffer.len() + 1 {
            return Err(WcharCopyError {});
        }
        //If buffer.len() is greater than wide.len(), then wide is extended by the
        // difference, with each additional slot filled with 0.
        // If buffer.len() is less than wide.len(), then wide is simply truncated.
        wide.resize(buffer.len(), 0);
        buffer.copy_from_slice(&wide);
        Ok(())
    }
}
