use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
use x11::xlib;

pub struct RootWindow {
    display: *mut xlib::Display,
    root_window: u64,
}

impl RootWindow {
    pub fn update_bar(&self, content: String) {
        unsafe {
            let status_c = CString::new(content).unwrap();
            xlib::XStoreName(
                self.display,
                self.root_window,
                status_c.as_ptr() as *mut c_char,
            );
            xlib::XFlush(self.display);
        }
    }

    pub fn init_window() -> Self {
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());
            let root_window = xlib::XRootWindow(display, xlib::XDefaultScreen(display));

            Self {
                display,
                root_window,
            }
        }
    }
}

impl Drop for RootWindow {
    fn drop(&mut self) {
        unsafe {
            xlib::XCloseDisplay(self.display);
        }
    }
}
