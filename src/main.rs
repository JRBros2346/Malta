use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;

use windows::Win32::System::{
    LibraryLoader::GetModuleHandleW,
    Environment::GetCommandLineW,
    Threading::{GetStartupInfoW, STARTUPINFOW},
    Diagnostics::Debug::MessageBeep,
};
use windows::Win32::UI::WindowsAndMessaging::*;
// use windows::Win32::System::Com::*;

const FILE_MENU_NEW: usize = 11;
const FILE_MENU_EXIT: usize = 13;
const HELP_MENU: usize = 20;

struct State {
    menu: HMENU,
}
impl State {
    fn add_menus(&mut self, wnd: HWND) -> Result<()> {
        self.menu = unsafe { CreateMenu() }?;
        let file_menu: HMENU = unsafe { CreateMenu() }?;
        let sub_menu: HMENU = unsafe { CreateMenu() }?;

        unsafe { AppendMenuW(sub_menu, MF_STRING, 0, w!("SubMenu Item")) };

        unsafe { AppendMenuW(file_menu, MF_STRING, FILE_MENU_NEW, w!("New")) };
        unsafe { AppendMenuW(file_menu, MF_POPUP, sub_menu.0 as usize, w!("Open Submenu")) };
        unsafe { AppendMenuW(file_menu, MF_SEPARATOR, 0, None) };
        unsafe { AppendMenuW(file_menu, MF_STRING, FILE_MENU_EXIT, w!("Exit")) };

        unsafe { AppendMenuW(self.menu, MF_POPUP, file_menu.0 as usize, w!("File")) };
        unsafe { AppendMenuW(self.menu, MF_STRING, HELP_MENU, w!("Help")) };

        unsafe { SetMenu(wnd, self.menu); }
        Ok(())
    }
}
impl Default for State {
    fn default() -> Self {
        State {
            menu: Default::default(),
        }
    }
}

unsafe extern "system" fn window_procedure( wnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let state_ptr: *mut State;
    if msg == WM_CREATE {
        state_ptr = (*(&lparam as *const _ as *const CREATESTRUCTW)).lpCreateParams as *mut State;
        SetWindowLongPtrW(wnd, GWLP_USERDATA, state_ptr as isize);
        if let Some(e) = (*state_ptr).add_menus(wnd).err() {
            eprintln!("Error creating menus: {:?}", e);
        }
    } else {
        state_ptr = unsafe { GetWindowLongPtrW(wnd, GWLP_USERDATA) } as *mut State;
    }
    match msg {
        WM_CLOSE => {
            if MessageBoxW(wnd, w!("Quit?"), w!("Malta"), MB_OKCANCEL) == IDOK {
                unsafe { DestroyWindow(wnd); }
                return LRESULT(0);
            }
            LRESULT(0)
        }
        WM_COMMAND => {
            match wparam.0 {
                HELP_MENU => unsafe { MessageBeep(MB_OK); },
                FILE_MENU_EXIT => unsafe { DestroyWindow(wnd); },
                FILE_MENU_NEW => unsafe { MessageBeep(MB_ICONINFORMATION); },

                _ => ()
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
