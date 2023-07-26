use super::*;
use windows::Win32::UI::WindowsAndMessaging::{AppendMenuW, CreateMenu};
pub use windows::Win32::UI::WindowsAndMessaging::{HMENU, MENU_ITEM_FLAGS};

use windows::Win32::UI::WindowsAndMessaging::{
    MF_APPEND, MF_BITMAP, MF_BYCOMMAND, MF_BYPOSITION, MF_CHANGE, MF_CHECKED, MF_DEFAULT,
    MF_DELETE, MF_DISABLED, MF_ENABLED, MF_END, MF_GRAYED, MF_HELP, MF_HILITE, MF_INSERT,
    MF_MENUBARBREAK, MF_MENUBREAK, MF_MOUSESELECT, MF_OWNERDRAW, MF_POPUP, MF_REMOVE,
    MF_RIGHTJUSTIFY, MF_SEPARATOR, MF_STRING, MF_SYSMENU, MF_UNCHECKED, MF_UNHILITE,
    MF_USECHECKBITMAPS,
};

pub trait Menu {
    #![allow(non_upper_case_globals)]
    const Append: MENU_ITEM_FLAGS;
    const Bitmap: MENU_ITEM_FLAGS;
    const ByCommand: MENU_ITEM_FLAGS;
    const ByPosition: MENU_ITEM_FLAGS;
    const Change: MENU_ITEM_FLAGS;
    const Checked: MENU_ITEM_FLAGS;
    const Default: MENU_ITEM_FLAGS;
    const Delete: MENU_ITEM_FLAGS;
    const Disabled: MENU_ITEM_FLAGS;
    const Enabled: MENU_ITEM_FLAGS;
    const End: MENU_ITEM_FLAGS;
    const Grayed: MENU_ITEM_FLAGS;
    const Help: MENU_ITEM_FLAGS;
    const Hilite: MENU_ITEM_FLAGS;
    const Insert: MENU_ITEM_FLAGS;
    const MenuBarBreak: MENU_ITEM_FLAGS;
    const MenuBreak: MENU_ITEM_FLAGS;
    const MouseSelct: MENU_ITEM_FLAGS;
    const OwnerDraw: MENU_ITEM_FLAGS;
    const Popup: MENU_ITEM_FLAGS;
    const Remove: MENU_ITEM_FLAGS;
    const RightJustify: MENU_ITEM_FLAGS;
    const Separator: MENU_ITEM_FLAGS;
    const String: MENU_ITEM_FLAGS;
    const SystemMenu: MENU_ITEM_FLAGS;
    const Unchecked: MENU_ITEM_FLAGS;
    const Unhilite: MENU_ITEM_FLAGS;
    const UseCheckBitmaps: MENU_ITEM_FLAGS;
    fn create() -> Result<HMENU>;
    fn append<P0: IntoParam<PCWSTR>>(self, _: MENU_ITEM_FLAGS, _: usize, _: P0) -> Result<()>;
}

impl Menu for HMENU {
    #![allow(non_upper_case_globals)]
    const Append: MENU_ITEM_FLAGS = MF_APPEND;
    const Bitmap: MENU_ITEM_FLAGS = MF_BITMAP;
    const ByCommand: MENU_ITEM_FLAGS = MF_BYCOMMAND;
    const ByPosition: MENU_ITEM_FLAGS = MF_BYPOSITION;
    const Change: MENU_ITEM_FLAGS = MF_CHANGE;
    const Checked: MENU_ITEM_FLAGS = MF_CHECKED;
    const Default: MENU_ITEM_FLAGS = MF_DEFAULT;
    const Delete: MENU_ITEM_FLAGS = MF_DELETE;
    const Disabled: MENU_ITEM_FLAGS = MF_DISABLED;
    const Enabled: MENU_ITEM_FLAGS = MF_ENABLED;
    const End: MENU_ITEM_FLAGS = MF_END;
    const Grayed: MENU_ITEM_FLAGS = MF_GRAYED;
    const Help: MENU_ITEM_FLAGS = MF_HELP;
    const Hilite: MENU_ITEM_FLAGS = MF_HILITE;
    const Insert: MENU_ITEM_FLAGS = MF_INSERT;
    const MenuBarBreak: MENU_ITEM_FLAGS = MF_MENUBARBREAK;
    const MenuBreak: MENU_ITEM_FLAGS = MF_MENUBREAK;
    const MouseSelct: MENU_ITEM_FLAGS = MF_MOUSESELECT;
    const OwnerDraw: MENU_ITEM_FLAGS = MF_OWNERDRAW;
    const Popup: MENU_ITEM_FLAGS = MF_POPUP;
    const Remove: MENU_ITEM_FLAGS = MF_REMOVE;
    const RightJustify: MENU_ITEM_FLAGS = MF_RIGHTJUSTIFY;
    const Separator: MENU_ITEM_FLAGS = MF_SEPARATOR;
    const String: MENU_ITEM_FLAGS = MF_STRING;
    const SystemMenu: MENU_ITEM_FLAGS = MF_SYSMENU;
    const Unchecked: MENU_ITEM_FLAGS = MF_UNCHECKED;
    const Unhilite: MENU_ITEM_FLAGS = MF_UNHILITE;
    const UseCheckBitmaps: MENU_ITEM_FLAGS = MF_USECHECKBITMAPS;
    #[inline]
    fn create() -> Result<HMENU> {
        unsafe { CreateMenu() }
    }
    #[inline]
    fn append<P0: IntoParam<PCWSTR>>(
        self,
        flags: MENU_ITEM_FLAGS,
        uid: usize,
        name: P0,
    ) -> Result<()> {
        unsafe { AppendMenuW(self, flags, uid, name) }.ok()
    }
}
