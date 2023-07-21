#![windows_subsystem = "windows"]

use malta::*;

fn main() -> Result<()> {
    let (instance, cmd_line, cmd_show) = set_entry_point()?;
    println!("{:?}", cmd_line);

    // Register the window class.
    const CLASS_NAME: PCWSTR = w!("malta_window_class");

    let mut window_class = WNDCLASSEXW::default();

    window_class.cbSize = std::mem::size_of::<WNDCLASSEXW>() as u32;
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
        CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT,

        None,       // Parent window
        None,       // Menu
        instance,  // Instance handle
        None,       // Additional application data
    )?;

    window.show(SHOW_WINDOW_CMD(cmd_show as u32));

    // Run the message loop.
    let mut message = MSG::new();
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
                destroy_window(window).unwrap_or_else(popup);
            }
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

            fill_rect(device_context, &paint_struct.rcPaint, HBRUSH(COLOR_WINDOW.0 as isize + 1));

            end_paint(window, &paint_struct);

            LRESULT(0)
        }
        _ => default_window_procedure(window, msg, w_param, l_param)
    }
    
}