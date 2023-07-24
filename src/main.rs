// #![windows_subsystem = "windows"]

use malta::*;
use std::cell::Cell;

const FILE_MENU_NEW: usize = 110;
const FILE_MENU_EXIT: usize = 140;
const HELP_MENU: usize = 200;

const CHANGE_TITLE: usize = 1000;

#[derive(Debug)]
struct State {
    menu: Cell<HMENU>,
    field: Cell<HWND>,
    edit: Cell<HWND>,
    button: Cell<HWND>,
    width: Cell<i32>,
    height: Cell<i32>,
}
#[allow(non_upper_case_globals)]
const state: State = State::new();
impl State {
    fn add_menus(&self, window: HWND) -> Result<()> {
        self.menu.set(HMENU::create()?);
        let file_menu = HMENU::create()?;
        let sub_menu = HMENU::create()?;

        sub_menu.append(MF_STRING, 0, w!("SubMenu Item"))?;

        file_menu.append(MF_STRING, FILE_MENU_NEW, w!("New"))?;
        file_menu.append(MF_POPUP, sub_menu.0 as usize, w!("Open SubMenu"))?;
        file_menu.append(MF_SEPARATOR, 0, None)?;
        file_menu.append(MF_STRING, FILE_MENU_EXIT, w!("Exit"))?;

        self.menu
            .get()
            .append(MF_POPUP, file_menu.0 as usize, w!("File"))?;
        self.menu.get().append(MF_STRING, HELP_MENU, w!("Help"))?;

        window.set_menu(self.menu.get())?;

        Ok(())
    }

    fn add_controls(&self, window: HWND) -> Result<()> {
        self.field.set(window.create_static(
            WINDOW_EX_STYLE(0),
            w!("Enter Text Here: "),
            WS_VISIBLE | WS_CHILD | WS_BORDER | WINDOW_STYLE(ES_CENTER as u32),
            self.width.get() / 2 - 50,
            100,
            100,
            50,
            None,
            None,
            None,
        )?);

        self.edit.set(window.create_edit(
            WINDOW_EX_STYLE(0),
            w!("..."),
            WS_VISIBLE
                | WS_CHILD
                | WS_BORDER
                | WINDOW_STYLE(ES_MULTILINE as u32)
                | WINDOW_STYLE(ES_AUTOVSCROLL as u32)
                | WINDOW_STYLE(ES_AUTOHSCROLL as u32),
            self.width.get() / 2 - 50,
            152,
            100,
            50,
            None,
            None,
            None,
        )?);

        self.button.set(window.create_button(
            WINDOW_EX_STYLE(0),
            w!("Change Title"),
            WS_VISIBLE | WS_CHILD,
            self.width.get() / 2 - 50,
            204,
            100,
            50,
            HMENU(CHANGE_TITLE as isize),
            None,
            None,
        )?);

        Ok(())
    }

    #[inline]
    const fn new() -> Self {
        State {
            menu: Cell::new(HMENU(0)),
            field: Cell::new(HWND(0)),
            edit: Cell::new(HWND(0)),
            button: Cell::new(HWND(0)),
            width: Cell::new(0),
            height: Cell::new(0),
        }
    }
}

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
        if message.translate() {
            println!("{:#?}", state)
        }
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
                    state.edit.get().get_text(&mut buffer).unwrap_or_else(popup);
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
            let mut rect = RECT::default();
            get_client_rect(window, &mut rect).unwrap_or_else(popup);
            state.width.set(rect.right);
            state.height.set(rect.bottom);

            state.add_menus(window).unwrap_or_else(popup);
            state.add_controls(window).unwrap_or_else(popup);
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
            state.width.set(loword!(l_param.0) as i32);
            state.height.set(hiword!(l_param.0) as i32);
            state
                .field
                .get()
                .set_pos(
                    None,
                    state.width.get() / 2 - 50,
                    100,
                    100,
                    50,
                    SWP_SHOWWINDOW,
                )
                .unwrap_or_else(popup);
            state
                .edit
                .get()
                .set_pos(
                    None,
                    state.width.get() / 2 - 50,
                    152,
                    100,
                    50,
                    SWP_SHOWWINDOW,
                )
                .unwrap_or_else(popup);
            state
                .button
                .get()
                .set_pos(
                    None,
                    state.width.get() / 2 - 50,
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
