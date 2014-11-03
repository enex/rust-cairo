use antialias::Antialias;
use content::Content;
use ffi::*;
use fillrule::FillRule;
use linecap::LineCap;
use linejoin::LineJoin;
use matrix::Matrix;
use operator::Operator;
use pattern::Pattern;
use surface::Surface;

///
///
/// Internally, this wraps a ```*mut cairo_t```
pub struct Context {
    ptr  : *mut cairo_t,
}

impl Context {
    pub fn new(surface : &mut Surface) -> Result<Context, &'static str> {
        unsafe {
            let cr = cairo_create(surface.surface_ptr());
            if cr.is_not_null() {
                Ok(Context { ptr : cr })
            } else {
                Err("Could not create cairo context")
            }
        }
    }

    pub unsafe fn cairo_ptr(&mut self) -> *mut cairo_t {
        self.ptr
    }

    pub fn get_reference_count(&mut self) -> u32 {
        unsafe {
            cairo_get_reference_count(self.ptr)
        }
    }

    pub fn save(&mut self) {
        unsafe {
            cairo_save(self.ptr);
        }
    }

    pub fn restore(&mut self) {
        unsafe {
            cairo_restore(self.ptr);
        }
    }

    pub fn push_group(&mut self) {
        unsafe {
            cairo_push_group(self.ptr);
        }
    }

    pub fn push_group_with_content(&mut self, content : Content) {
        unsafe {
            cairo_push_group_with_content(self.ptr, content as u32);
        }
    }

    pub fn pop_group(&mut self) -> Pattern {
        unsafe {
            Pattern::new_from_ptr(cairo_pop_group(self.ptr))
        }
    }

    pub fn pop_group_to_source(&mut self) {
        unsafe {
            cairo_pop_group_to_source(self.ptr);
        }
    }

    pub fn set_operator(&mut self, op : Operator) {
        unsafe {
            cairo_set_operator(self.ptr, op as u32);
        }
    }

    pub fn set_source(&mut self, source : &mut Pattern) {
        unsafe {
            cairo_set_source(self.ptr, source.pattern_ptr());
        }
    }

    pub fn set_source_rgb(&mut self, r : f64, g : f64, b : f64) {
        unsafe {
            cairo_set_source_rgb(self.ptr, r, g, b);
        }
    }

    pub fn set_source_rgba(&mut self, r : f64, g : f64, b : f64) {
        unsafe {
            cairo_set_source_rgb(self.ptr, r, g, b);
        }
    }

    pub fn set_source_surface(&mut self, surface : &mut Surface, x : f64, y : f64) {
        unsafe {
            cairo_set_source_surface(self.ptr, surface.surface_ptr(), x, y);
        }
    }

    pub fn set_tolerance(&mut self, tolerance : f64) {
        unsafe {
            cairo_set_tolerance(self.ptr, tolerance);
        }
    }

    pub fn set_antialias(&mut self, antialias : Antialias) {
        unsafe {
            cairo_set_antialias(self.ptr, antialias as u32);
        }
    }

    pub fn set_fill_rule(&mut self, fill_rule: FillRule) {
        unsafe {
            cairo_set_fill_rule(self.ptr, fill_rule as u32);
        }
    }

    pub fn set_line_width(&mut self, width: f64) {
        unsafe {
            cairo_set_line_width(self.ptr, width);
        }
    }

    pub fn set_line_cap(&mut self, line_cap: LineCap) {
        unsafe {
            cairo_set_line_cap(self.ptr, line_cap as u32);
        }
    }

    pub fn set_line_join(&mut self, line_join: LineJoin) {
        unsafe {
            cairo_set_line_join(self.ptr, line_join as u32);
        }
    }

    pub fn set_dash(&mut self, dashes : &f64, num_dashes: i32, offset: f64) {
        unsafe {
            cairo_set_dash(self.ptr, dashes, num_dashes, offset);
        }
    }

    pub fn set_miter_limit(&mut self, limit: f64) {
        unsafe {
            cairo_set_miter_limit(self.ptr, limit);
        }
    }

    pub fn translate(&mut self, tx: f64, ty: f64) {
        unsafe {
            cairo_translate(self.ptr, tx, ty);
        }
    }

    pub fn scale(&mut self, sx: f64, sy: f64) {
        unsafe {
            cairo_scale(self.ptr, sx, sy);
        }
    }

    pub fn rotate(&mut self, angle: f64) {
        unsafe {
            cairo_rotate(self.ptr, angle);
        }
    }

    pub fn transform(&mut self, matrix : &Matrix) {
        unsafe {
            cairo_transform(self.ptr, matrix.as_cairo_matrix_t());
        }
    }

    pub fn set_matrix(&mut self, matrix : &Matrix) {
        unsafe {
            cairo_set_matrix(self.ptr, matrix.as_cairo_matrix_t());
        }
    }

    pub fn identity_matrix(&mut self) {
        unsafe {
            cairo_identity_matrix(self.ptr);
        }
    }

    pub fn user_to_device(&mut self, x : &mut f64, y : &mut f64) {
        unsafe {
            cairo_user_to_device(self.ptr, x, y);
        }
    }

    pub fn user_to_device_distance(&mut self, dx : &mut f64, dy : &mut f64) {
        unsafe {
            cairo_user_to_device_distance(self.ptr, dx, dy);
        }
    }

    pub fn device_to_user(&mut self, x : &mut f64, y : &mut f64) {
        unsafe {
            cairo_device_to_user(self.ptr, x, y);
        }
    }

    pub fn device_to_user_distance(&mut self, dx : &mut f64, dy : &mut f64) {
        unsafe {
            cairo_device_to_user_distance(self.ptr, dx, dy);
        }
    }

    pub fn new_path(&mut self) {
        unsafe {
            cairo_new_path(self.ptr);
        }
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
        unsafe {
            cairo_move_to(self.ptr, x, y);
        }
    }

    pub fn new_sub_path(&mut self) {
        unsafe {
            cairo_new_sub_path(self.ptr);
        }
    }

    pub fn line_to(&mut self, x: f64, y: f64) {
        unsafe {
            cairo_line_to(self.ptr, x, y);
        }
    }

    pub fn curve_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        unsafe {
            cairo_curve_to(self.ptr, x1, y1, x2, y2, x3, y3);
        }
    }

    pub fn arc(&mut self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64) {
        unsafe {
            cairo_arc(self.ptr, xc, yc, radius, angle1, angle2);
        }
    }

    pub fn arc_negative(&mut self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64) {
        unsafe {
            cairo_arc_negative(self.ptr, xc, yc, radius, angle1, angle2);
        }
    }

    pub fn rel_move_to(&mut self, dx: f64, dy: f64) {
        unsafe {
            cairo_rel_move_to(self.ptr, dx, dy);
        }
    }

    pub fn rel_line_to(&mut self, dx: f64, dy: f64) {
        unsafe {
            cairo_rel_line_to(self.ptr, dx, dy);
        }
    }

    pub fn rel_curve_to(&mut self, dx1: f64, dy1: f64, dx2: f64, dy2: f64, dx3: f64, dy3: f64) {
        unsafe {
            cairo_rel_curve_to(self.ptr, dx1, dy1, dx2, dy2, dx3, dy3);
        }
    }

    pub fn rectangle(&mut self, x: f64, y: f64, width: f64, height: f64) {
        unsafe {
            cairo_rectangle(self.ptr, x, y, width, height);
        }
    }

    pub fn close_path(&mut self) {
        unsafe {
            cairo_close_path(self.ptr);
        }
    }

    pub fn path_extents(&mut self, x1 : &mut f64, y1 : &mut f64, x2 : &mut f64, y2 : &mut f64) {
        unsafe {
            cairo_path_extents(self.ptr, x1, y1, x2, y2);
        }
    }

    pub fn paint(&mut self) {
        unsafe {
            cairo_paint(self.ptr);
        }
    }

    pub fn paint_with_alpha(&mut self, alpha: f64) {
        unsafe {
            cairo_paint_with_alpha(self.ptr, alpha);
        }
    }

    pub fn mask(&mut self, pattern : &mut Pattern) {
        unsafe {
            cairo_mask(self.ptr, pattern.pattern_ptr());
        }
    }

    pub fn mask_surface(&mut self, surface : &mut Surface, surface_x: f64, surface_y: f64) {
        unsafe {
            cairo_mask_surface(self.ptr, surface.surface_ptr(), surface_x, surface_y);
        }
    }

    pub fn stroke(&mut self) {
        unsafe {
            cairo_stroke(self.ptr);
        }
    }

    pub fn stroke_preserve(&mut self) {
        unsafe {
            cairo_stroke_preserve(self.ptr);
        }
    }

    pub fn fill(&mut self) {
        unsafe {
            cairo_fill(self.ptr);
        }
    }

    pub fn fill_preserve(&mut self) {
        unsafe {
            cairo_fill_preserve(self.ptr);
        }
    }

    pub fn copy_page(&mut self) {
        unsafe {
            cairo_copy_page(self.ptr);
        }
    }

    pub fn show_page(&mut self) {
        unsafe {
            cairo_show_page(self.ptr);
        }
    }

    pub fn in_stroke(&mut self, x: f64, y: f64) -> bool {
        unsafe {
            cairo_in_stroke(self.ptr, x, y) != 0
        }
    }

    pub fn in_fill(&mut self, x: f64, y: f64) -> bool {
        unsafe {
            cairo_in_fill(self.ptr, x, y) != 0
        }
    }

    pub fn in_clip(&mut self, x: f64, y: f64) -> bool {
        unsafe {
            cairo_in_clip(self.ptr, x, y) != 0
        }
    }

    pub fn stroke_extents(&mut self, x1 : &mut f64, y1 : &mut f64, x2 : &mut f64, y2 : &mut f64) {
        unsafe {
            cairo_stroke_extents(self.ptr, x1, y1, x2, y2);
        }
    }

    pub fn fill_extents(&mut self, x1 : &mut f64, y1 : &mut f64, x2 : &mut f64, y2 : &mut f64) {
        unsafe {
            cairo_fill_extents(self.ptr, x1, y1, x2, y2);
        }
    }

    pub fn reset_clip(&mut self) {
        unsafe {
            cairo_reset_clip(self.ptr);
        }
    }

    pub fn clip(&mut self) {
        unsafe {
            cairo_clip(self.ptr);
        }
    }

    pub fn clip_preserve(&mut self) {
        unsafe {
            cairo_clip_preserve(self.ptr);
        }
    }

    pub fn clip_extents(&mut self, x1 : &mut f64, y1 : &mut f64, x2 : &mut f64, y2 : &mut f64) {
        unsafe {
            cairo_clip_extents(self.ptr, x1, y1, x2, y2);
        }
    }

    pub fn copy_clip_rectangle_list(&mut self) -> *mut cairo_rectangle_list_t {
        unimplemented!();
    }
}

impl Clone for Context {
    fn clone(&self) -> Context {
        unsafe {
            cairo_reference(self.ptr);
            Context { ptr : self.ptr }
        }
    }
}


impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            cairo_destroy(self.ptr);
        }
    }
}