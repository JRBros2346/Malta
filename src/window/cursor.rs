use super::*;
use windows::Win32::UI::WindowsAndMessaging::LoadCursorW;
pub use windows::Win32::UI::WindowsAndMessaging::HCURSOR;

mod cursor_names;
pub use cursor_names::*;

pub trait Cursor {
    fn load<P0, P1>(instance: P0, cursor_name: P1) -> Result<HCURSOR>
    where
        P0: IntoParam<HMODULE>,
        P1: IntoParam<PCWSTR>;
}

impl Cursor for HCURSOR {
    #[inline]
    fn load<P0, P1>(instance: P0, cursor_name: P1) -> Result<HCURSOR>
    where
        P0: IntoParam<HMODULE>,
        P1: IntoParam<PCWSTR>,
    {
        Ok(unsafe { LoadCursorW(instance, cursor_name) }?)
    }
}
