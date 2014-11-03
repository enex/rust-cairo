use ffi::*;

#[repr(u32)]
#[deriving(FromPrimitive, Show, Copy)]
pub enum Antialias {
    Default = CAIRO_ANTIALIAS_DEFAULT,
    None = CAIRO_ANTIALIAS_NONE,
    Gray = CAIRO_ANTIALIAS_GRAY,
    Subpixel = CAIRO_ANTIALIAS_SUBPIXEL,
    Fast = CAIRO_ANTIALIAS_FAST,
    Good = CAIRO_ANTIALIAS_GOOD,
    Best = CAIRO_ANTIALIAS_BEST,
}
