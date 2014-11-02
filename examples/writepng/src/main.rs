#![feature(if_let)]

extern crate cairo;

use cairo::{ Context, ImageSurface };

fn main() {
    let mut surface = ImageSurface::new(cairo::ARGB32, 256, 256).unwrap();
    let mut context = Context::new(&mut surface).unwrap();

    context.set_source_rgb(0.0, 0.0, 0.0);
    context.set_line_width(1.0);

    context.move_to(0.0, 0.0);
    context.line_to(256.0, 256.0);

    context.stroke();

    match surface.write_to_png("write.png") {
        Ok(()) => println!("Write successful."),
        Err(a) => println!("Write failed: {}", a)
    }
}
