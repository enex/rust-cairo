use ffi::*;

#[repr(u32)]
#[derive(FromPrimitive, Show, Copy)]
pub enum Content {
    Color = CAIRO_CONTENT_COLOR,
    Alpha = CAIRO_CONTENT_ALPHA,
    ColorAlpha = CAIRO_CONTENT_COLOR_ALPHA,
}
