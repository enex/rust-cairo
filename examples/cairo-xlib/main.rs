extern crate libc;

extern crate cairo;
extern crate xlib;

use std::ptr;
use std::io::timer;
use std::time::duration::Duration;
use cairo::{ Context, XlibSurface };

// FIXME This example is *terrible* right now...
// FIXME This example crashes when the window is closed.

fn main() {
    let width = 640;
    let height = 480;

    let display = unsafe { xlib::XOpenDisplay(ptr::null_mut()).as_mut() };
    let display = display.unwrap();

    let root_window = unsafe { xlib::XDefaultRootWindow(display) };
    let root_window = root_window;

    let window = unsafe {
        xlib::XCreateSimpleWindow(
            display,
            root_window,
            0, 0, // default location
            width, height, // width, height
            0, 1u64, 0u64)
    };

    let visual = unsafe { xlib::XDefaultVisual(display, 0).as_mut() };
    let visual = visual.unwrap();

    let surface =XlibSurface::new(display, window, visual, width as i32, height as i32);
    let mut surface = surface.unwrap();
    let context = Context::new(&mut surface);
    let mut context = context.unwrap();

    unsafe {
        xlib::XMapWindow(display, window);
        xlib::XFlush(display);
    }

    let mut x = 0;
    let mut y = 0;

    loop {
        context.set_source_rgb(0.0, 0.0, 0.0);
        context.rectangle(0.0, 0.0, width as f64, height as f64);
        context.fill();

        context.set_source_rgb(1.0, 1.0, 1.0);
        context.set_line_width(1.0);

        context.move_to(x as f64, y as f64);
        context.line_to((width - x) as f64, (height - y) as f64);

        x = (x + 5) % width;
        y = (y + 5) % height;

        context.stroke();

        unsafe { xlib::XFlush(display); }

        timer::sleep(Duration::milliseconds(10));
    }

    unsafe { xlib::XCloseDisplay(display); }
}
