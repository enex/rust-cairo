use ffi::*;

#[repr(u32)]
#[deriving(FromPrimitive, Show, Copy)]
pub enum LineCap {
    Butt = CAIRO_LINE_CAP_BUTT,
    Round = CAIRO_LINE_CAP_ROUND,
    Square = CAIRO_LINE_CAP_SQUARE,
}
