use super::*;
pub use windows::Win32::UI::WindowsAndMessaging::{MSG, MESSAGEBOX_STYLE, MESSAGEBOX_RESULT};
use windows::Win32::UI::WindowsAndMessaging::{GetMessageW, TranslateMessage, DispatchMessageW, MessageBoxW, DestroyWindow, PostQuitMessage};

mod messagebox_styles;
pub use messagebox_styles::*;
mod messagebox_results;
pub use messagebox_results::*;

pub trait Message {
    fn new() -> Self;
    fn get<P0: IntoParam<HWND>>(&mut self, window: P0, filter_min: u32, filter_max: u32) -> Result<bool>;
    fn translate(&self) -> bool;
    fn dispatch(&self) -> LRESULT;
}

impl Message for MSG {
    #[inline]
    fn new() -> Self { Default::default() }
    #[inline]
    fn get<P0: IntoParam<HWND>>(&mut self, window: P0, filter_min: u32, filter_max: u32) -> Result<bool> {
        match unsafe { GetMessageW(self, window, filter_min, filter_max) } {
            BOOL(-1) => Err(last_error()),
            BOOL(0) => Ok(false),
            _ => Ok(true)
        }
    }
    #[inline]
    fn translate(&self) -> bool {
        if unsafe { TranslateMessage(self) } == BOOL(0) {
            false
        } else {
            true
        }
    }
    #[inline]
    fn dispatch(&self) -> LRESULT {
        unsafe { DispatchMessageW(self) }
    }
}

#[inline]
pub fn post_quit_message(exit_code: i32) {
    unsafe { PostQuitMessage(exit_code); }
}

#[inline]
pub fn message_box<P0, P1, P2>(window: P0, text: P1, caption: P2, style: MESSAGEBOX_STYLE) -> Result<MESSAGEBOX_RESULT>
where
    P0: IntoParam<HWND>,
    P1: IntoParam<PCWSTR>,
    P2: IntoParam<PCWSTR>,
{
    let res = unsafe { MessageBoxW(window, text, caption, style) };
    if res == MESSAGEBOX_RESULT(0) {
        return Err(last_error());
    }

    Ok(res)
}

#[inline]
pub fn destroy_window<P0: IntoParam<HWND>>(window: P0) -> Result<()> {
    if unsafe { DestroyWindow(window) } == BOOL(0) {
        return Err(last_error());
    }

    Ok(())
}