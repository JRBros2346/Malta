// #![windows_subsystem = "windows"]
#![allow(non_upper_case_globals)]

use malta::*;
use std::sync::Mutex;

const FILE_MENU_NEW: usize = 110;
const FILE_MENU_EXIT: usize = 140;
const HELP_MENU: usize = 200;

const CHANGE_TITLE: usize = 1000;

static menu: Mutex<winwrap::win32::ui::windows_and_messaging::HMENU> =
    Mutex::new(winwrap::win32::ui::windows_and_messaging::HMENU(0));
static field: Mutex<winwrap::HWND> = Mutex::new(winwrap::HWND(0));
static edit: Mutex<winwrap::HWND> = Mutex::new(winwrap::HWND(0));
static button: Mutex<winwrap::HWND> = Mutex::new(winwrap::HWND(0));

struct State {
    width: i32,
    height: i32,
}
static state: Mutex<State> = Mutex::new(State {
    width: 0,
    height: 0,
});

fn main() -> winwrap::Result<()> {
    let (instance, cmd_line, cmd_show) = winwrap::set_entry_point()?;
    println!("{cmd_line:?}");

    // Register the window class.
    const CLASS_NAME: winwrap::PCWSTR = winwrap::w!("malta_window_class");

    let window_class = winwrap::win32::ui::windows_and_messaging::WNDCLASSEXW {
        cbSize: std::mem::size_of::<winwrap::win32::ui::windows_and_messaging::WNDCLASSEXW>()
            .try_into()
            .unwrap(),
        hbrBackground: winwrap::win32::graphics::gdi::HBRUSH(
            winwrap::win32::graphics::gdi::COLOR_WINDOW
                .0
                .try_into()
                .unwrap(),
        ),
        hCursor: winwrap::win32::ui::windows_and_messaging::load_cursor(
            None,
            winwrap::win32::ui::windows_and_messaging::IDC_ARROW,
        )?,
        lpfnWndProc: Some(window_procedure),
        hInstance: instance,
        lpszClassName: CLASS_NAME,
        ..winwrap::win32::ui::windows_and_messaging::WNDCLASSEXW::default()
    };

    winwrap::win32::ui::windows_and_messaging::register_class(&window_class)?;

    // Create the window.

    let window = winwrap::win32::ui::windows_and_messaging::create_window(
        winwrap::win32::ui::windows_and_messaging::WINDOW_EX_STYLE(0),
        CLASS_NAME,
        winwrap::w!("Malta"),
        winwrap::win32::ui::windows_and_messaging::WS_OVERLAPPEDWINDOW,
        // Size and position
        winwrap::RECT {
            left: winwrap::win32::ui::windows_and_messaging::CW_USEDEFAULT,
            top: winwrap::win32::ui::windows_and_messaging::CW_USEDEFAULT,
            right: winwrap::win32::ui::windows_and_messaging::CW_USEDEFAULT,
            bottom: winwrap::win32::ui::windows_and_messaging::CW_USEDEFAULT,
        },
        None,     // Parent window
        None,     // Menu
        instance, // Instance handle
    )?;

    winwrap::win32::ui::windows_and_messaging::show_window(
        window,
        winwrap::win32::ui::windows_and_messaging::SHOW_WINDOW_CMD(cmd_show as u32),
    );

    // Run the message loop.
    let mut message = winwrap::win32::ui::windows_and_messaging::MSG::default();
    while winwrap::win32::ui::windows_and_messaging::get_message(&mut message, None, 0, 0)? {
        winwrap::win32::ui::windows_and_messaging::translate_message(&message);
        winwrap::win32::ui::windows_and_messaging::dispatch_message(&message);
    }

    winwrap::Result::Ok(())
}

extern "system" fn window_procedure(
    window: winwrap::HWND,
    msg: u32,
    w_param: winwrap::WPARAM,
    l_param: winwrap::LPARAM,
) -> winwrap::LRESULT {
    match msg {
        winwrap::win32::ui::windows_and_messaging::WM_CLOSE => {
            if winwrap::win32::ui::windows_and_messaging::message_box(
                window,
                winwrap::w!("Quit?"),
                winwrap::w!("Malta"),
                winwrap::win32::ui::windows_and_messaging::MB_OKCANCEL,
            )
            .unwrap_or_else(winwrap::popup)
                == winwrap::win32::ui::windows_and_messaging::IDOK
            {
                winwrap::win32::ui::windows_and_messaging::destroy_window(window)
                    .unwrap_or_else(winwrap::popup);
            }
            winwrap::LRESULT(0)
        }
        winwrap::win32::ui::windows_and_messaging::WM_COMMAND => {
            match w_param {
                winwrap::WPARAM(FILE_MENU_EXIT) => {
                    winwrap::win32::ui::windows_and_messaging::destroy_window(window)
                        .unwrap_or_else(winwrap::popup)
                }
                winwrap::WPARAM(FILE_MENU_NEW) => {
                    winwrap::win32::system::diagnostics::debug::message_beep(
                        winwrap::win32::ui::windows_and_messaging::MB_ICONINFORMATION,
                    )
                    .unwrap_or_else(winwrap::popup)
                }
                winwrap::WPARAM(CHANGE_TITLE) => {
                    let mut buffer = [0u16; 128];
                    winwrap::win32::ui::windows_and_messaging::get_window_text(
                        *edit.lock().expect("Can't Acquire Lock..!"),
                        &mut buffer,
                    )
                    .unwrap_or_else(winwrap::popup);
                    winwrap::win32::ui::windows_and_messaging::set_window_text(
                        window,
                        winwrap::PCWSTR(buffer.as_ptr()),
                    )
                    .unwrap_or_else(winwrap::popup);
                }
                winwrap::WPARAM(HELP_MENU) => {
                    winwrap::win32::system::diagnostics::debug::message_beep(
                        winwrap::win32::ui::windows_and_messaging::MB_OK,
                    )
                    .unwrap_or_else(winwrap::popup)
                }

                _ => (),
            }

            winwrap::LRESULT(0)
        }
        winwrap::win32::ui::windows_and_messaging::WM_CREATE => {
            add_controls(window).unwrap_or_else(winwrap::popup);
            add_menus(window).unwrap_or_else(winwrap::popup);

            let mut rect = winwrap::RECT::default();
            winwrap::win32::ui::windows_and_messaging::get_client_rect(window, &mut rect)
                .unwrap_or_else(winwrap::popup);
            state.lock().expect("Can't Acquire Lock..!").width = rect.right;
            state.lock().expect("Can't Acquire Lock..!").height = rect.bottom;

            winwrap::LRESULT(0)
        }
        winwrap::win32::ui::windows_and_messaging::WM_DESTROY => {
            winwrap::win32::ui::windows_and_messaging::post_quit_message(0);
            winwrap::LRESULT(0)
        }
        winwrap::win32::ui::windows_and_messaging::WM_PAINT => {
            let mut paint_struct = winwrap::win32::graphics::gdi::PAINTSTRUCT::default();
            let device_context =
                winwrap::win32::graphics::gdi::begin_paint(window, &mut paint_struct);

            // All painting occurs here, between BeginPaint and EndPaint.

            winwrap::win32::graphics::gdi::fill_rect(
                device_context,
                &paint_struct.rcPaint,
                winwrap::win32::graphics::gdi::HBRUSH(
                    (winwrap::win32::graphics::gdi::COLOR_WINDOW.0 + 1)
                        .try_into()
                        .unwrap(),
                ),
            );

            winwrap::win32::graphics::gdi::end_paint(window, &paint_struct);

            winwrap::LRESULT(0)
        }
        winwrap::win32::ui::windows_and_messaging::WM_SIZE => {
            state.lock().expect("Can't Acquire Lock..!").width = loword!(l_param.0) as i32;
            state.lock().expect("Can't Acquire Lock..!").height = hiword!(l_param.0) as i32;
            winwrap::win32::ui::windows_and_messaging::set_window_pos(
                *field.lock().expect("Can't Acquire Lock..!"),
                None,
                winwrap::RECT {
                    left: state.lock().expect("Can't Acquire Lock..!").width / 2 - 50,
                    top: 100,
                    right: 100,
                    bottom: 50,
                },
                winwrap::win32::ui::windows_and_messaging::SWP_SHOWWINDOW,
            )
            .unwrap_or_else(winwrap::popup);
            winwrap::win32::ui::windows_and_messaging::set_window_pos(
                *edit.lock().expect("Can't Acquire Lock..!"),
                None,
                winwrap::RECT {
                    left: state.lock().expect("Can't Acquire Lock..!").width / 2 - 50,
                    top: 152,
                    right: 100,
                    bottom: 50,
                },
                winwrap::win32::ui::windows_and_messaging::SWP_SHOWWINDOW,
            )
            .unwrap_or_else(winwrap::popup);
            winwrap::win32::ui::windows_and_messaging::set_window_pos(
                *button.lock().expect("Can't Acquire Lock..!"),
                None,
                winwrap::RECT {
                    left: state.lock().expect("Can't Acquire Lock..!").width / 2 - 50,
                    top: 204,
                    right: 100,
                    bottom: 50,
                },
                winwrap::win32::ui::windows_and_messaging::SWP_SHOWWINDOW,
            )
            .unwrap_or_else(winwrap::popup);
            winwrap::LRESULT(0)
        }
        _ => winwrap::win32::ui::windows_and_messaging::default_window_procedure(
            window, msg, w_param, l_param,
        ),
    }
}

fn add_menus(window: winwrap::HWND) -> winwrap::Result<()> {
    *menu.lock().expect("Can't Acquire Lock..!") =
        winwrap::win32::ui::windows_and_messaging::create_menu()?;
    let file_menu = winwrap::win32::ui::windows_and_messaging::create_menu()?;
    let sub_menu = winwrap::win32::ui::windows_and_messaging::create_menu()?;

    winwrap::win32::ui::windows_and_messaging::append_menu(
        sub_menu,
        winwrap::win32::ui::windows_and_messaging::MF_STRING,
        0,
        winwrap::w!("SubMenu Item"),
    )?;

    winwrap::win32::ui::windows_and_messaging::append_menu(
        file_menu,
        winwrap::win32::ui::windows_and_messaging::MF_STRING,
        FILE_MENU_NEW,
        winwrap::w!("New"),
    )?;
    winwrap::win32::ui::windows_and_messaging::append_menu(
        file_menu,
        winwrap::win32::ui::windows_and_messaging::MF_POPUP,
        sub_menu.0 as usize,
        winwrap::w!("Open SubMenu"),
    )?;
    winwrap::win32::ui::windows_and_messaging::append_menu(
        file_menu,
        winwrap::win32::ui::windows_and_messaging::MF_SEPARATOR,
        0,
        None,
    )?;
    winwrap::win32::ui::windows_and_messaging::append_menu(
        file_menu,
        winwrap::win32::ui::windows_and_messaging::MF_STRING,
        FILE_MENU_EXIT,
        winwrap::w!("Exit"),
    )?;

    winwrap::win32::ui::windows_and_messaging::append_menu(
        *menu.lock().expect("Can't Acquire Lock..!"),
        winwrap::win32::ui::windows_and_messaging::MF_POPUP,
        file_menu.0 as usize,
        winwrap::w!("File"),
    )?;

    winwrap::win32::ui::windows_and_messaging::append_menu(
        *menu.lock().expect("Can't Acquire Lock..!"),
        winwrap::win32::ui::windows_and_messaging::MF_STRING,
        HELP_MENU,
        winwrap::w!("Help"),
    )?;

    winwrap::win32::ui::windows_and_messaging::set_menu(
        window,
        *menu.lock().expect("Can't Acquire Lock..!"),
    )?;

    Ok(())
}

fn add_controls(window: winwrap::HWND) -> winwrap::Result<()> {
    *field.lock().expect("Can't Acquire Lock..!") =
        winwrap::win32::ui::windows_and_messaging::create_static_window(
            winwrap::win32::ui::windows_and_messaging::WINDOW_EX_STYLE(0),
            winwrap::w!("Enter Text Here: "),
            winwrap::win32::ui::windows_and_messaging::WS_VISIBLE
                | winwrap::win32::ui::windows_and_messaging::WS_CHILD
                | winwrap::win32::ui::windows_and_messaging::WS_BORDER
                | winwrap::win32::ui::windows_and_messaging::WINDOW_STYLE(
                    winwrap::win32::ui::windows_and_messaging::ES_CENTER as u32,
                ),
            winwrap::RECT {
                left: state.lock().expect("Can't Acquire Lock..!").width / 2 - 50,
                top: 100,
                right: 100,
                bottom: 50,
            },
            window,
            0,
        )?;

    *edit.lock().expect("Can't Acquire Lock..!") =
        winwrap::win32::ui::windows_and_messaging::create_edit_window(
            winwrap::win32::ui::windows_and_messaging::WINDOW_EX_STYLE(0),
            winwrap::w!("..."),
            winwrap::win32::ui::windows_and_messaging::WS_VISIBLE
                | winwrap::win32::ui::windows_and_messaging::WS_CHILD
                | winwrap::win32::ui::windows_and_messaging::WS_BORDER
                | winwrap::win32::ui::windows_and_messaging::WINDOW_STYLE(
                    winwrap::win32::ui::windows_and_messaging::ES_MULTILINE as u32,
                )
                | winwrap::win32::ui::windows_and_messaging::WINDOW_STYLE(
                    winwrap::win32::ui::windows_and_messaging::ES_AUTOVSCROLL as u32,
                )
                | winwrap::win32::ui::windows_and_messaging::WINDOW_STYLE(
                    winwrap::win32::ui::windows_and_messaging::ES_AUTOHSCROLL as u32,
                ),
            winwrap::RECT {
                left: state.lock().expect("Can't Acquire Lock..!").width / 2 - 50,
                top: 152,
                right: 100,
                bottom: 50,
            },
            window,
            0,
        )?;

    *button.lock().expect("Can't Acquire Lock..!") =
        winwrap::win32::ui::windows_and_messaging::create_button_window(
            winwrap::win32::ui::windows_and_messaging::WINDOW_EX_STYLE(0),
            winwrap::w!("Change Title"),
            winwrap::win32::ui::windows_and_messaging::WS_VISIBLE
                | winwrap::win32::ui::windows_and_messaging::WS_CHILD,
            winwrap::RECT {
                left: state.lock().expect("Can't Acquire Lock..!").width / 2 - 50,
                top: 204,
                right: 100,
                bottom: 50,
            },
            window,
            CHANGE_TITLE as isize,
        )?;

    Ok(())
}
