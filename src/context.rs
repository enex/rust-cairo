use ffi::*;
use surface::Surface;

pub struct Context {
    pub cairo : *mut cairo_t,
}

impl Context {
    pub fn new(surface : &Surface) -> Context {
        unsafe {
            let cairo = cairo_create(surface.surface_ptr());
            assert!(cairo.is_not_null());
            Context { cairo : cairo }
        }
    }

    pub unsafe fn cairo_ptr(&self) -> *mut cairo_t {
        self.cairo
    }

    pub fn set_source_rgb(&self, r : f64, g : f64, b : f64) {
        unsafe {
            cairo_set_source_rgb(self.cairo, r, g, b);
        }
    }

    pub fn set_source_rgba(&self, r : f64, g : f64, b : f64) {
        unsafe {
            cairo_set_source_rgb(self.cairo, r, g, b);
        }
    }

    pub fn set_line_width(&self, width : f64) {
        unsafe {
            cairo_set_line_width(self.cairo, width);
        }
    }

    pub fn move_to(&self, x : f64, y : f64) {
        unsafe {
            cairo_move_to(self.cairo, x, y);
        }
    }

    pub fn line_to(&self, x : f64, y : f64) {
        unsafe {
            cairo_line_to(self.cairo, x, y);
        }
    }

    pub fn stroke(&self) {
        unsafe {
            cairo_stroke(self.cairo);
        }
    }
}

impl Clone for Context {
    fn clone(&self) -> Context {
        unsafe {
            cairo_reference(self.cairo);
            assert!(self.cairo.is_not_null());
            Context { cairo : self.cairo }
        }
    }
}


impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            cairo_destroy(self.cairo);
        }
    }
}