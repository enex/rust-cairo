extern crate libc;

extern crate cairo;
extern crate xlib;

use std::ptr;

fn main() {
    let display = unsafe {
        xlib::XOpenDisplay(
            ptr::null_mut::<libc::c_char>())
    };
    let root_window = unsafe {
        xlib::XDefaultRootWindow(display)
    };
    let window = unsafe {
        xlib::XCreateSimpleWindow(
            display,
            root_window,
            0, 0, // default location
            640, 480, // width, height
            0, 1u64, 0u64)
    };

    unsafe {
        xlib::XMapWindow(display, window);
        xlib::XFlush(display);

        for var in range(0, 1000000i) {
            println!("hello");
        }

        xlib::XCloseDisplay(display);
    }
}
