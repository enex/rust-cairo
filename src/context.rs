use antialias::Antialias;
use ffi::*;
use fillrule::FillRule;
use linecap::LineCap;
use linejoin::LineJoin;
use operator::Operator;
use surface::Surface;

pub struct Context {
    cairo  : *mut cairo_t,
}

impl Context {
    pub fn new(surface : &mut Surface) -> Result<Context, &'static str> {
        unsafe {
            let cairo = cairo_create(surface.surface_ptr());
            if cairo.is_not_null() {
                Ok(Context { cairo : cairo })
            } else {
                Err("Could not create cairo context")
            }
        }
    }

    pub fn get_reference_count(&mut self) -> u32 {
        unsafe {
            cairo_get_reference_count(self.cairo)
        }
    }

    pub fn save(&mut self) {
        unsafe {
            cairo_save(self.cairo);
        }
    }

    pub fn restore(&mut self) {
        unsafe {
            cairo_restore(self.cairo);
        }
    }

    pub fn push_group(&mut self) {
        unsafe {
            cairo_push_group(self.cairo);
        }
    }

    pub fn push_group_with_content(&mut self, content : cairo_content_t) {
        unimplemented!();
    }

    pub fn pop_group(&mut self) -> *mut cairo_pattern_t {
        unsafe {
            cairo_pop_group(self.cairo)
        }
    }

    pub fn pop_group_to_source(&mut self) {
        unsafe {
            cairo_pop_group_to_source(self.cairo);
        }
    }

    pub fn set_operator(&mut self, op : Operator) {
        unsafe {
            cairo_set_operator(self.cairo, op as u32);
        }
    }

    pub fn set_source(&mut self, source  : &mut cairo_pattern_t) {
        unimplemented!();
    }

    pub fn set_source_rgb(&mut self, r : f64, g : f64, b : f64) {
        unsafe {
            cairo_set_source_rgb(self.cairo, r, g, b);
        }
    }

    pub fn set_source_rgba(&mut self, r : f64, g : f64, b : f64) {
        unsafe {
            cairo_set_source_rgb(self.cairo, r, g, b);
        }
    }

    pub fn set_source_surface(&mut self, surface : &mut Surface, x : f64, y : f64) {
        unsafe {
            cairo_set_source_surface(self.cairo, surface.surface_ptr(), x, y);
        }
    }

    pub fn set_tolerance(&mut self, tolerance : f64) {
        unsafe {
            cairo_set_tolerance(self.cairo, tolerance);
        }
    }

    pub fn set_antialias(&mut self, antialias : Antialias) {
        unsafe {
            cairo_set_antialias(self.cairo, antialias as u32);
        }
    }

    pub fn set_fill_rule(&mut self, fill_rule: FillRule) {
        unsafe {
            cairo_set_fill_rule(self.cairo, fill_rule as u32);
        }
    }

    pub fn set_line_width(&mut self, width: f64) {
        unsafe {
            cairo_set_line_width(self.cairo, width);
        }
    }

    pub fn set_line_cap(&mut self, line_cap: LineCap) {
        unsafe {
            cairo_set_line_cap(self.cairo, line_cap as u32);
        }
    }

    pub fn set_line_join(&mut self, line_join: LineJoin) {
        unsafe {
            cairo_set_line_join(self.cairo, line_join as u32);
        }
    }

    pub fn set_dash(&mut self, dashes : &f64, num_dashes: i32, offset: f64) {
        unsafe {
            cairo_set_dash(self.cairo, dashes, num_dashes, offset);
        }
    }

    pub fn set_miter_limit(&mut self, limit: f64) {
        unsafe {
            cairo_set_miter_limit(self.cairo, limit);
        }
    }

    pub fn translate(&mut self, tx: f64, ty: f64) {
        unsafe {
            cairo_translate(self.cairo, tx, ty);
        }
    }

    pub fn scale(&mut self, sx: f64, sy: f64) {
        unsafe {
            cairo_scale(self.cairo, sx, sy);
        }
    }

    pub fn rotate(&mut self, angle: f64) {
        unsafe {
            cairo_rotate(self.cairo, angle);
        }
    }

    pub fn transform(&mut self, matrix : &cairo_matrix_t) {
        unimplemented!();
    }

    pub fn set_matrix(&mut self, matrix : &cairo_matrix_t) {
        unimplemented!();
    }

    pub fn identity_matrix(&mut self) {
        unsafe {
            cairo_identity_matrix(self.cairo);
        }
    }

    pub fn user_to_device(&mut self, x : &mut f64, y : &mut f64) {
        unsafe {
            cairo_user_to_device(self.cairo, x, y);
        }
    }

    pub fn user_to_device_distance(&mut self, dx : &mut f64, dy : &mut f64) {
        unsafe {
            cairo_user_to_device_distance(self.cairo, dx, dy);
        }
    }

    pub fn device_to_user(&mut self, x : &mut f64, y : &mut f64) {
        unsafe {
            cairo_device_to_user(self.cairo, x, y);
        }
    }

    pub fn device_to_user_distance(&mut self, dx : &mut f64, dy : &mut f64) {
        unsafe {
            cairo_device_to_user_distance(self.cairo, dx, dy);
        }
    }

    pub fn new_path(&mut self) {
        unsafe {
            cairo_new_path(self.cairo);
        }
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
        unsafe {
            cairo_move_to(self.cairo, x, y);
        }
    }

    pub fn new_sub_path(&mut self) {
        unsafe {
            cairo_new_sub_path(self.cairo);
        }
    }

    pub fn line_to(&mut self, x: f64, y: f64) {
        unsafe {
            cairo_line_to(self.cairo, x, y);
        }
    }

    pub fn curve_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        unsafe {
            cairo_curve_to(self.cairo, x1, y1, x2, y2, x3, y3);
        }
    }

    pub fn arc(&mut self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64) {
        unsafe {
            cairo_arc(self.cairo, xc, yc, radius, angle1, angle2);
        }
    }

    pub fn arc_negative(&mut self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64) {
        unsafe {
            cairo_arc_negative(self.cairo, xc, yc, radius, angle1, angle2);
        }
    }

    pub fn rel_move_to(&mut self, dx: f64, dy: f64) {
        unsafe {
            cairo_rel_move_to(self.cairo, dx, dy);
        }
    }

    pub fn rel_line_to(&mut self, dx: f64, dy: f64) {
        unsafe {
            cairo_rel_line_to(self.cairo, dx, dy);
        }
    }

    pub fn rel_curve_to(&mut self, dx1: f64, dy1: f64, dx2: f64, dy2: f64, dx3: f64, dy3: f64) {
        unsafe {
            cairo_rel_curve_to(self.cairo, dx1, dy1, dx2, dy2, dx3, dy3);
        }
    }

    pub fn rectangle(&mut self, x: f64, y: f64, width: f64, height: f64) {
        unsafe {
            cairo_rectangle(self.cairo, x, y, width, height);
        }
    }

    pub fn close_path(&mut self) {
        unsafe {
            cairo_close_path(self.cairo);
        }
    }

    pub fn path_extents(&mut self, x1 : &mut f64, y1 : &mut f64, x2 : &mut f64, y2 : &mut f64) {
        unsafe {
            cairo_path_extents(self.cairo, x1, y1, x2, y2);
        }
    }

    pub fn paint(&mut self) {
        unsafe {
            cairo_paint(self.cairo);
        }
    }

    pub fn paint_with_alpha(&mut self, alpha: f64) {
        unsafe {
            cairo_paint_with_alpha(self.cairo, alpha);
        }
    }

    pub fn mask(&mut self, pattern : &mut cairo_pattern_t) {
        unimplemented!();
    }

    pub fn mask_surface(&mut self, surface : &mut Surface, surface_x: f64, surface_y: f64) {
        unsafe {
            cairo_mask_surface(self.cairo, surface.surface_ptr(), surface_x, surface_y);
        }
    }

    pub fn stroke(&mut self) {
        unsafe {
            cairo_stroke(self.cairo);
        }
    }

    pub fn stroke_preserve(&mut self) {
        unsafe {
            cairo_stroke_preserve(self.cairo);
        }
    }

    pub fn fill(&mut self) {
        unsafe {
            cairo_fill(self.cairo);
        }
    }

    pub fn fill_preserve(&mut self) {
        unsafe {
            cairo_fill_preserve(self.cairo);
        }
    }

    pub fn copy_page(&mut self) {
        unsafe {
            cairo_copy_page(self.cairo);
        }
    }

    pub fn show_page(&mut self) {
        unsafe {
            cairo_show_page(self.cairo);
        }
    }

    pub fn in_stroke(&mut self, x: f64, y: f64) -> bool {
        unsafe {
            cairo_in_stroke(self.cairo, x, y) != 0
        }
    }

    pub fn in_fill(&mut self, x: f64, y: f64) -> bool {
        unsafe {
            cairo_in_fill(self.cairo, x, y) != 0
        }
    }

    pub fn in_clip(&mut self, x: f64, y: f64) -> bool {
        unsafe {
            cairo_in_clip(self.cairo, x, y) != 0
        }
    }

    pub fn stroke_extents(&mut self, x1 : &mut f64, y1 : &mut f64, x2 : &mut f64, y2 : &mut f64) {
        unsafe {
            cairo_stroke_extents(self.cairo, x1, y1, x2, y2);
        }
    }

    pub fn fill_extents(&mut self, x1 : &mut f64, y1 : &mut f64, x2 : &mut f64, y2 : &mut f64) {
        unsafe {
            cairo_fill_extents(self.cairo, x1, y1, x2, y2);
        }
    }

    pub fn reset_clip(&mut self) {
        unsafe {
            cairo_reset_clip(self.cairo);
        }
    }

    pub fn clip(&mut self) {
        unsafe {
            cairo_clip(self.cairo);
        }
    }

    pub fn clip_preserve(&mut self) {
        unsafe {
            cairo_clip_preserve(self.cairo);
        }
    }

    pub fn clip_extents(&mut self, x1 : &mut f64, y1 : &mut f64, x2 : &mut f64, y2 : &mut f64) {
        unsafe {
            cairo_clip_extents(self.cairo, x1, y1, x2, y2);
        }
    }

    pub fn copy_clip_rectangle_list(&mut self) -> *mut cairo_rectangle_list_t {
        unimplemented!();
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