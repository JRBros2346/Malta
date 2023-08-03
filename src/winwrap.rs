pub mod core;
pub mod win32;
pub use {self::core::*, win32::foundation::*};

#[inline]
pub fn last_error() -> Error {
    let error = get_last_error();
    set_last_error(NO_ERROR, 0);
    error.into()
}

#[inline]
pub fn set_entry_point() -> Result<(HMODULE, Vec<String>, i32)> {
    #[cfg(target_pointer_width = "16")]
    panic!("Target is 16-bit Architecture..!");

    let mut instance = HMODULE::default();
    win32::system::library_loader::get_module_handle(0, None, &mut instance)?;

    let argv =
        win32::ui::shell::command_line_to_argv(win32::system::environment::get_command_line())?;

    let mut startup_info = win32::system::threading::STARTUPINFOW::default();
    win32::system::threading::get_startup_info(&mut startup_info);
    let cmd_show: i32 = startup_info.wShowWindow.into();

    Ok((instance, argv, cmd_show))
}

#[inline]
pub fn popup<T: Default>(e: Error) -> T {
    win32::ui::windows_and_messaging::message_box(
        None,
        PCWSTR(e.message().as_ptr()),
        None,
        win32::ui::windows_and_messaging::MB_OK,
    )
    .unwrap_or_else(popup);

    T::default()
}

#[macro_export]
macro_rules! loword {
    ($x:expr) => {
        ($x & 0xffff) as u32
    };
}

#[macro_export]
macro_rules! hiword {
    ($x:expr) => {
        (($x >> 16) & 0xffff) as u32
    };
}

pub use {loword, hiword};