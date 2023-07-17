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

static mut menu: HMENU = HMENU(0);
static mut edit: HWND = HWND(0);

const FILE_MENU_NEW: usize = 110;
const FILE_MENU_EXIT: usize = 130;
const HELP_MENU: usize = 200;
const CHANGE_TITLE: usize = 121;

#[derive(Debug)]
#[repr(C)]
struct State {
    // x: i32,
    // y: i32,
    width: i32,
    height: i32,
}

fn add_menus(wnd: HWND) -> Result<()> {
    unsafe { menu = CreateMenu()?; }
    let file_menu = unsafe { CreateMenu() }?;
    let sub_menu = unsafe { CreateMenu() }?;

    unsafe { AppendMenuW(sub_menu, MF_STRING, CHANGE_TITLE, w!("Change Title")); }

    unsafe { AppendMenuW(file_menu, MF_STRING, FILE_MENU_NEW, w!("New")); }
    unsafe { AppendMenuW(file_menu, MF_POPUP, sub_menu.0 as usize, w!("Open Submenu")); }
    unsafe { AppendMenuW(file_menu, MF_SEPARATOR, 0, None); }
    unsafe { AppendMenuW(file_menu, MF_STRING, FILE_MENU_EXIT, w!("Exit")); }

    unsafe { AppendMenuW(menu, MF_POPUP, file_menu.0 as usize, w!("File")); }
    unsafe { AppendMenuW(menu, MF_STRING, HELP_MENU,  w!("Help")); }

    unsafe { SetMenu(wnd, menu); }
    Ok(())
}
fn add_controls(state: &State, wnd: HWND) {
    unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE(0),
            w!("static"),
            w!("Enter text here: "),
            WS_VISIBLE | WS_CHILD | WS_BORDER | WINDOW_STYLE(ES_CENTER as u32),
            ((state.width as f32) / 2.) as i32 - 50,
            100, 100, 50,
            wnd, None, None, None);
    }
    unsafe { edit = CreateWindowExW(
        WINDOW_EX_STYLE(0),
        w!("edit"),
        w!("..."),
        WS_VISIBLE | WS_CHILD | WS_BORDER | 
        WINDOW_STYLE(ES_MULTILINE as u32) | WINDOW_STYLE(ES_AUTOVSCROLL as u32) | WINDOW_STYLE(ES_AUTOHSCROLL as u32),
        ((state.width as f32) / 2.) as i32 - 50,
        152, 100, 50,
        wnd, None, None, None)
    };
    println!("{:?}", unsafe { edit });
}
impl Default for State {
    fn default() -> Self {
        State {
            // x: Default::default(),
            // y: Default::default(),
            width: Default::default(),
            height: Default::default(),
        }
    }
}

unsafe extern "system" fn window_procedure( wnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let state: *mut State;
    if msg == WM_CREATE {
        state = (*(&lparam as *const _ as *const CREATESTRUCTW)).lpCreateParams as *mut State;
        SetWindowLongPtrW(wnd, GWLP_USERDATA, state as isize);
        if let Some(e) = add_menus(wnd).err() {
            eprintln!("Error creating menus: {:?}", e);
        }
        add_controls(&*state, wnd);
        println!("{:?}", *state);
    } else {
        state = GetWindowLongPtrW(wnd, GWLP_USERDATA) as *mut State;
    }
    match msg {
        WM_CLOSE => {
            if MessageBoxW(wnd, w!("Quit?"), w!("Malta"), MB_OKCANCEL) == IDOK {
                DestroyWindow(wnd);
                return LRESULT(0);
            }
            LRESULT(0)
        }
        WM_COMMAND => {
            match wparam.0 {
                HELP_MENU => { MessageBeep(MB_OK); }
                FILE_MENU_EXIT => { DestroyWindow(wnd); }
                FILE_MENU_NEW => { MessageBeep(MB_ICONINFORMATION); }
                CHANGE_TITLE => {
                    println!("{:?}", *state);
                    let mut text: [u16; 256] = [0; 256];
                    GetWindowTextW(edit, &mut text);
                    println!("Title: {}", String::from_utf16(&text).unwrap_or(String::from("")));
                    SetWindowTextW(wnd, PCWSTR::from_raw(text.as_ptr()));
                }
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
        WM_SIZE => {
            let mut rect: RECT = Default::default();
             GetClientRect(wnd, &mut rect);
            (*state).width = rect.right;
            (*state).height = rect.bottom;
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
    
    let mut state: State = Default::default();

    let wnd: HWND = unsafe{
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
    let mut rect: RECT = Default::default();
    unsafe { GetClientRect(wnd, &mut rect); }
    // state.x = wnd.x;
    // state.y = wnd.y;
    state.width = rect.right;
    state.height = rect.bottom;

    unsafe { ShowWindow(wnd, SHOW_WINDOW_CMD(cmd_show as u32)); }

    // Run the msg loop.
    let mut msg: MSG = Default::default();
    while unsafe { GetMessageW(&mut msg, None, 0, 0) }.as_bool() 
    {
            unsafe { TranslateMessage(&msg); }
            unsafe { DispatchMessageW(&msg); }
    }

    Result::Ok(())
}
