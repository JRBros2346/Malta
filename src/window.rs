use super::*;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DestroyWindow, GetClientRect, GetWindowTextW, SetMenu, SetWindowTextW,
    ShowWindow,
};
pub use windows::Win32::UI::WindowsAndMessaging::{
    HMENU, SHOW_WINDOW_CMD, WINDOW_EX_STYLE, WINDOW_STYLE,
};

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
        param: Option<*const std::ffi::c_void>,
    ) -> Result<HWND>
    where
        P0: IntoParam<PCWSTR>,
        P1: IntoParam<PCWSTR>,
        P2: IntoParam<HWND>,
        P3: IntoParam<HMENU>,
        P4: IntoParam<HMODULE>;
    fn create_static<P0, P1, P2>(
        self,
        ex_style: WINDOW_EX_STYLE,
        window_name: P0,
        style: WINDOW_STYLE,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        menu: P1,
        instance: P2,
        param: Option<*const std::ffi::c_void>,
    ) -> Result<HWND>
    where
        P0: IntoParam<PCWSTR>,
        P1: IntoParam<HMENU>,
        P2: IntoParam<HMODULE>;
    fn create_edit<P0, P1, P2>(
        self,
        ex_style: WINDOW_EX_STYLE,
        window_name: P0,
        style: WINDOW_STYLE,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        menu: P1,
        instance: P2,
        param: Option<*const std::ffi::c_void>,
    ) -> Result<HWND>
    where
        P0: IntoParam<PCWSTR>,
        P1: IntoParam<HMENU>,
        P2: IntoParam<HMODULE>;
    fn create_button<P0, P1, P2>(
        self,
        ex_style: WINDOW_EX_STYLE,
        window_name: P0,
        style: WINDOW_STYLE,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        menu: P1,
        instance: P2,
        param: Option<*const std::ffi::c_void>,
    ) -> Result<HWND>
    where
        P0: IntoParam<PCWSTR>,
        P1: IntoParam<HMENU>,
        P2: IntoParam<HMODULE>;
    fn show(self, cmd_show: SHOW_WINDOW_CMD) -> bool;
    fn set_menu<P0: IntoParam<HMENU>>(self, menu: P0) -> Result<()>;
    fn destroy(self) -> Result<()>;
    fn get_text(self, buffer: &mut [u16]) -> Result<i32>;
    fn set_text<P0: IntoParam<PCWSTR>>(self, string: P0) -> Result<()>;
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
        param: Option<*const std::ffi::c_void>,
    ) -> Result<Self>
    where
        P0: IntoParam<PCWSTR>,
        P1: IntoParam<PCWSTR>,
        P2: IntoParam<HWND>,
        P3: IntoParam<HMENU>,
        P4: IntoParam<HMODULE>,
    {
        let wnd = unsafe {
            CreateWindowExW(
                ex_style,
                class_name,
                window_name,
                style,
                x,
                y,
                width,
                height,
                parent,
                menu,
                instance,
                param,
            )
        };
        if wnd == Self(0) {
            return Err(last_error());
        }

        Ok(wnd)
    }
    #[inline]
    fn create_static<P0, P1, P2>(
        self,
        ex_style: WINDOW_EX_STYLE,
        window_name: P0,
        style: WINDOW_STYLE,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        menu: P1,
        instance: P2,
        param: Option<*const std::ffi::c_void>,
    ) -> Result<HWND>
    where
        P0: IntoParam<PCWSTR>,
        P1: IntoParam<HMENU>,
        P2: IntoParam<HMODULE>,
    {
        Self::create(
            ex_style,
            w!("STATIC"),
            window_name,
            style,
            x,
            y,
            width,
            height,
            self,
            menu,
            instance,
            param,
        )
    }
    #[inline]
    fn create_edit<P0, P1, P2>(
        self,
        ex_style: WINDOW_EX_STYLE,
        window_name: P0,
        style: WINDOW_STYLE,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        menu: P1,
        instance: P2,
        param: Option<*const std::ffi::c_void>,
    ) -> Result<HWND>
    where
        P0: IntoParam<PCWSTR>,
        P1: IntoParam<HMENU>,
        P2: IntoParam<HMODULE>,
    {
        Self::create(
            ex_style,
            w!("EDIT"),
            window_name,
            style,
            x,
            y,
            width,
            height,
            self,
            menu,
            instance,
            param,
        )
    }
    #[inline]
    fn create_button<P0, P1, P2>(
        self,
        ex_style: WINDOW_EX_STYLE,
        window_name: P0,
        style: WINDOW_STYLE,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        menu: P1,
        instance: P2,
        param: Option<*const std::ffi::c_void>,
    ) -> Result<HWND>
    where
        P0: IntoParam<PCWSTR>,
        P1: IntoParam<HMENU>,
        P2: IntoParam<HMODULE>,
    {
        Self::create(
            ex_style,
            w!("BUTTON"),
            window_name,
            style,
            x,
            y,
            width,
            height,
            self,
            menu,
            instance,
            param,
        )
    }
    #[inline]
    fn show(self, cmd_show: SHOW_WINDOW_CMD) -> bool {
        unsafe { ShowWindow(self, cmd_show) }.as_bool()
    }
    #[inline]
    fn set_menu<P0: IntoParam<HMENU>>(self, menu: P0) -> Result<()> {
        if !unsafe { SetMenu(self, menu) }.as_bool() {
            return Err(last_error());
        }

        Ok(())
    }
    #[inline]
    fn destroy(self) -> Result<()> {
        if !unsafe { DestroyWindow(self) }.as_bool() {
            return Err(last_error());
        }

        Ok(())
    }
    fn get_text(self, buffer: &mut [u16]) -> Result<i32> {
        let len = unsafe { GetWindowTextW(self, buffer) };
        if len == 0 {
            return Err(last_error());
        }

        Ok(len)
    }
    fn set_text<P0: IntoParam<PCWSTR>>(self, string: P0) -> Result<()> {
        if !unsafe { SetWindowTextW(self, string) }.as_bool() {
            return Err(last_error());
        }

        Ok(())
    }
}

#[inline]
pub fn get_client_rect(window: HWND, rect: &mut RECT) -> Result<()> {
    if !unsafe { GetClientRect(window, rect) }.as_bool() {
        return Err(last_error());
    }

    Ok(())
}
