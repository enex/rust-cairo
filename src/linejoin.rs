use ffi::*;

#[repr(u32)]
#[deriving(FromPrimitive, Show, Copy)]
pub enum LineJoin {
    Miter = CAIRO_LINE_JOIN_MITER,
    Round = CAIRO_LINE_JOIN_ROUND,
    Bevel = CAIRO_LINE_JOIN_BEVEL,
}
