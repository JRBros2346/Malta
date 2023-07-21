use super::*;
pub use windows::Win32::UI::WindowsAndMessaging::WNDPROC;
use windows::Win32::UI::WindowsAndMessaging::DefWindowProcW;

mod window_messages;
pub use window_messages::*;

#[inline]
pub fn default_window_procedure<P0, P1, P2>(window: P0, msg: u32, w_param: P1, l_param: P2) -> LRESULT
where
    P0: IntoParam<HWND>,
    P1: IntoParam<WPARAM>,
    P2: IntoParam<LPARAM>,
{ unsafe { DefWindowProcW(window, msg, w_param, l_param) } }