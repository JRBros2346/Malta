pub use super::*;
pub use windows::Win32::System::Diagnostics::Debug::*;

#[inline]
pub fn message_beep(
    style: crate::winwrap::win32::ui::windows_and_messaging::MESSAGEBOX_STYLE,
) -> Result<()> {
    unsafe { MessageBeep(style) }.ok()
}
