use ffi::*;

/// 
///
/// Internally, this wraps a ```*mut cairo_pattern_t```
pub struct Pattern {
    ptr : *mut cairo_pattern_t,
}

impl Pattern {
    pub fn new_from_ptr(ptr : *mut cairo_pattern_t) -> Pattern {
        assert!(ptr.is_not_null());
        Pattern { ptr : ptr }
    }

    pub unsafe fn pattern_ptr(&mut self) -> *mut cairo_pattern_t {
        self.ptr
    }
}

impl Clone for Pattern {
    fn clone(&self) -> Pattern {
        unsafe {
            cairo_pattern_reference(self.ptr);
            Pattern { ptr : self.ptr }
        }
    }
}


impl Drop for Pattern {
    fn drop(&mut self) {
        unsafe {
            cairo_pattern_destroy(self.ptr);
        }
    }
}