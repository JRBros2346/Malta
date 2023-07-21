use super::*;
pub use windows::Win32::UI::WindowsAndMessaging::{HMENU, MENU_ITEM_FLAGS};
use windows::Win32::UI::WindowsAndMessaging::{CreateMenu, AppendMenuW};

mod menu_item_flags;
pub use menu_item_flags::*;

pub trait Menu {
    fn new() -> Result<HMENU>;
    fn append<P0: IntoParam<PCWSTR>>(self, flags: MENU_ITEM_FLAGS, uid: usize, name: P0) -> Result<()>;
}

impl Menu for HMENU {
    #[inline]
    fn new() -> Result<HMENU> { unsafe { CreateMenu() } }
    #[inline]
    fn append<P0: IntoParam<PCWSTR>>(self, flags: MENU_ITEM_FLAGS, uid: usize, name: P0) -> Result<()> {
        if unsafe { AppendMenuW(self, flags, uid, name) } == BOOL(0) {
            return Err(last_error());
        }

        Ok(())
    }
}