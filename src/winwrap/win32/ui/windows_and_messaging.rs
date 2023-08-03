pub use crate::winwrap::*;
pub use windows::Win32::UI::WindowsAndMessaging::*;

#[inline]
#[allow(clippy::too_many_arguments)]
pub fn create_window<P0, P1, P2, P3, P4>(
    extended_style: WINDOW_EX_STYLE,
    class_name: P0,
    window_name: P1,
    style: WINDOW_STYLE,
    rect: RECT,
    parent: P2,
    menu: P3,
    instance: P4,
) -> Result<HWND>
where
    P0: IntoParam<PCWSTR>,
    P1: IntoParam<PCWSTR>,
    P2: IntoParam<HWND>,
    P3: IntoParam<HMENU>,
    P4: IntoParam<HMODULE>,
{
    let wnd = unsafe {
        CreateWindowExW(
            extended_style,
            class_name,
            window_name,
            style,
            rect.left,
            rect.top,
            rect.right,
            rect.bottom,
            parent,
            menu,
            instance,
            None,
        )
    };
    if wnd == HWND(0) {
        return Err(crate::winwrap::last_error());
    }

    Ok(wnd)
}
#[inline]
pub fn create_static_window<P0, P1>(
    extended_style: WINDOW_EX_STYLE,
    window_name: P0,
    style: WINDOW_STYLE,
    rect: RECT,
    parent: P1,
    id: isize,
) -> Result<HWND>
where
    P0: IntoParam<PCWSTR>,
    P1: IntoParam<HWND>,
{
    create_window(
        extended_style,
        w!("Static"),
        window_name,
        style,
        rect,
        parent,
        HMENU(id),
        None,
    )
}
#[inline]
pub fn create_edit_window<P0, P1>(
    extended_style: WINDOW_EX_STYLE,
    window_name: P0,
    style: WINDOW_STYLE,
    rect: RECT,
    parent: P1,
    id: isize,
) -> Result<HWND>
where
    P0: IntoParam<PCWSTR>,
    P1: IntoParam<HWND>,
{
    create_window(
        extended_style,
        w!("Edit"),
        window_name,
        style,
        rect,
        parent,
        HMENU(id),
        None,
    )
}
#[inline]
pub fn create_button_window<P0, P1>(
    extended_style: WINDOW_EX_STYLE,
    window_name: P0,
    style: WINDOW_STYLE,
    rect: RECT,
    parent: P1,
    id: isize,
) -> Result<HWND>
where
    P0: IntoParam<PCWSTR>,
    P1: IntoParam<HWND>,
{
    create_window(
        extended_style,
        w!("Button"),
        window_name,
        style,
        rect,
        parent,
        HMENU(id),
        None,
    )
}
#[inline]
pub fn create_combo_box_window<P0, P1>(
    extended_style: WINDOW_EX_STYLE,
    window_name: P0,
    style: WINDOW_STYLE,
    rect: RECT,
    parent: P1,
    id: isize,
) -> Result<HWND>
where
    P0: IntoParam<PCWSTR>,
    P1: IntoParam<HWND>,
{
    create_window(
        extended_style,
        w!("ComboBox"),
        window_name,
        style,
        rect,
        parent,
        HMENU(id),
        None,
    )
}
#[inline]
pub fn show_window<P0: IntoParam<HWND>>(window: P0, cmd_show: SHOW_WINDOW_CMD) -> bool {
    unsafe { ShowWindow(window, cmd_show) }.as_bool()
}
#[inline]
pub fn set_menu<P0, P1>(window: P0, menu: P1) -> Result<()>
where
    P0: IntoParam<HWND>,
    P1: IntoParam<HMENU>,
{
    unsafe { SetMenu(window, menu) }.ok()
}
#[inline]
pub fn destroy_window<P0: IntoParam<HWND>>(window: P0) -> Result<()> {
    unsafe { DestroyWindow(window) }.ok()
}
#[inline]
pub fn get_window_text<P0: IntoParam<HWND>>(window: P0, buffer: &mut [u16]) -> Result<i32> {
    let len = unsafe { GetWindowTextW(window, buffer) };
    if len == 0 {
        return Err(crate::winwrap::last_error());
    }

    Ok(len)
}
#[inline]
pub fn set_window_text<P0, P1>(window: P0, string: P1) -> Result<()>
where
    P0: IntoParam<HWND>,
    P1: IntoParam<PCWSTR>,
{
    unsafe { SetWindowTextW(window, string) }.ok()
}
#[inline]
pub fn set_window_pos<P0, P1>(
    window: P0,
    insert_after: P1,
    rect: RECT,
    flags: SET_WINDOW_POS_FLAGS,
) -> Result<()>
where
    P0: IntoParam<HWND>,
    P1: IntoParam<HWND>,
{
    unsafe {
        SetWindowPos(
            window,
            insert_after,
            rect.left,
            rect.top,
            rect.right,
            rect.bottom,
            flags,
        )
    }
    .ok()
}
#[inline]
pub fn find_window<P0, P1, P2, P3>(
    parent: P0,
    child_after: P1,
    class: P2,
    window: P3,
) -> Result<HWND>
where
    P0: IntoParam<HWND>,
    P1: IntoParam<HWND>,
    P2: IntoParam<PCWSTR>,
    P3: IntoParam<PCWSTR>,
{
    let child = unsafe { FindWindowExW(parent, child_after, class, window) };
    if child == HWND(0) {
        return Err(crate::winwrap::last_error());
    }

    Ok(child)
}
#[inline]
pub fn get_client_rect<P0: IntoParam<HWND>>(window: P0, rect: &mut RECT) -> Result<()> {
    unsafe { GetClientRect(window, rect) }.ok()
}

#[inline]
pub fn get_message<P0: IntoParam<HWND>>(
    message: &mut MSG,
    window: P0,
    min: u32,
    max: u32,
) -> Result<bool> {
    match unsafe { GetMessageW(message, window, min, max) } {
        BOOL(-1) => Err(crate::winwrap::last_error()),
        BOOL(0) => Ok(false),
        _ => Ok(true),
    }
}
#[inline]
pub fn translate_message(message: &MSG) -> bool {
    unsafe { TranslateMessage(message) }.as_bool()
}
#[inline]
pub fn dispatch_message(message: &MSG) -> LRESULT {
    unsafe { DispatchMessageW(message) }
}

#[inline]
pub fn post_quit_message(exit_code: i32) {
    unsafe {
        PostQuitMessage(exit_code);
    }
}

#[inline]
pub fn message_box<P0, P1, P2>(
    window: P0,
    text: P1,
    caption: P2,
    style: MESSAGEBOX_STYLE,
) -> Result<MESSAGEBOX_RESULT>
where
    P0: IntoParam<HWND>,
    P1: IntoParam<PCWSTR>,
    P2: IntoParam<PCWSTR>,
{
    let res = unsafe { MessageBoxW(window, text, caption, style) };
    if res == MESSAGEBOX_RESULT(0) {
        return Err(crate::winwrap::last_error());
    }

    Ok(res)
}

#[inline]
pub fn default_window_procedure<P0, P1, P2>(
    window: P0,
    msg: u32,
    param_wide: P1,
    param_long: P2,
) -> LRESULT
where
    P0: IntoParam<HWND>,
    P1: IntoParam<WPARAM>,
    P2: IntoParam<LPARAM>,
{
    unsafe { DefWindowProcW(window, msg, param_wide, param_long) }
}

#[inline]
pub fn load_cursor<P0, P1>(instance: P0, cursor_name: P1) -> Result<HCURSOR>
where
    P0: IntoParam<HMODULE>,
    P1: IntoParam<PCWSTR>,
{
    unsafe { LoadCursorW(instance, cursor_name) }
}

#[inline]
pub fn create_menu() -> Result<HMENU> {
    unsafe { CreateMenu() }
}
#[inline]
pub fn append_menu<P0, P1>(menu: P0, flags: MENU_ITEM_FLAGS, uid: usize, name: P1) -> Result<()>
where
    P0: IntoParam<HMENU>,
    P1: IntoParam<PCWSTR>,
{
    unsafe { AppendMenuW(menu, flags, uid, name) }.ok()
}

#[inline]
pub fn register_class(window_class: &WNDCLASSEXW) -> Result<u16> {
    let atom = unsafe { RegisterClassExW(window_class) };
    if atom == 0 {
        return Err(last_error());
    }

    Ok(atom)
}

#[inline]
pub fn send_message<P0, P1, P2>(window: P0, message: u32, param_wide: P1, param_long: P2) -> LRESULT
where
    P0: IntoParam<HWND>,
    P1: IntoParam<WPARAM>,
    P2: IntoParam<LPARAM>,
{
    unsafe { SendMessageW(window, message, param_wide, param_long) }
}
