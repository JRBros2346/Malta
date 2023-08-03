pub use super::*;
pub use windows::Win32::UI::Shell::*;

pub fn command_line_to_argv<P0: IntoParam<PCWSTR>>(cmd_line: P0) -> Result<Vec<String>> {
    let mut num: i32 = 0;
    let first: *mut PWSTR = unsafe { CommandLineToArgvW(cmd_line, &mut num) };
    if num < 0 || first.is_null() {
        return Err(last_error());
    }
    let last: *mut PWSTR = unsafe { first.offset(num.try_into().unwrap()) };

    let mut argv: Vec<String> = Vec::<String>::with_capacity(num.try_into().unwrap());

    let mut cursor: *mut PWSTR = first;
    while cursor != last {
        match unsafe { (*cursor).to_string() } {
            Ok(arg) => argv.push(arg),
            Err(e) => return Err(e.into()),
        }

        cursor = unsafe { cursor.offset(1) };
    }

    Ok(argv)
}
