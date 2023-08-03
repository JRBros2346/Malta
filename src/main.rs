// #![windows_subsystem = "windows"]
#![allow(non_upper_case_globals)]

use malta::*;
use std::sync::Mutex;
use winwrap::win32::graphics::gdi::*;
use winwrap::win32::ui::windows_and_messaging::*;

const FILE_MENU_NEW: usize = 110;
const FILE_MENU_EXIT: usize = 140;
const HELP_MENU: usize = 200;

const BORDER: i32 = 10;

static menu: Mutex<HMENU> = Mutex::new(HMENU(0));
static clients: Mutex<HWND> = Mutex::new(HWND(0));

struct State {
    width: i32,
    height: i32,
}
static state: Mutex<State> = Mutex::new(State {
    width: 0,
    height: 0,
});

fn main() -> Result<()> {
    let (instance, cmd_line, cmd_show) = set_entry_point()?;
    println!("{cmd_line:?}");

    // Register the window class.
    const CLASS_NAME: PCWSTR = w!("malta_window_class");

    let window_class = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>().try_into().unwrap(),
        hbrBackground: HBRUSH(COLOR_WINDOW.0.try_into().unwrap()),
        hCursor: load_cursor(None, IDC_ARROW)?,
        lpfnWndProc: Some(window_procedure),
        hInstance: instance,
        lpszClassName: CLASS_NAME,
        ..WNDCLASSEXW::default()
    };

    register_class(&window_class)?;

    // Create the window.

    let window = create_window(
        WINDOW_EX_STYLE(0),
        CLASS_NAME,
        w!("Malta"),
        WS_OVERLAPPEDWINDOW,
        // Size and position
        RECT {
            left: CW_USEDEFAULT,
            top: CW_USEDEFAULT,
            right: CW_USEDEFAULT,
            bottom: CW_USEDEFAULT,
        },
        None,     // Parent window
        None,     // Menu
        instance, // Instance handle
    )?;

    show_window(window, SHOW_WINDOW_CMD(cmd_show as u32));

    // Run the message loop.
    let mut message = MSG::default();
    while get_message(&mut message, None, 0, 0)? {
        translate_message(&message);
        dispatch_message(&message);
    }

    Result::Ok(())
}

extern "system" fn window_procedure(
    window: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match msg {
        WM_CLOSE => {
            if message_box(window, w!("Quit?"), w!("Malta"), MB_OKCANCEL).unwrap_or_else(popup)
                == IDOK
            {
                destroy_window(window).unwrap_or_else(popup);
            }
            LRESULT(0)
        }
        WM_COMMAND => {
            match w_param {
                WPARAM(FILE_MENU_EXIT) => destroy_window(window).unwrap_or_else(popup),
                WPARAM(FILE_MENU_NEW) => {
                    win32::system::diagnostics::debug::message_beep(MB_ICONINFORMATION)
                        .unwrap_or_else(popup)
                }
                WPARAM(HELP_MENU) => {
                    win32::system::diagnostics::debug::message_beep(MB_OK).unwrap_or_else(popup)
                }

                _ => (),
            }

            LRESULT(0)
        }
        WM_CREATE => {
            add_controls(window).unwrap_or_else(popup);
            add_menus(window).unwrap_or_else(popup);

            let mut rect = RECT::default();
            get_client_rect(window, &mut rect).unwrap_or_else(popup);
            state.lock().expect("Can't Acquire Lock..!").width = rect.right;
            state.lock().expect("Can't Acquire Lock..!").height = rect.bottom;

            LRESULT(0)
        }
        WM_DESTROY => {
            post_quit_message(0);
            LRESULT(0)
        }
        WM_PAINT => {
            let mut paint_struct = PAINTSTRUCT::default();
            let device_context = begin_paint(window, &mut paint_struct);

            // All painting occurs here, between BeginPaint and EndPaint.

            fill_rect(
                device_context,
                &paint_struct.rcPaint,
                HBRUSH((COLOR_WINDOW.0 + 1).try_into().unwrap()),
            );

            end_paint(window, &paint_struct);

            LRESULT(0)
        }
        WM_SIZE => LRESULT(0),
        _ => default_window_procedure(window, msg, w_param, l_param),
    }
}

fn add_menus(window: HWND) -> Result<()> {
    *menu.lock().expect("Can't Acquire Lock..!") = create_menu()?;
    let file_menu = create_menu()?;
    let sub_menu = create_menu()?;

    append_menu(sub_menu, MF_STRING, 0, w!("SubMenu Item"))?;

    append_menu(file_menu, MF_STRING, FILE_MENU_NEW, w!("New"))?;
    append_menu(file_menu, MF_POPUP, sub_menu.0 as usize, w!("Open SubMenu"))?;
    append_menu(file_menu, MF_SEPARATOR, 0, None)?;
    append_menu(file_menu, MF_STRING, FILE_MENU_EXIT, w!("Exit"))?;

    append_menu(
        *menu.lock().expect("Can't Acquire Lock..!"),
        MF_POPUP,
        file_menu.0 as usize,
        w!("File"),
    )?;

    append_menu(
        *menu.lock().expect("Can't Acquire Lock..!"),
        MF_STRING,
        HELP_MENU,
        w!("Help"),
    )?;

    set_menu(window, *menu.lock().expect("Can't Acquire Lock..!"))?;

    Ok(())
}

fn add_controls(window: HWND) -> Result<()> {
    *clients.lock().expect("Can't Acquire Lock..!") = create_combo_box_window(
        WINDOW_EX_STYLE(0),
        w!("Enter Text Here: "),
        WS_VISIBLE | WS_CHILD | WS_BORDER | WINDOW_STYLE(CBS_DROPDOWN as u32),
        RECT {
            left: BORDER,
            top: BORDER,
            right: 100,
            bottom: 50,
        },
        window,
        0,
    )?;

    Ok(())
}
