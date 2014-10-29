#[repr(u32)]
#[deriving(FromPrimitive, Show, Copy)]
pub enum LineCap {
    Butt = 0,
    Round = 1,
    Square = 2
}
