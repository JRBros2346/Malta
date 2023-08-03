pub use windows::Win32::Foundation::*;

pub fn get_last_error() -> WIN32_ERROR {
    unsafe { GetLastError() }
}

pub fn set_last_error(error_code: WIN32_ERROR, types: u32) {
    unsafe {
        SetLastErrorEx(error_code, types);
    }
}
