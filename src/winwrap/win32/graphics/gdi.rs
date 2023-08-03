pub use crate::winwrap::*;
pub use windows::Win32::Graphics::Gdi::*;

#[inline]
pub fn begin_paint<P0: IntoParam<HWND>>(window: P0, paint_struct: &mut PAINTSTRUCT) -> HDC {
    unsafe { BeginPaint(window, paint_struct) }
}

#[inline]
pub fn fill_rect<P0, P1>(device_context: P0, rect: &RECT, brush: P1) -> i32
where
    P0: IntoParam<HDC>,
    P1: IntoParam<HBRUSH>,
{
    unsafe { FillRect(device_context, rect, brush) }
}

#[inline]
pub fn end_paint<P0: IntoParam<HWND>>(window: P0, paint_struct: &PAINTSTRUCT) {
    unsafe {
        EndPaint(window, paint_struct);
    }
}
