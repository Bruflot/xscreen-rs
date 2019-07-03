use crate::{Display, Window, XGCValues, XGC};
use std::mem;
use x11::xlib;

pub struct GCValues {
    inner: XGCValues,
}

impl GCValues {
    pub fn as_raw(&self) -> XGCValues {
        self.inner
    }
}

impl Default for GCValues {
    fn default() -> Self {
        Self {
            inner: unsafe { mem::zeroed() },
        }
    }
}

pub struct GContext<'a> {
    display: &'a Display,
    inner: XGC,
}

impl<'a> GContext<'a> {
    pub fn new(display: &'a Display, drawable: &Window, value_mask: u64, values: GCValues) -> Self {
        let gc = unsafe {
            xlib::XCreateGC(
                display.as_raw(),
                drawable.as_raw(),
                value_mask,
                values.as_raw(),
            )
        };
        Self { display, inner: gc }
    }

    pub fn set_background(&self, color: u64) {
        unsafe {
            xlib::XSetBackground(self.display.as_raw(), self.inner, color);
        }
    }

    pub fn set_foreground(&self, color: u64) {
        unsafe {
            xlib::XSetForeground(self.display.as_raw(), self.inner, color);
        }
    }

    pub fn as_raw(&self) -> XGC {
        self.inner
    }
}
