use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;

use windows::Win32::System::{
    LibraryLoader::GetModuleHandleW,
    Environment::GetCommandLineW,
    Threading::{GetStartupInfoW, STARTUPINFOW},
};
use windows::Win32::UI::WindowsAndMessaging::*;

struct State;
impl Default for State {
    fn default() -> Self {
        State
    }
}

#[inline]
fn get_app_state(hwnd: HWND) -> *const State {
    let ptr: isize = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) };
    let state_ptr: *const State = ptr as *const State;
    return state_ptr;
}

unsafe extern "system" fn window_proc(
        #[allow(non_snake_case)]
        hwnd: HWND, 
        #[allow(non_snake_case)]
        uMsg: u32, 
        #[allow(non_snake_case)]
        wParam: WPARAM, 
        #[allow(non_snake_case)]
        lParam: LPARAM) -> LRESULT {
    let state_ptr: *const State;
    if uMsg == WM_CREATE {
        let create_ptr: *const CREATESTRUCTW = &lParam as *const _ as *const CREATESTRUCTW;
        state_ptr = (*create_ptr).lpCreateParams as *const State;
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, state_ptr as isize);
    } else {
        state_ptr = get_app_state(hwnd);
    }
    match uMsg {
        WM_CLOSE => {
            if MessageBoxW(hwnd, PCWSTR::from_raw(w!("Quit?").as_ptr()), PCWSTR::from_raw(w!("Malta").as_ptr()), MB_OKCANCEL) == IDOK {
                unsafe { DestroyWindow(hwnd); }
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
            let hdc: HDC = BeginPaint(hwnd, &mut paint_struct);

            // All painting occurs here, between BeginPaint and EndPaint.

            FillRect(hdc, &paint_struct.rcPaint, HBRUSH(COLOR_WINDOW.0 as isize + 1));

            EndPaint(hwnd, &paint_struct);

            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, uMsg, wParam, lParam)
    }
}

fn main() -> Result<()> {
    #[allow(non_snake_case)]
    let hInstance: HMODULE = unsafe { GetModuleHandleW(None)? };

    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    let pCmdLine: PCWSTR = unsafe{ GetCommandLineW() };

    #[allow(non_snake_case)]
    let mut wStartupInfo: STARTUPINFOW = Default::default();
    unsafe { GetStartupInfoW(&mut wStartupInfo) };
    #[allow(non_snake_case)]
    let nCmdShow: i32 = wStartupInfo.wShowWindow as i32;

    // Register the window class.
    let class_name: PCWSTR = PCWSTR::from_raw(w!("malta_window_class").as_ptr());

    let mut window_class: WNDCLASSW = Default::default();

    window_class.lpfnWndProc = Some(window_proc);
    window_class.hInstance = hInstance;
    window_class.lpszClassName = class_name;

    unsafe{ RegisterClassW(&window_class); }

    // Create the window.
    
    let state: State = Default::default();

    let hwnd: HWND = unsafe{
        CreateWindowExW (
            WINDOW_EX_STYLE(0),
            class_name,
            PCWSTR::from_raw(w!("Malta").as_ptr()),
            WS_OVERLAPPEDWINDOW,

            // Size and position
            CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT,

            None,       // Parent window
            None,       // Menu
            hInstance,  // Instance handle
            Some(&state as *const _ as *const std::ffi::c_void),       // Additional application data
        )
    };

    unsafe { ShowWindow(hwnd, SHOW_WINDOW_CMD(nCmdShow as u32)) };

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
