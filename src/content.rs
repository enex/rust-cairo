#[repr(u32)]
#[deriving(FromPrimitive, Show, Copy)]
pub enum Content {
    Color = 4096,
    Alpha = 8192,
    ColorAlpha = 12288,
}
