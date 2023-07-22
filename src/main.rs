// #![windows_subsystem = "windows"]

use std::sync::RwLock;
use malta::*;

const FILE_MENU_NEW: usize = 110;
const CHANGE_TITLE: usize = 121;
const FILE_MENU_EXIT: usize = 140;
const HELP_MENU: usize = 200;

struct State {
    menu: HMENU,
    edit: HWND,
    width: i32,
    height: i32,
}
#[allow(non_upper_case_globals)]
static state: RwLock<State> = RwLock::new(State::new());
impl State {
    fn add_menus(&mut self, window: HWND) -> Result<()> {
        self.menu = HMENU::create()?;
        let file_menu = HMENU::create()?;
        let sub_menu = HMENU::create()?;

        sub_menu.append(MF_STRING, CHANGE_TITLE, w!("Change Title"))?;

        file_menu.append(MF_STRING, FILE_MENU_NEW, w!("New"))?;
        file_menu.append(MF_POPUP, sub_menu.0 as usize, w!("Open SubMenu"))?;
        file_menu.append(MF_SEPARATOR, 0, None)?;
        file_menu.append(MF_STRING, FILE_MENU_EXIT, w!("Exit"))?;

        self.menu.append(MF_POPUP, file_menu.0 as usize, w!("File"))?;
        self.menu.append(MF_STRING, HELP_MENU, w!("Help"))?;
    
        window.set_menu(self.menu)?;
    
        Ok(())
    }

    fn add_controls(&mut self, window: HWND) -> Result<()> {
        window.create_static(
            WINDOW_EX_STYLE(0),
            w!("Enter Text Here: "),
            WS_VISIBLE | WS_CHILD | WS_BORDER | WINDOW_STYLE(ES_CENTER as u32),
            self.width / 2 - 50,
            100,
            100,
            50,
            None,
            None,
            None
        )?;

        self.edit = window.create_edit(
            WINDOW_EX_STYLE(0),
            w!("..."),
            WS_VISIBLE | WS_CHILD | WS_BORDER |
            WINDOW_STYLE(ES_MULTILINE as u32) | WINDOW_STYLE(ES_AUTOVSCROLL as u32) | WINDOW_STYLE(ES_AUTOHSCROLL as u32),
            self.width / 2 - 50,
            152,
            100,
            50,
            None,
            None,
            None
        )?;

        Ok(())
    }

    #[inline]
    const fn new() -> Self {
        State {
            menu: HMENU(0),
            edit: HWND(0),
            width: 0,
            height: 0,
        }
    }
}

fn main() -> Result<()> {
    let (instance, cmd_line, cmd_show) = set_entry_point()?;
    println!("{:?}", cmd_line);

    // Register the window class.
    const CLASS_NAME: PCWSTR = w!("malta_window_class");

    let mut window_class = WNDCLASSEXW::default();

    window_class.cbSize = std::mem::size_of::<WNDCLASSEXW>() as u32;
    window_class.hbrBackground = HBRUSH(COLOR_WINDOW.0.try_into().unwrap());
    window_class.hCursor = HCURSOR::load(None, IDC_ARROW)?;
    window_class.lpfnWndProc = Some(window_procedure);
    window_class.hInstance = instance;
    window_class.lpszClassName = CLASS_NAME;

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

        None,       // Parent window
        None,       // Menu
        instance,  // Instance handle
        None,       // Additional application data
    )?;

    window.show(SHOW_WINDOW_CMD(cmd_show as u32));

    // Run the message loop.
    let mut message = MSG::default();
    while message.get(None, 0, 0)?
    {
        message.translate();
        message.dispatch();
    }

    Result::Ok(())
}

extern "system" fn window_procedure(window: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match msg {
        WM_CLOSE => {
            if message_box(window, w!("Quit?"), w!("Malta"), MB_OKCANCEL).unwrap_or_else(popup) == IDOK {
                window
                    .destroy()
                    .unwrap_or_else(popup);
            }
            LRESULT(0)
        }
        WM_COMMAND => {
            match w_param {
                WPARAM(FILE_MENU_EXIT) => window.destroy().unwrap_or_else(popup),
                WPARAM(FILE_MENU_NEW) => message_beep(MB_ICONINFORMATION).unwrap_or_else(popup),
                WPARAM(CHANGE_TITLE) => {
                    let mut buffer: [u16;256] = [0;256];
                    state
                        .read()
                        .unwrap()
                        .edit
                        .get_text(&mut buffer)
                        .unwrap_or_else(popup);
                    window
                        .set_text(PCWSTR(buffer.as_ptr()))
                        .unwrap_or_else(popup);
                }
                WPARAM(HELP_MENU) => message_beep(MB_OK).unwrap_or_else(popup),

                _ => ()
            }

            LRESULT(0)
        }
        WM_CREATE => {
            let mut rect = RECT::default();
            get_client_rect(window, &mut rect).unwrap_or_else(popup);
            let mut state_write = state
                .write()
                .unwrap();
            (*state_write).width = rect.right;
            (*state_write).height = rect.bottom;

            state_write
                .add_menus(window)
                .unwrap_or_else(popup);
            state_write
                .add_controls(window)
                .unwrap_or_else(popup);
            drop(state_write);
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

            fill_rect(device_context, &paint_struct.rcPaint, HBRUSH(<i32 as TryInto<isize>>::try_into(COLOR_WINDOW.0).unwrap() + 1));

            end_paint(window, &paint_struct);

            LRESULT(0)
        }
        _ => default_window_procedure(window, msg, w_param, l_param)
    }
    
}