use crate::error::check_error;
use crate::str::ToWin32Str;
use std::ffi::c_void;
use thiserror::Error;
use windows::core::{HRESULT, PCWSTR};
use windows::Win32::Foundation::{CloseHandle, BOOL, ERROR_ALREADY_EXISTS, HANDLE};
use windows::Win32::Security::{
    GetTokenInformation, TokenStatistics, TOKEN_QUERY, TOKEN_STATISTICS,
};
use windows::Win32::System::Threading::{CreateMutexW, GetCurrentProcess, OpenProcessToken};

/// Use to ensure there is only a single instance of an application running.
/// Note: Dropping the `UniqueInstance` will drop the close the associated handle.
pub struct UniqueInstance {
    mutex: HANDLE,
}

impl UniqueInstance {
    pub fn acquire_unique_to_session(app_name: &str) -> Result<Self, Error> {
        // https://www.codeproject.com/Articles/538/Avoiding-Multiple-Instances-of-an-Application
        unsafe {
            let mut token = HANDLE::default();
            OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token)
                .ok()
                .map_err(Error::OpenProcessToken)?;

            let mut len = 0;
            let mut data = TOKEN_STATISTICS::default();
            let ptr = ((&mut data) as *mut TOKEN_STATISTICS) as *mut c_void;
            GetTokenInformation(
                token,
                TokenStatistics,
                ptr,
                std::mem::size_of_val(&data) as u32,
                &mut len,
            )
            .ok()
            .map_err(Error::GetTokenInformation)?;

            let luid = data.AuthenticationId;
            let mutex_name = format!("{}-{}-{}", app_name, luid.HighPart, luid.LowPart).to_wchar();
            // If the mutex is a named mutex and the object existed before this function call, the return value is a handle to the existing object
            let mutex = check_error(|| {
                CreateMutexW(
                    std::ptr::null_mut(),
                    BOOL::from(true),
                    PCWSTR(mutex_name.as_ptr()),
                )
            })
            .map(|r| r.unwrap())
            .map_err(|e| {
                if e.code() == HRESULT::from(ERROR_ALREADY_EXISTS) {
                    return Error::AlreadyExists;
                }
                Error::CreateMutexW(e)
            })?;
            Ok(UniqueInstance { mutex })
        }
    }
}

impl Drop for UniqueInstance {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.mutex);
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Call to OpenProcessToken() failed: {0}")]
    OpenProcessToken(#[source] windows::core::Error),
    #[error("Call to GetTokenInformation() failed: {0}")]
    GetTokenInformation(#[source] windows::core::Error),
    #[error("Call to CreateMutexW() failed: {0}")]
    CreateMutexW(#[source] windows::core::Error),
    #[error("Another instance already exists")]
    AlreadyExists,
}

impl Error {
    pub fn is_already_exists(&self) -> bool {
        matches!(self, Error::AlreadyExists)
    }
}
