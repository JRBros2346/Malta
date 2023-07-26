use super::*;
use windows::Win32::UI::WindowsAndMessaging::{AppendMenuW, CreateMenu};
pub use windows::Win32::UI::WindowsAndMessaging::{HMENU, MENU_ITEM_FLAGS};

mod menu_item_flags;
pub use menu_item_flags::*;

pub trait Menu {
    fn create() -> Result<HMENU>;
    fn append<P0: IntoParam<PCWSTR>>(
        self,
        _: MENU_ITEM_FLAGS,
        _: usize,
        _: P0,
    ) -> Result<()>;
}

impl Menu for HMENU {
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
        if !unsafe { AppendMenuW(self, flags, uid, name) }.as_bool() {
            return Err(last_error());
        }

        Ok(())
    }
}
