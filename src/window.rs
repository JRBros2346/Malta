use super::*;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DestroyWindow, FindWindowExW, GetClientRect, GetWindowTextW, SetMenu,
    SetWindowPos, SetWindowTextW, ShowWindow,
};
pub use windows::Win32::UI::WindowsAndMessaging::{
    HMENU, SET_WINDOW_POS_FLAGS, SHOW_WINDOW_CMD, WINDOW_EX_STYLE, WINDOW_STYLE,
};

pub use windows::Win32::UI::WindowsAndMessaging::{
    HWND_BOTTOM, HWND_DESKTOP, HWND_MESSAGE, HWND_NOTOPMOST, HWND_TOP, HWND_TOPMOST,
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
mod set_window_position_flags;
pub use set_window_position_flags::*;
pub use windows::Win32::UI::WindowsAndMessaging::CW_USEDEFAULT;

pub trait Window {
    fn create<P0, P1, P2, P3, P4>(
        _: WINDOW_EX_STYLE,
        _: P0,
        _: P1,
        _: WINDOW_STYLE,
        _: RECT,
        _: P2,
        _: P3,
        _: P4,
    ) -> Result<HWND>
    where
        P0: IntoParam<PCWSTR>,
        P1: IntoParam<PCWSTR>,
        P2: IntoParam<HWND>,
        P3: IntoParam<HMENU>,
        P4: IntoParam<HMODULE>;
    fn create_static<P0: IntoParam<PCWSTR>>(
        self,
        _: WINDOW_EX_STYLE,
        _: P0,
        _: WINDOW_STYLE,
        _: RECT,
        _: isize,
    ) -> Result<HWND>;
    fn create_edit<P0: IntoParam<PCWSTR>>(
        self,
        _: WINDOW_EX_STYLE,
        _: P0,
        _: WINDOW_STYLE,
        _: RECT,
        _: isize,
    ) -> Result<HWND>;
    fn create_button<P0: IntoParam<PCWSTR>>(
        self,
        _: WINDOW_EX_STYLE,
        _: P0,
        _: WINDOW_STYLE,
        _: RECT,
        _: isize,
    ) -> Result<HWND>;
    fn show(self, _: SHOW_WINDOW_CMD) -> bool;
    fn set_menu<P0: IntoParam<HMENU>>(self, _: P0) -> Result<()>;
    fn destroy(self) -> Result<()>;
    fn get_text(self, _: &mut [u16]) -> Result<i32>;
    fn set_text<P0: IntoParam<PCWSTR>>(self, _: P0) -> Result<()>;
    fn set_pos<P1: IntoParam<HWND>>(self, _: P1, _: RECT, _: SET_WINDOW_POS_FLAGS) -> Result<()>;
    fn find_child<P0, P1, P2>(self, _: P0, _: P1, _: P2) -> Result<HWND>
    where
        P0: IntoParam<HWND>,
        P1: IntoParam<PCWSTR>,
        P2: IntoParam<PCWSTR>;
}

impl Window for HWND {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn create<P0, P1, P2, P3, P4>(
        ex_style: WINDOW_EX_STYLE,
        class_name: P0,
        window_name: P1,
        style: WINDOW_STYLE,
        rect: RECT,
        parent: P2,
        menu: P3,
        instance: P4,
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
                rect.left,
                rect.top,
                rect.right,
                rect.bottom,
                parent,
                menu,
                instance,
                None,
            )
        };
        if wnd == Self(0) {
            return Err(last_error());
        }

        Ok(wnd)
    }
    #[inline]
    fn create_static<P0: IntoParam<PCWSTR>>(
        self,
        ex_style: WINDOW_EX_STYLE,
        window_name: P0,
        style: WINDOW_STYLE,
        rect: RECT,
        id: isize,
    ) -> Result<HWND> {
        Self::create(
            ex_style,
            w!("STATIC"),
            window_name,
            style,
            rect,
            self,
            HMENU(id),
            None,
        )
    }
    #[inline]
    fn create_edit<P0: IntoParam<PCWSTR>>(
        self,
        ex_style: WINDOW_EX_STYLE,
        window_name: P0,
        style: WINDOW_STYLE,
        rect: RECT,
        id: isize,
    ) -> Result<HWND> {
        Self::create(
            ex_style,
            w!("EDIT"),
            window_name,
            style,
            rect,
            self,
            HMENU(id),
            None,
        )
    }
    #[inline]
    fn create_button<P0: IntoParam<PCWSTR>>(
        self,
        ex_style: WINDOW_EX_STYLE,
        window_name: P0,
        style: WINDOW_STYLE,
        rect: RECT,
        id: isize,
    ) -> Result<HWND> {
        Self::create(
            ex_style,
            w!("BUTTON"),
            window_name,
            style,
            rect,
            self,
            HMENU(id),
            None,
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
    #[inline]
    fn get_text(self, buffer: &mut [u16]) -> Result<i32> {
        let len = unsafe { GetWindowTextW(self, buffer) };
        if len == 0 {
            return Err(last_error());
        }

        Ok(len)
    }
    #[inline]
    fn set_text<P0: IntoParam<PCWSTR>>(self, string: P0) -> Result<()> {
        if !unsafe { SetWindowTextW(self, string) }.as_bool() {
            return Err(last_error());
        }

        Ok(())
    }
    #[inline]
    fn set_pos<P1: IntoParam<HWND>>(
        self,
        insert_after: P1,
        rect: RECT,
        flags: SET_WINDOW_POS_FLAGS,
    ) -> Result<()> {
        if !unsafe {
            SetWindowPos(
                self,
                insert_after,
                rect.left,
                rect.top,
                rect.right,
                rect.bottom,
                flags,
            )
        }
        .as_bool()
        {
            return Err(last_error());
        }

        Ok(())
    }
    #[inline]
    fn find_child<P0, P1, P2>(self, child_after: P0, class: P1, window: P2) -> Result<HWND>
    where
        P0: IntoParam<HWND>,
        P1: IntoParam<PCWSTR>,
        P2: IntoParam<PCWSTR>,
    {
        let child = unsafe { FindWindowExW(self, child_after, class, window) };
        if child == HWND(0) {
            return Err(last_error());
        }

        Ok(child)
    }
}

#[inline]
pub fn get_client_rect(window: HWND, rect: &mut RECT) -> Result<()> {
    if !unsafe { GetClientRect(window, rect) }.as_bool() {
        return Err(last_error());
    }

    Ok(())
}
