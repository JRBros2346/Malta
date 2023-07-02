#![allow(non_snake_case)]

use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;

use windows::Win32::System::{
    LibraryLoader::GetModuleHandleW,
    Environment::GetCommandLineW,
    Threading::{GetStartupInfoW, STARTUPINFOW},
};
use windows::Win32::UI::WindowsAndMessaging::*;

unsafe extern "system" fn window_proc(hwnd: HWND, uMsg: u32, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    match uMsg {
        WM_DESTROY => {
            PostQuitMessage(0);
            return LRESULT(0);
        }
        WM_PAINT => {
            let mut paint_struct: PAINTSTRUCT = Default::default();
            let hdc: HDC = BeginPaint(hwnd, &mut paint_struct);

            // All painting occurs here, between BeginPaint and EndPaint.

            FillRect(hdc, &paint_struct.rcPaint, HBRUSH(COLOR_WINDOW.0 as isize + 1));

            EndPaint(hwnd, &paint_struct);

            return LRESULT(0);
        }
        _ => DefWindowProcW(hwnd, uMsg, wParam, lParam)
    }
    
}

fn main() -> Result<()> {
    let hInstance: HMODULE = unsafe { GetModuleHandleW(None)? };

    let hPrevInstance: Option<HMODULE> = None;

    let pCmdLine: PCWSTR = unsafe{ GetCommandLineW() as PCWSTR };

    let mut wStartupInfo: STARTUPINFOW = Default::default();
    unsafe { GetStartupInfoW(&mut wStartupInfo) };
    let nCmdShow: i32 = wStartupInfo.wShowWindow as i32;

    // Register the window class.
    let class_name: PCWSTR = PCWSTR::from_raw(w!("malta_window_class").as_ptr() as *const u16);

    let mut window_class: WNDCLASSW = Default::default();

    window_class.lpfnWndProc = Some(window_proc);
    window_class.hInstance = hInstance;
    window_class.lpszClassName = class_name;

    unsafe{ RegisterClassW(&window_class); }

    // Create the window.
    
    let hwnd: HWND = unsafe{
        CreateWindowExW (
            WINDOW_EX_STYLE(0),
            class_name,
            PCWSTR::from_raw(w!("Malta").as_ptr() as *const u16),
            WS_OVERLAPPEDWINDOW,

            // Size and position
            CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT,

            None,       // Parent window
            None,       // Menu
            hInstance,  // Instance handle
            None,       // Additional application data
        )
    };

    unsafe { ShowWindow(hwnd, SHOW_WINDOW_CMD(nCmdShow as u32)) };

    // Run the message loop.
    let mut message: MSG = Default::default();
    while unsafe { GetMessageW(&mut message, hwnd, 0, 0) }.0 > BOOL(0).0
    {
        unsafe {
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }

    Result::Ok(())
}
