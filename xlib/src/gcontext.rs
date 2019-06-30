use crate::{Display, XGCValues, XGC};
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

pub struct GContext {
    inner: XGC,
}

impl GContext {
    pub fn new<T: Into<u64>>(
        display: &Display,
        drawable: T,
        value_mask: u64,
        values: GCValues,
    ) -> Self {
        let gc = unsafe {
            xlib::XCreateGC(
                display.as_raw(),
                drawable.into(),
                value_mask,
                values.as_raw(),
            )
        };
        Self { inner: gc }
    }

    pub fn flush(&self) {}

    fn free(&self) {}

    pub fn as_raw(&self) -> XGC {
        self.inner
    }
}
