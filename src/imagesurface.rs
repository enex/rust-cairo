use ffi::*;
use format::Format;
use status::Status;
use status::Status::{ InvalidStatus, Success };
use surface::Surface;

/// Internally, this wraps a ```*mut cairo_surface_t```
pub struct ImageSurface {
    ptr : *mut cairo_surface_t,
}

impl ImageSurface {
    pub fn new(format : Format, width : i32, height : i32) -> Result<ImageSurface, &'static str> {
        unsafe {
            let ptr = cairo_image_surface_create(format as i32, width, height);
            if !ptr.is_null() {
                Ok(ImageSurface { ptr : ptr })
            } else {
                Err("Could not create image surface.")
            }
        }
    }

    pub fn stride(&self) -> i32 {
        unsafe {
            cairo_image_surface_get_stride(self.ptr)
        }
    }

    pub fn write_to_png(&self, filename : &str) -> Result<(), Status> {
        use std;
        let filename = std::ffi::CString::from_slice(filename.as_bytes());
        let filename = filename.as_ptr();
        let status = unsafe {
            cairo_surface_write_to_png(self.ptr, filename)
        };
        let status : Option<Status> = std::num::FromPrimitive::from_u32(status);
        match status {
            Some(Success) => Ok(()),
            Some(a)       => Err(a),
            None          => Err(InvalidStatus),
        }
    }
}

impl Surface for ImageSurface {
    fn surface_ptr(&mut self) -> *mut cairo_surface_t {
        self.ptr
    }

    fn width(&self) -> i32 {
        unsafe {
            cairo_image_surface_get_width(self.ptr)
        }
    }

    fn height(&self) -> i32 {
        unsafe {
            cairo_image_surface_get_height(self.ptr)
        }
    }
}

impl Clone for ImageSurface {
    fn clone(&self) -> ImageSurface {
        unsafe {
            cairo_surface_reference(self.ptr);
            ImageSurface { ptr : self.ptr}
        }
    }
}


impl Drop for ImageSurface {
    fn drop(&mut self) {
        unsafe {
            cairo_surface_destroy(self.ptr);
        }
    }
}
