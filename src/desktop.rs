use std::{process, ptr};
use x11::{xlib, xtest};

pub struct Desktop {
    display: *mut xlib::_XDisplay,
    root: xlib::Window,
}

#[derive(PartialEq)]
pub enum ClickType {
    Once,
    Down,
    Up,
}

impl Desktop {
    pub fn new() -> Self {
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());

            if display.is_null() {
                println!("Could not open Display");
                process::exit(1);
            }

            let root = xlib::XDefaultRootWindow(display);

            Desktop { display, root }
        }
    }

    pub fn move_cursor(&self, x: u32, y: u32) {
        unsafe {
            xlib::XWarpPointer(self.display, 0, self.root, 0, 0, 0, 0, x as _, y as _);
            xlib::XFlush(self.display);
        }
    }

    pub fn left_click(&self, click_type: ClickType) {
        unsafe {
            // display, button, is_press, delay
            if click_type == ClickType::Down || click_type == ClickType::Once {
                xtest::XTestFakeButtonEvent(self.display, 1, 1, xlib::CurrentTime);
            }
            if click_type == ClickType::Up || click_type == ClickType::Once {
                xtest::XTestFakeButtonEvent(self.display, 1, 0, xlib::CurrentTime);
            }

            xlib::XFlush(self.display);
        }
    }
}

// Probably fine, no 2 draw processes are running concurrently
unsafe impl Send for Desktop {}
unsafe impl Sync for Desktop {}

impl Drop for Desktop {
    fn drop(&mut self) {
        unsafe {
            xlib::XFlush(self.display);
            xlib::XCloseDisplay(self.display);
        }
    }
}
