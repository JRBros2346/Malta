pub use crate::winwrap::*;
pub use windows::Win32::System::Environment::*;

pub fn get_command_line() -> PCWSTR {
    unsafe { GetCommandLineW() }
}
