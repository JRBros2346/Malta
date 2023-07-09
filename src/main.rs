use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;

use windows::Win32::System::{
    LibraryLoader::GetModuleHandleW,
    Environment::GetCommandLineW,
    Threading::{GetStartupInfoW, STARTUPINFOW},
};
use windows::Win32::UI::WindowsAndMessaging::*;
// use windows::Win32::System::Com::*;

struct State;
impl Default for State {
    fn default() -> Self {
        State
    }
}

unsafe extern "system" fn window_procedure( wnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let state_ptr: *const State;
    if msg == WM_CREATE {
        let create_ptr: *const CREATESTRUCTW = &lparam as *const _ as *const CREATESTRUCTW;
        state_ptr = (*create_ptr).lpCreateParams as *const State;
        SetWindowLongPtrW(wnd, GWLP_USERDATA, state_ptr as isize);
    } else {
        state_ptr = unsafe { GetWindowLongPtrW(wnd, GWLP_USERDATA) } as *const State;
    }
    match msg {
        WM_CLOSE => {
            if MessageBoxW(wnd, w!("Quit?"), w!("Malta"), MB_OKCANCEL) == IDOK {
                unsafe { DestroyWindow(wnd); }
                return LRESULT(0);
            }
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        WM_PAINT => {
            let mut paint_struct: PAINTSTRUCT = Default::default();
            let hdc: HDC = BeginPaint(wnd, &mut paint_struct);

            // All painting occurs here, between BeginPaint and EndPaint.

            FillRect(hdc, &paint_struct.rcPaint, HBRUSH(COLOR_WINDOW.0 as isize + 1));

            EndPaint(wnd, &paint_struct);

            LRESULT(0)
        }
        _ => DefWindowProcW(wnd, msg, wparam, lparam)
    }
}

fn main() -> Result<()> {
    let instance: HMODULE = unsafe { GetModuleHandleW(None) }?;

    let cmd_line: PCWSTR = unsafe{ GetCommandLineW() };

    let mut startup_info: STARTUPINFOW = Default::default();
    unsafe { GetStartupInfoW(&mut startup_info) };
    let cmd_show: i32 = startup_info.wShowWindow as i32;

    // Register the window class.
    let class_name: PCWSTR = w!("malta_window_class");

    let mut window_class: WNDCLASSW = Default::default();

    window_class.lpfnWndProc = Some(window_procedure);
    window_class.hInstance = instance;
    window_class.lpszClassName = class_name;
    window_class.hbrBackground = HBRUSH(COLOR_WINDOW.0 as isize);
    window_class.hCursor = unsafe { LoadCursorW(None, IDC_ARROW) }?;

    unsafe { RegisterClassW(&window_class); }

    // Create the window.
    
    let state: State = Default::default();

    let hwnd: HWND = unsafe{
        CreateWindowExW (
            WINDOW_EX_STYLE(0),
            class_name,
            w!("Malta"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,

            // Size and position
            CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT,

            None,       // Parent window
            None,       // Menu
            instance,  // Instance handle
            Some(&state as *const _ as *const std::ffi::c_void),       // Additional application data
        )
    };

    unsafe { ShowWindow(hwnd, SHOW_WINDOW_CMD(cmd_show as u32)) };

    // Run the msg loop.
    let mut msg: MSG = Default::default();
    while unsafe { GetMessageW(&mut msg, None, 0, 0) }.as_bool() 
    {
        unsafe {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    Result::Ok(())
}
