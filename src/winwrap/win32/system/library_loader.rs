pub use crate::winwrap::*;
pub use windows::Win32::System::LibraryLoader::*;

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn get_module_handle<P0: IntoParam<PCWSTR>>(
    flags: u32,
    module_name: P0,
    module: &mut HMODULE,
) -> Result<()> {
    unsafe { GetModuleHandleExW(flags, module_name, module) }.ok()
}
