use std::ffi::{OsStr, OsString};
use std::iter::once;
use std::os::windows::ffi::{OsStrExt, OsStringExt};

pub trait FromWin32Str {
    fn from_wchar_lossy(wchar: &[u16]) -> Self;
}

impl FromWin32Str for String {
    fn from_wchar_lossy(wchar: &[u16]) -> Self {
        let end = wchar.iter().position(|&x| x == 0).unwrap_or(wchar.len());
        let truncated = &wchar[0..end];
        OsString::from_wide(truncated).to_string_lossy().into()
    }
}

pub trait ToWin32Str {
    fn to_wchar(&self) -> Vec<u16>;
}

impl ToWin32Str for String {
    fn to_wchar(&self) -> Vec<u16> {
        OsStr::new(self).encode_wide().chain(once(0)).collect()
    }
}
