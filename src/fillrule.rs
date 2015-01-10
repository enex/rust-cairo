use ffi::*;

#[repr(u32)]
#[derive(FromPrimitive, Show, Copy)]
pub enum FillRule {
    Winding = CAIRO_FILL_RULE_WINDING,
    EvenOdd = CAIRO_FILL_RULE_EVEN_ODD,
}
