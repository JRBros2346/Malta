pub use windows::Win32::System::Threading::*;

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn get_startup_info(startup_info: &mut STARTUPINFOW) {
    unsafe { GetStartupInfoW(startup_info) }
}
