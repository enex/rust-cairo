use ffi::*;

#[repr(i32)]
#[derive(FromPrimitive, Show, Copy)]
#[allow(non_camel_case_types)]
pub enum Format {
    Invalid = CAIRO_FORMAT_INVALID,
    ARGB32 = CAIRO_FORMAT_ARGB32,
    RGB24 = CAIRO_FORMAT_RGB24,
    A8 = CAIRO_FORMAT_A8,
    A1 = CAIRO_FORMAT_A1,
    RGB16_565 = CAIRO_FORMAT_RGB16_565,
    RGB30 = CAIRO_FORMAT_RGB30
}
