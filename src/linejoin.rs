#[repr(u32)]
#[deriving(FromPrimitive, Show, Copy)]
pub enum LineJoin {
    Miter = 0,
    Round = 1,
    Bevel = 2
}
