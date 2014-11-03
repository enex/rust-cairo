use ffi::*;
use ffi::xlib::*;
use surface::Surface;
use xlib::{ Display, Drawable, Pixmap, Screen, Visual };

///
///
/// Internally, this wraps a ```*mut cairo_surface_t```
pub struct XlibSurface {
    ptr : *mut cairo_surface_t,
}

impl XlibSurface {
    pub fn new(display : &mut Display, 
               drawable : Drawable,
               visual : &mut Visual,
               width : i32,
               height : i32)
               -> Result<XlibSurface, &'static str> {
        unsafe {
            let ptr = cairo_xlib_surface_create(
                display, drawable, visual, width, height);
            if ptr.is_not_null() {
                Ok(XlibSurface { ptr : ptr })
            } else {
                Err("Failed to create XlibSurface.")
            }
        }
    }

    pub fn new_with_bitmap(display : &mut Display,
                           bitmap : Pixmap,
                           screen : &mut Screen,
                           width : i32,
                           height : i32)
                           -> Result<XlibSurface, &'static str> {
        unsafe {
            let ptr = cairo_xlib_surface_create_for_bitmap(
                display, bitmap, screen, width, height);
            if ptr.is_not_null() {
                Ok(XlibSurface { ptr : ptr })
            } else {
                Err("Failed to create XlibSurface.")
            }
        }
    }

    pub fn set_size(&mut self, width : i32, height : i32) {
        unsafe {
            cairo_xlib_surface_set_size(self.ptr, width, height);
        }
    }

    pub fn set_drawable(&mut self, drawable : Drawable, width : i32, height : i32) {
        unsafe {
            cairo_xlib_surface_set_drawable(self.ptr, drawable, width, height);
        }
    }

    pub fn get_display(&mut self) -> *mut Display {
        unsafe {
            cairo_xlib_surface_get_display(self.ptr)
        }
    }

    pub fn get_drawable(&mut self) -> Drawable {
        unsafe {
            cairo_xlib_surface_get_drawable(self.ptr)
        }
    }

    pub fn get_screen(&mut self) -> *mut Screen {
        unsafe {
            cairo_xlib_surface_get_screen(self.ptr)
        }
    }

    pub fn get_visual(&mut self) -> *mut Visual {
        unsafe {
            cairo_xlib_surface_get_visual(self.ptr)
        }
    }

    pub fn get_depth(&mut self) -> i32 {
        unsafe {
            cairo_xlib_surface_get_depth(self.ptr)
        }
    }
}

impl Surface for XlibSurface {
    fn surface_ptr(&mut self) -> *mut cairo_surface_t {
        self.ptr
    }

    fn width(&self) -> i32 {
        unsafe {
            cairo_xlib_surface_get_width(self.ptr)
        }
    }

    fn height(&self) -> i32 {
        unsafe {
            cairo_xlib_surface_get_height(self.ptr)
        }
    }
}

impl Clone for XlibSurface {
    fn clone(&self) -> XlibSurface {
        unsafe {
            cairo_surface_reference(self.ptr);
            XlibSurface { ptr : self.ptr}
        }
    }
}


impl Drop for XlibSurface {
    fn drop(&mut self) {
        unsafe {
            cairo_surface_destroy(self.ptr);
        }
    }
}
