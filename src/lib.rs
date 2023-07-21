pub use windows::core::*;
pub use windows::Win32::Foundation::*;

mod wndclass;
pub use wndclass::*;
mod wnd;
pub use wnd::*;
mod msg;
pub use msg::*;
mod wndproc;
pub use wndproc::*;
mod graphics;
pub use graphics::*;

use windows::Win32::System::{
    LibraryLoader::GetModuleHandleExW,
    Environment::GetCommandLineW,
    Threading::{GetStartupInfoW, STARTUPINFOW},
};
use windows::Win32::UI::Shell::CommandLineToArgvW;

#[inline]
pub fn last_error() -> Error {
    let error = unsafe { GetLastError() };
    unsafe { SetLastError(NO_ERROR) };
    error.into()
}

#[inline]
pub fn set_entry_point() -> Result<(HMODULE, Vec<String>, i32)> {
    let mut instance = HMODULE::default();
    if unsafe { GetModuleHandleExW(0, None, &mut instance) } == BOOL(0) {
        return Err(last_error());
    }

    let cmd_line = unsafe { GetCommandLineW() };
    let mut n = 0;
    let first = unsafe { CommandLineToArgvW(cmd_line, &mut n) };
    if first.is_null() {
        return Err(last_error());
    }
    let last = unsafe { first.offset(n.try_into().unwrap()) };
    let mut cursor = first;
    let mut cmd_line = Vec::<String>::with_capacity(n.try_into().expect("Arguments List Maybe Negative!..."));
    while cursor != last {
        match unsafe { (*cursor).to_string() } {
            Ok(arg) => cmd_line.push(arg),
            Err(e) => return Err(e.into()),
        }

        cursor = unsafe { cursor.offset(1) };
    }

    let mut startup_info: STARTUPINFOW = Default::default();
    unsafe { GetStartupInfoW(&mut startup_info); }
    let cmd_show: i32 = startup_info.wShowWindow as i32;

    Ok((instance, cmd_line, cmd_show))
}

pub fn popup<T: Default>(e: Error) -> T {
    message_box(None, PCWSTR(e.message().as_ptr()), None, MB_OK).unwrap_or_else(popup);

    Default::default()
}