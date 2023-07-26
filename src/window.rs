use super::*;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DestroyWindow, FindWindowExW, GetClientRect, GetWindowTextW, SetMenu,
    SetWindowPos, SetWindowTextW, ShowWindow,
};
use windows::Win32::UI::WindowsAndMessaging::{
    ES_AUTOHSCROLL, ES_AUTOVSCROLL, ES_CENTER, ES_LEFT, ES_LOWERCASE, ES_MULTILINE, ES_NOHIDESEL,
    ES_NUMBER, ES_OEMCONVERT, ES_PASSWORD, ES_READONLY, ES_RIGHT, ES_UPPERCASE, ES_WANTRETURN,
    WS_ACTIVECAPTION, WS_BORDER, WS_CAPTION, WS_CHILD, WS_CHILDWINDOW, WS_CLIPCHILDREN,
    WS_CLIPSIBLINGS, WS_DISABLED, WS_DLGFRAME, WS_GROUP, WS_HSCROLL, WS_ICONIC, WS_MAXIMIZE,
    WS_MAXIMIZEBOX, WS_MINIMIZE, WS_MINIMIZEBOX, WS_OVERLAPPED, WS_OVERLAPPEDWINDOW, WS_POPUP,
    WS_POPUPWINDOW, WS_SIZEBOX, WS_SYSMENU, WS_TABSTOP, WS_THICKFRAME, WS_TILED, WS_TILEDWINDOW,
    WS_VISIBLE, WS_VSCROLL,
};
pub use windows::Win32::UI::WindowsAndMessaging::{
    HMENU, SET_WINDOW_POS_FLAGS, SHOW_WINDOW_CMD, WINDOW_EX_STYLE, WINDOW_STYLE,
};
use windows::Win32::UI::WindowsAndMessaging::{
    HWND_BOTTOM, HWND_DESKTOP, HWND_MESSAGE, HWND_NOTOPMOST, HWND_TOP, HWND_TOPMOST,
};
use windows::Win32::UI::WindowsAndMessaging::{
    SWP_ASYNCWINDOWPOS, SWP_DEFERERASE, SWP_DRAWFRAME, SWP_FRAMECHANGED, SWP_HIDEWINDOW,
    SWP_NOACTIVATE, SWP_NOCOPYBITS, SWP_NOMOVE, SWP_NOOWNERZORDER, SWP_NOREDRAW, SWP_NOREPOSITION,
    SWP_NOSENDCHANGING, SWP_NOSIZE, SWP_NOZORDER, SWP_SHOWWINDOW,
};
use windows::Win32::UI::WindowsAndMessaging::{
    WS_EX_ACCEPTFILES, WS_EX_APPWINDOW, WS_EX_CLIENTEDGE, WS_EX_COMPOSITED, WS_EX_CONTEXTHELP,
    WS_EX_CONTROLPARENT, WS_EX_DLGMODALFRAME, WS_EX_LAYERED, WS_EX_LAYOUTRTL, WS_EX_LEFT,
    WS_EX_LEFTSCROLLBAR, WS_EX_LTRREADING, WS_EX_MDICHILD, WS_EX_NOACTIVATE, WS_EX_NOINHERITLAYOUT,
    WS_EX_NOPARENTNOTIFY, WS_EX_NOREDIRECTIONBITMAP, WS_EX_OVERLAPPEDWINDOW, WS_EX_PALETTEWINDOW,
    WS_EX_RIGHT, WS_EX_RIGHTSCROLLBAR, WS_EX_RTLREADING, WS_EX_STATICEDGE, WS_EX_TOOLWINDOW,
    WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_EX_WINDOWEDGE,
};

mod cursor;
pub use cursor::*;
mod window_class;
pub use window_class::*;
mod menu;
pub use menu::*;
pub use windows::Win32::UI::WindowsAndMessaging::CW_USEDEFAULT;

pub trait WindowStyle: Sized {
    #![allow(non_upper_case_globals)]
    const EditAutoHorizontalScroll: Self;
    const EditAutoVerticalScroll: Self;
    const EditCenter: Self;
    const EditLeft: Self;
    const EditLowercase: Self;
    const EditMultiline: Self;
    const EditNoHideSel: Self;
    const EditNumber: Self;
    const EditOEMConvert: Self;
    const EditPassword: Self;
    const EditReadOnly: Self;
    const EditRight: Self;
    const EditUppercase: Self;
    const EditWantReturn: Self;

    const ActiveCaption: Self;
    const Border: Self;
    const Caption: Self;
    const Child: Self;
    const ChildWindow: Self;
    const ClipChildren: Self;
    const ClipSiblings: Self;
    const Disabled: Self;
    const DialogueFrame: Self;
    const Group: Self;
    const HorizontalScroll: Self;
    const Iconic: Self;
    const Maximize: Self;
    const MaximizeBox: Self;
    const Minimize: Self;
    const MinimizeBox: Self;
    const Overlapped: Self;
    const OverlappedWindow: Self;
    const Popup: Self;
    const PopupWindow: Self;
    const SizeBox: Self;
    const SystemMenu: Self;
    const TabStop: Self;
    const ThickFrame: Self;
    const Tiled: Self;
    const TiledWindow: Self;
    const Visible: Self;
    const VerticalScroll: Self;
}
impl WindowStyle for WINDOW_STYLE {
    #![allow(non_upper_case_globals)]
    const EditAutoHorizontalScroll: Self = Self(ES_AUTOHSCROLL as u32);
    const EditAutoVerticalScroll: Self = Self(ES_AUTOVSCROLL as u32);
    const EditCenter: Self = Self(ES_CENTER as u32);
    const EditLeft: Self = Self(ES_LEFT as u32);
    const EditLowercase: Self = Self(ES_LOWERCASE as u32);
    const EditMultiline: Self = Self(ES_MULTILINE as u32);
    const EditNoHideSel: Self = Self(ES_NOHIDESEL as u32);
    const EditNumber: Self = Self(ES_NUMBER as u32);
    const EditOEMConvert: Self = Self(ES_OEMCONVERT as u32);
    const EditPassword: Self = Self(ES_PASSWORD as u32);
    const EditReadOnly: Self = Self(ES_READONLY as u32);
    const EditRight: Self = Self(ES_RIGHT as u32);
    const EditUppercase: Self = Self(ES_UPPERCASE as u32);
    const EditWantReturn: Self = Self(ES_WANTRETURN as u32);

    const ActiveCaption: Self = WS_ACTIVECAPTION;
    const Border: Self = WS_BORDER;
    const Caption: Self = WS_CAPTION;
    const Child: Self = WS_CHILD;
    const ChildWindow: Self = WS_CHILDWINDOW;
    const ClipChildren: Self = WS_CLIPCHILDREN;
    const ClipSiblings: Self = WS_CLIPSIBLINGS;
    const Disabled: Self = WS_DISABLED;
    const DialogueFrame: Self = WS_DLGFRAME;
    const Group: Self = WS_GROUP;
    const HorizontalScroll: Self = WS_HSCROLL;
    const Iconic: Self = WS_ICONIC;
    const Maximize: Self = WS_MAXIMIZE;
    const MaximizeBox: Self = WS_MAXIMIZEBOX;
    const Minimize: Self = WS_MINIMIZE;
    const MinimizeBox: Self = WS_MINIMIZEBOX;
    const Overlapped: Self = WS_OVERLAPPED;
    const OverlappedWindow: Self = WS_OVERLAPPEDWINDOW;
    const Popup: Self = WS_POPUP;
    const PopupWindow: Self = WS_POPUPWINDOW;
    const SizeBox: Self = WS_SIZEBOX;
    const SystemMenu: Self = WS_SYSMENU;
    const TabStop: Self = WS_TABSTOP;
    const ThickFrame: Self = WS_THICKFRAME;
    const Tiled: Self = WS_TILED;
    const TiledWindow: Self = WS_TILEDWINDOW;
    const Visible: Self = WS_VISIBLE;
    const VerticalScroll: Self = WS_VSCROLL;
}

pub trait WindowExtendedStyle: Sized {
    #![allow(non_upper_case_globals)]
    const AcceptFiles: Self;
    const AppWindow: Self;
    const ClientEdge: Self;
    const Composited: Self;
    const ContextHelp: Self;
    const ControlParent: Self;
    const DialogueModalFrame: Self;
    const Layered: Self;
    const LayoutRTL: Self;
    const Left: Self;
    const LeftScrollBar: Self;
    const LTRReading: Self;
    const MDIChild: Self;
    const NoActivate: Self;
    const NoInheritLayout: Self;
    const NoParentNotify: Self;
    const NoRedirectionBitmap: Self;
    const OverappedWindow: Self;
    const PaletteWindow: Self;
    const Right: Self;
    const RightScrollBar: Self;
    const RTLReading: Self;
    const StaticEdge: Self;
    const ToolWindow: Self;
    const TopMost: Self;
    const Transparent: Self;
    const WindowEdge: Self;
}
impl WindowExtendedStyle for WINDOW_EX_STYLE {
    #![allow(non_upper_case_globals)]
    const AcceptFiles: WINDOW_EX_STYLE = WS_EX_ACCEPTFILES;
    const AppWindow: WINDOW_EX_STYLE = WS_EX_APPWINDOW;
    const ClientEdge: WINDOW_EX_STYLE = WS_EX_CLIENTEDGE;
    const Composited: WINDOW_EX_STYLE = WS_EX_COMPOSITED;
    const ContextHelp: WINDOW_EX_STYLE = WS_EX_CONTEXTHELP;
    const ControlParent: WINDOW_EX_STYLE = WS_EX_CONTROLPARENT;
    const DialogueModalFrame: WINDOW_EX_STYLE = WS_EX_DLGMODALFRAME;
    const Layered: WINDOW_EX_STYLE = WS_EX_LAYERED;
    const LayoutRTL: WINDOW_EX_STYLE = WS_EX_LAYOUTRTL;
    const Left: WINDOW_EX_STYLE = WS_EX_LEFT;
    const LeftScrollBar: WINDOW_EX_STYLE = WS_EX_LEFTSCROLLBAR;
    const LTRReading: WINDOW_EX_STYLE = WS_EX_LTRREADING;
    const MDIChild: WINDOW_EX_STYLE = WS_EX_MDICHILD;
    const NoActivate: WINDOW_EX_STYLE = WS_EX_NOACTIVATE;
    const NoInheritLayout: WINDOW_EX_STYLE = WS_EX_NOINHERITLAYOUT;
    const NoParentNotify: WINDOW_EX_STYLE = WS_EX_NOPARENTNOTIFY;
    const NoRedirectionBitmap: WINDOW_EX_STYLE = WS_EX_NOREDIRECTIONBITMAP;
    const OverappedWindow: WINDOW_EX_STYLE = WS_EX_OVERLAPPEDWINDOW;
    const PaletteWindow: WINDOW_EX_STYLE = WS_EX_PALETTEWINDOW;
    const Right: WINDOW_EX_STYLE = WS_EX_RIGHT;
    const RightScrollBar: WINDOW_EX_STYLE = WS_EX_RIGHTSCROLLBAR;
    const RTLReading: WINDOW_EX_STYLE = WS_EX_RTLREADING;
    const StaticEdge: WINDOW_EX_STYLE = WS_EX_STATICEDGE;
    const ToolWindow: WINDOW_EX_STYLE = WS_EX_TOOLWINDOW;
    const TopMost: WINDOW_EX_STYLE = WS_EX_TOPMOST;
    const Transparent: WINDOW_EX_STYLE = WS_EX_TRANSPARENT;
    const WindowEdge: WINDOW_EX_STYLE = WS_EX_WINDOWEDGE;
}

pub trait SetWindowPosition {
    #![allow(non_upper_case_globals)]
    const AsynchronousWindowPosition: SET_WINDOW_POS_FLAGS;
    const DeferErase: SET_WINDOW_POS_FLAGS;
    const DrawFrame: SET_WINDOW_POS_FLAGS;
    const FrameChanged: SET_WINDOW_POS_FLAGS;
    const HideWindow: SET_WINDOW_POS_FLAGS;
    const NoActivate: SET_WINDOW_POS_FLAGS;
    const NoCopyBits: SET_WINDOW_POS_FLAGS;
    const NoMove: SET_WINDOW_POS_FLAGS;
    const NoOwnerZOrder: SET_WINDOW_POS_FLAGS;
    const NoRedraw: SET_WINDOW_POS_FLAGS;
    const NoReposition: SET_WINDOW_POS_FLAGS;
    const NoSendChanging: SET_WINDOW_POS_FLAGS;
    const NoSize: SET_WINDOW_POS_FLAGS;
    const NoZOrder: SET_WINDOW_POS_FLAGS;
    const ShowWindow: SET_WINDOW_POS_FLAGS;
}
impl SetWindowPosition for SET_WINDOW_POS_FLAGS {
    #![allow(non_upper_case_globals)]
    const AsynchronousWindowPosition: SET_WINDOW_POS_FLAGS = SWP_ASYNCWINDOWPOS;
    const DeferErase: SET_WINDOW_POS_FLAGS = SWP_DEFERERASE;
    const DrawFrame: SET_WINDOW_POS_FLAGS = SWP_DRAWFRAME;
    const FrameChanged: SET_WINDOW_POS_FLAGS = SWP_FRAMECHANGED;
    const HideWindow: SET_WINDOW_POS_FLAGS = SWP_HIDEWINDOW;
    const NoActivate: SET_WINDOW_POS_FLAGS = SWP_NOACTIVATE;
    const NoCopyBits: SET_WINDOW_POS_FLAGS = SWP_NOCOPYBITS;
    const NoMove: SET_WINDOW_POS_FLAGS = SWP_NOMOVE;
    const NoOwnerZOrder: SET_WINDOW_POS_FLAGS = SWP_NOOWNERZORDER;
    const NoRedraw: SET_WINDOW_POS_FLAGS = SWP_NOREDRAW;
    const NoReposition: SET_WINDOW_POS_FLAGS = SWP_NOREPOSITION;
    const NoSendChanging: SET_WINDOW_POS_FLAGS = SWP_NOSENDCHANGING;
    const NoSize: SET_WINDOW_POS_FLAGS = SWP_NOSIZE;
    const NoZOrder: SET_WINDOW_POS_FLAGS = SWP_NOZORDER;
    const ShowWindow: SET_WINDOW_POS_FLAGS = SWP_SHOWWINDOW;
}

pub trait Window: Sized {
    #![allow(non_upper_case_globals)]
    const Bottom: Self;
    const Desktop: Self;
    const Message: Self;
    const NoTopMost: Self;
    const Top: Self;
    const TopMost: Self;
    #[allow(clippy::too_many_arguments)]
    fn create<P0, P1, P2, P3, P4>(
        _: WINDOW_EX_STYLE,
        _: P0,
        _: P1,
        _: WINDOW_STYLE,
        _: RECT,
        _: P2,
        _: P3,
        _: P4,
    ) -> Result<Self>
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
    ) -> Result<Self>;
    fn create_edit<P0: IntoParam<PCWSTR>>(
        self,
        _: WINDOW_EX_STYLE,
        _: P0,
        _: WINDOW_STYLE,
        _: RECT,
        _: isize,
    ) -> Result<Self>;
    fn create_button<P0: IntoParam<PCWSTR>>(
        self,
        _: WINDOW_EX_STYLE,
        _: P0,
        _: WINDOW_STYLE,
        _: RECT,
        _: isize,
    ) -> Result<Self>;
    fn show(self, _: SHOW_WINDOW_CMD) -> bool;
    fn set_menu<P0: IntoParam<HMENU>>(self, _: P0) -> Result<()>;
    fn destroy(self) -> Result<()>;
    fn get_text(self, _: &mut [u16]) -> Result<i32>;
    fn set_text<P0: IntoParam<PCWSTR>>(self, _: P0) -> Result<()>;
    fn set_pos<P1: IntoParam<HWND>>(self, _: P1, _: RECT, _: SET_WINDOW_POS_FLAGS) -> Result<()>;
    fn get_client_rect(self, _: &mut RECT) -> Result<()>;
    fn find_child<P0, P1, P2>(self, _: P0, _: P1, _: P2) -> Result<Self>
    where
        P0: IntoParam<HWND>,
        P1: IntoParam<PCWSTR>,
        P2: IntoParam<PCWSTR>;
}

impl Window for HWND {
    #![allow(non_upper_case_globals)]
    const Bottom: Self = HWND_BOTTOM;
    const Desktop: Self = HWND_DESKTOP;
    const Message: Self = HWND_MESSAGE;
    const NoTopMost: Self = HWND_NOTOPMOST;
    const Top: Self = HWND_TOP;
    const TopMost: Self = HWND_TOPMOST;
    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn create<P0, P1, P2, P3, P4>(
        extended_style: WINDOW_EX_STYLE,
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
                extended_style,
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
        extended_style: WINDOW_EX_STYLE,
        window_name: P0,
        style: WINDOW_STYLE,
        rect: RECT,
        id: isize,
    ) -> Result<Self> {
        Self::create(
            extended_style,
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
        extended_style: WINDOW_EX_STYLE,
        window_name: P0,
        style: WINDOW_STYLE,
        rect: RECT,
        id: isize,
    ) -> Result<Self> {
        Self::create(
            extended_style,
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
        extended_style: WINDOW_EX_STYLE,
        window_name: P0,
        style: WINDOW_STYLE,
        rect: RECT,
        id: isize,
    ) -> Result<Self> {
        Self::create(
            extended_style,
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
    fn find_child<P0, P1, P2>(self, child_after: P0, class: P1, window: P2) -> Result<Self>
    where
        P0: IntoParam<HWND>,
        P1: IntoParam<PCWSTR>,
        P2: IntoParam<PCWSTR>,
    {
        let child = unsafe { FindWindowExW(self, child_after, class, window) };
        if child == Self(0) {
            return Err(last_error());
        }

        Ok(child)
    }
    #[inline]
    fn get_client_rect(self, rect: &mut RECT) -> Result<()> {
        if !unsafe { GetClientRect(self, rect) }.as_bool() {
            return Err(last_error());
        }

        Ok(())
    }
}
