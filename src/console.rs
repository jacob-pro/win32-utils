use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Console::GetConsoleWindow;
use windows::Win32::System::Threading::GetCurrentProcessId;
use windows::Win32::UI::WindowsAndMessaging::{GetWindowThreadProcessId, ShowWindow, SW_HIDE};

/// Hides the Console Window if and only if the Window belongs to the current process
pub fn hide_console_window_if_in_process() {
    unsafe {
        let console = GetConsoleWindow();
        if !HANDLE(console.0).is_invalid() {
            let mut console_pid = 0;
            GetWindowThreadProcessId(console, &mut console_pid);
            if console_pid == GetCurrentProcessId() {
                ShowWindow(console, SW_HIDE);
            }
        }
    }
}
