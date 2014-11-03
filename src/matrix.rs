use std::mem;
use ffi::cairo_matrix_t;


///
///
/// Internally, this should be identical to a ```cairo_matrix_t```
#[repr(C)]
pub struct Matrix {
    pub xx : f64,
    pub yx : f64,
    pub xy : f64,
    pub yy : f64,
    pub x0 : f64,
    pub y0 : f64,
}

impl Matrix {
    pub fn as_cairo_matrix_t(&self) -> &cairo_matrix_t {
        unsafe {
            mem::transmute(self)
        }
    }

    pub fn as_mut_cairo_matrix_t(&mut self) -> &mut cairo_matrix_t {
        unsafe {
            mem::transmute(self)
        }
    }
}
