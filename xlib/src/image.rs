// XCreateImage, XDestroyImage, XGetImage, XGetSubImage
// XInitImage, XPutImage, XSubImage(?)
use crate::{Display, Window, XImage};
use std::{mem, slice};
use x11::xlib;

pub struct Image {
    inner: XImage,
    width: u32,
    height: u32,
}

impl Image {
    pub fn get_image(
        display: &Display,
        window: &Window,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        format: i32,
    ) -> Option<Self> {
        unsafe {
            let img = xlib::XGetImage(
                display.as_raw(),
                window.as_raw(),
                x,
                y,
                width,
                height,
                xlib::XAllPlanes(),
                format,
            );

            if img.is_null(){
                return None;
            }

            Some(Self {
                inner: img,
                width,
                height,
            })
        }
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> u64 {
        unsafe { xlib::XGetPixel(self.inner, x, y) }
    }

    pub fn get_data(&self) -> &[u8] {
        unsafe {
            let data = (*self.inner).data;
            slice::from_raw_parts(data as *mut u8, (self.width * self.height * 4) as usize)
        }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            if !self.inner.is_null() {
                xlib::XDestroyImage(self.inner);
            }
        }
    }
}

pub struct Visual(pub xlib::Visual);

impl Visual {
    pub fn as_raw(&self) -> xlib::Visual {
        self.0
    }
}

impl Default for Visual {
    fn default() -> Self {
        unsafe { Self(mem::zeroed()) }
    }
}

pub struct VisualInfo(pub xlib::XVisualInfo);

impl VisualInfo {
    pub fn as_raw(&self) -> xlib::XVisualInfo {
        self.0
    }

    pub fn from(display: &Display, screen: i32, depth: i32, color: i32) -> Self {
        let mut info = Self::default();
        let mut raw = info.as_raw();
        unsafe {
            xlib::XMatchVisualInfo(display.as_raw(), screen, depth, color, &mut raw);
        }
        info.0 = raw;
        info
    }
}

impl Default for VisualInfo {
    fn default() -> Self {
        unsafe { Self(mem::zeroed()) }
    }
}
