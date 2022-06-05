use std::ffi::{OsStr, OsString};
use std::iter::once;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
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
}

impl ToWin32Str for String {
    fn to_wchar(&self) -> Vec<u16> {
        self.as_str().to_wchar()
    }
}

impl ToWin32Str for str {
    fn to_wchar(&self) -> Vec<u16> {
        OsStr::new(self).encode_wide().chain(once(0)).collect()
    }
}
