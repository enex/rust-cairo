#[repr(u32)]
#[deriving(FromPrimitive, Show, Copy)]
pub enum FillRule {
    Winding = 0,
    EvenOdd = 1
}
