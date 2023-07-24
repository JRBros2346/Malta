use super::*;
use windows::Win32::UI::WindowsAndMessaging::RegisterClassExW;
pub use windows::Win32::UI::WindowsAndMessaging::WNDCLASSEXW;
pub trait WindowClass {
    fn register(&self) -> Result<u16>;
}

impl WindowClass for WNDCLASSEXW {
    #[inline]
    fn register(&self) -> Result<u16> {
        let atom = unsafe { RegisterClassExW(self) };
        if atom == 0 {
            return Err(last_error());
        }

        Ok(atom)
    }
}
