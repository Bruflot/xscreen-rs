// XCreateImage, XDestroyImage, XGetImage, XGetSubImage
// XInitImage, XPutImage, XSubImage(?)
use crate::{Display, Window, XImage};
use std::slice;
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
    ) -> Self {
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

            Self {
                inner: img,
                width,
                height,
            }
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
