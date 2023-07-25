// #![windows_subsystem = "windows"]
#![allow(non_upper_case_globals)]

use malta::*;
use std::sync::Mutex;

const FILE_MENU_NEW: usize = 110;
const FILE_MENU_EXIT: usize = 140;
const HELP_MENU: usize = 200;

const CHANGE_TITLE: usize = 1000;

static menu: Mutex<HMENU> = Mutex::new(HMENU(0));
static field: Mutex<HWND> = Mutex::new(HWND(0));
static edit: Mutex<HWND> = Mutex::new(HWND(0));
static button: Mutex<HWND> = Mutex::new(HWND(0));

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
    println!("{:?}", cmd_line);

    // Register the window class.
    const CLASS_NAME: PCWSTR = w!("malta_window_class");

    let window_class = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>()
            .try_into()
            .expect("WNDCLASSEXW is Too Big..!"),
        hbrBackground: HBRUSH(COLOR_WINDOW.0.try_into().unwrap()),
        hCursor: HCURSOR::load(None, IDC_ARROW)?,
        lpfnWndProc: Some(window_procedure),
        hInstance: instance,
        lpszClassName: CLASS_NAME,
        ..WNDCLASSEXW::default()
    };

    window_class.register()?;

    // Create the window.

    let window = HWND::create(
        WINDOW_EX_STYLE(0),
        CLASS_NAME,
        w!("Malta"),
        WS_OVERLAPPEDWINDOW,
        // Size and position
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        None,     // Parent window
        None,     // Menu
        instance, // Instance handle
        None,     // Additional application data
    )?;

    window.show(SHOW_WINDOW_CMD(cmd_show as u32));

    // Run the message loop.
    let mut message = MSG::default();
    while message.get(None, 0, 0)? {
        message.translate();
        message.dispatch();
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
                window.destroy().unwrap_or_else(popup);
            }
            LRESULT(0)
        }
        WM_COMMAND => {
            match w_param {
                WPARAM(FILE_MENU_EXIT) => window.destroy().unwrap_or_else(popup),
                WPARAM(FILE_MENU_NEW) => message_beep(MB_ICONINFORMATION).unwrap_or_else(popup),
                WPARAM(CHANGE_TITLE) => {
                    let mut buffer = [0u16; 128];
                    edit.lock()
                        .expect("Can't Acquire Lock..!")
                        .get_text(&mut buffer)
                        .unwrap_or_else(popup);
                    window
                        .set_text(PCWSTR(buffer.as_ptr()))
                        .unwrap_or_else(popup);
                }
                WPARAM(HELP_MENU) => message_beep(MB_OK).unwrap_or_else(popup),

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
        WM_SIZE => {
            state.lock().expect("Can't Acquire Lock..!").width = loword!(l_param.0) as i32;
            state.lock().expect("Can't Acquire Lock..!").height = hiword!(l_param.0) as i32;
            field
                .lock()
                .expect("Can't Acquire Lock..!")
                .set_pos(
                    None,
                    state.lock().expect("Can't Acquire Lock..!").width / 2 - 50,
                    100,
                    100,
                    50,
                    SWP_SHOWWINDOW,
                )
                .unwrap_or_else(popup);
            edit.lock()
                .expect("Can't Acquire Lock..!")
                .set_pos(
                    None,
                    state.lock().expect("Can't Acquire Lock..!").width / 2 - 50,
                    152,
                    100,
                    50,
                    SWP_SHOWWINDOW,
                )
                .unwrap_or_else(popup);
            button
                .lock()
                .expect("Can't Acquire Lock..!")
                .set_pos(
                    None,
                    state.lock().expect("Can't Acquire Lock..!").width / 2 - 50,
                    204,
                    100,
                    50,
                    SWP_SHOWWINDOW,
                )
                .unwrap_or_else(popup);
            LRESULT(0)
        }
        _ => default_window_procedure(window, msg, w_param, l_param),
    }
}

fn add_menus(window: HWND) -> Result<()> {
    *menu.lock().expect("Can't Acquire Lock..!") = HMENU::create()?;
    let file_menu = HMENU::create()?;
    let sub_menu = HMENU::create()?;

    sub_menu.append(MF_STRING, 0, w!("SubMenu Item"))?;

    file_menu.append(MF_STRING, FILE_MENU_NEW, w!("New"))?;
    file_menu.append(MF_POPUP, sub_menu.0 as usize, w!("Open SubMenu"))?;
    file_menu.append(MF_SEPARATOR, 0, None)?;
    file_menu.append(MF_STRING, FILE_MENU_EXIT, w!("Exit"))?;

    menu.lock()
        .expect("Can't Acquire Lock..!")
        .append(MF_POPUP, file_menu.0 as usize, w!("File"))?;
    menu.lock()
        .expect("Can't Acquire Lock..!")
        .append(MF_STRING, HELP_MENU, w!("Help"))?;

    window.set_menu(*menu.lock().expect("Can't Acquire Lock..!"))?;

    Ok(())
}

fn add_controls(window: HWND) -> Result<()> {
    *field.lock().expect("Can't Acquire Lock..!") = window.create_static(
        WINDOW_EX_STYLE(0),
        w!("Enter Text Here: "),
        WS_VISIBLE | WS_CHILD | WS_BORDER | WINDOW_STYLE(ES_CENTER as u32),
        state.lock().expect("Can't Acquire Lock..!").width / 2 - 50,
        100,
        100,
        50,
        None,
        None,
        None,
    )?;

    *edit.lock().expect("Can't Acquire Lock..!") = window.create_edit(
        WINDOW_EX_STYLE(0),
        w!("..."),
        WS_VISIBLE
            | WS_CHILD
            | WS_BORDER
            | WINDOW_STYLE(ES_MULTILINE as u32)
            | WINDOW_STYLE(ES_AUTOVSCROLL as u32)
            | WINDOW_STYLE(ES_AUTOHSCROLL as u32),
        state.lock().expect("Can't Acquire Lock..!").width / 2 - 50,
        152,
        100,
        50,
        None,
        None,
        None,
    )?;

    *button.lock().expect("Can't Acquire Lock..!") = window.create_button(
        WINDOW_EX_STYLE(0),
        w!("Change Title"),
        WS_VISIBLE | WS_CHILD,
        state.lock().expect("Can't Acquire Lock..!").width / 2 - 50,
        204,
        100,
        50,
        HMENU(CHANGE_TITLE as isize),
        None,
        None,
    )?;

    Ok(())
}
