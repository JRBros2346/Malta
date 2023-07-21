use super::*;
pub use windows::Win32::UI::WindowsAndMessaging::{WINDOW_EX_STYLE, WINDOW_STYLE, HMENU, SHOW_WINDOW_CMD};
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, ShowWindow, SetMenu};

mod cursor;
pub use cursor::*;
mod window_class;
pub use window_class::*;
mod window_styles;
pub use window_styles::*;
mod window_extended_styles;
pub use window_extended_styles::*;
mod menu;
pub use menu::*;
pub use windows::Win32::UI::WindowsAndMessaging::CW_USEDEFAULT;

pub trait Window {
    fn create<P0, P1, P2, P3, P4>(
        ex_style: WINDOW_EX_STYLE,
        class_name: P0,
        window_name: P1,
        style: WINDOW_STYLE,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        parent: P2,
        menu: P3,
        instance: P4,
        param: Option<*const std::ffi::c_void>
    ) -> Result<HWND>
    where
        P0: IntoParam<PCWSTR>,
        P1: IntoParam<PCWSTR>,
        P2: IntoParam<HWND>,
        P3: IntoParam<HMENU>,
        P4: IntoParam<HMODULE>;
    fn show(self, cmd_show: SHOW_WINDOW_CMD) -> bool;
    fn set_menu<P0: IntoParam<HMENU>>(self, menu: P0) -> Result<()>;
}

impl Window for HWND {
    #[inline]
    fn create<P0, P1, P2, P3, P4>(
        ex_style: WINDOW_EX_STYLE,
        class_name: P0,
        window_name: P1,
        style: WINDOW_STYLE,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        parent: P2,
        menu: P3,
        instance: P4,
        param: Option<*const std::ffi::c_void>
    ) -> Result<Self>
    where
        P0: IntoParam<PCWSTR>,
        P1: IntoParam<PCWSTR>,
        P2: IntoParam<HWND>,
        P3: IntoParam<HMENU>,
        P4: IntoParam<HMODULE>,
    {
        let wnd = unsafe { CreateWindowExW(ex_style, class_name, window_name, style, x, y, width, height, parent, menu, instance, param) };
        if wnd == Self(0) {
            return Err(last_error());
        }

        Ok(wnd)
    }
    #[inline]
    fn show(self, cmd_show: SHOW_WINDOW_CMD) -> bool {
        if unsafe { ShowWindow(self, cmd_show) } == BOOL(0) {
            false
        } else {
            true
        }
    }
    #[inline]
    fn set_menu<P0: IntoParam<HMENU>>(self, menu: P0) -> Result<()> {
        if unsafe { SetMenu(self, menu) } == BOOL(0) {
            return Err(last_error());
        }

        Ok(())
    }
}