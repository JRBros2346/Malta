use super::*;
use windows::Win32::UI::WindowsAndMessaging::LoadCursorW;
pub use windows::Win32::UI::WindowsAndMessaging::HCURSOR;

use windows::Win32::UI::WindowsAndMessaging::{
    IDC_APPSTARTING, IDC_ARROW, IDC_CROSS, IDC_HAND, IDC_HELP, IDC_IBEAM, IDC_ICON, IDC_NO,
    IDC_PERSON, IDC_PIN, IDC_SIZE, IDC_SIZEALL, IDC_SIZENESW, IDC_SIZENS, IDC_SIZENWSE, IDC_SIZEWE,
    IDC_UPARROW, IDC_WAIT,
};

pub trait Cursor: Sized {
    #![allow(non_upper_case_globals)]
    const AppStarting: PCWSTR;
    const Arrow: PCWSTR;
    const Cross: PCWSTR;
    const Hand: PCWSTR;
    const Help: PCWSTR;
    const IBeam: PCWSTR;
    const Icon: PCWSTR;
    const No: PCWSTR;
    const Person: PCWSTR;
    const Pin: PCWSTR;
    const Size: PCWSTR;
    const SizeAll: PCWSTR;
    const SizeNESW: PCWSTR;
    const SizeNS: PCWSTR;
    const SizeNWSE: PCWSTR;
    const SizeWE: PCWSTR;
    const UpArrow: PCWSTR;
    const Wait: PCWSTR;
    fn load<P0, P1>(_: P0, _: P1) -> Result<Self>
    where
        P0: IntoParam<HMODULE>,
        P1: IntoParam<PCWSTR>;
}

impl Cursor for HCURSOR {
    #![allow(non_upper_case_globals)]
    const AppStarting: PCWSTR = IDC_APPSTARTING;
    const Arrow: PCWSTR = IDC_ARROW;
    const Cross: PCWSTR = IDC_CROSS;
    const Hand: PCWSTR = IDC_HAND;
    const Help: PCWSTR = IDC_HELP;
    const IBeam: PCWSTR = IDC_IBEAM;
    const Icon: PCWSTR = IDC_ICON;
    const No: PCWSTR = IDC_NO;
    const Person: PCWSTR = IDC_PERSON;
    const Pin: PCWSTR = IDC_PIN;
    const Size: PCWSTR = IDC_SIZE;
    const SizeAll: PCWSTR = IDC_SIZEALL;
    const SizeNESW: PCWSTR = IDC_SIZENESW;
    const SizeNS: PCWSTR = IDC_SIZENS;
    const SizeNWSE: PCWSTR = IDC_SIZENWSE;
    const SizeWE: PCWSTR = IDC_SIZEWE;
    const UpArrow: PCWSTR = IDC_UPARROW;
    const Wait: PCWSTR = IDC_WAIT;
    #[inline]
    fn load<P0, P1>(instance: P0, cursor_name: P1) -> Result<Self>
    where
        P0: IntoParam<HMODULE>,
        P1: IntoParam<PCWSTR>,
    {
        unsafe { LoadCursorW(instance, cursor_name) }
    }
}
