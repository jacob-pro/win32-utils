use crate::error::check_error;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetWindowLongPtrW, GWLP_USERDATA};

pub trait WindowDataExtension {
    /// # Safety
    /// This will cast whatever value is stored in `GetWindowLongPtrW()` to a `&mut T`
    /// Returns None if user data is NULL
    unsafe fn get_user_data<T>(&self) -> Option<&mut T>;
}

impl WindowDataExtension for HWND {
    unsafe fn get_user_data<T>(&self) -> Option<&mut T> {
        let user_data = check_error(|| GetWindowLongPtrW(*self, GWLP_USERDATA)).unwrap() as *mut T;
        if user_data.is_null() {
            return None;
        }
        Some(&mut *(user_data))
    }
}
