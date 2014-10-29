use ffi::cairo_surface_t;

pub trait Surface : Drop + Clone {
    unsafe fn surface_ptr(&mut self) -> *mut cairo_surface_t;
    fn width(&self) -> i32;
    fn height(&self) -> i32;
}
