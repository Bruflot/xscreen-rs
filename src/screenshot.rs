extern crate image;
extern crate xlib;

use image::{ImageBuffer, RgbImage};
use std::io;
use std::path::Path;
use xlib::{Display, Image, Rect, Window};

const RED_MASK: u64 = 16_711_680;
const GREEN_MASK: u64 = 65_280;
const BLUE_MASK: u64 = 255;

pub struct Screenshot {
    data: Image,
    width: u32,
    height: u32,
}

impl Screenshot {
    pub fn fullscreen(display: &Display) -> Option<Self> {
        let root = display.default_window();
        let width = display.get_width(0) as u32;
        let height = display.get_height(0) as u32;
        Self::with_rect(
            &display,
            &root,
            Rect {
                x: 0,
                y: 0,
                width,
                height,
            },
        )
    }

    pub fn window(display: &Display, window: &Window) -> Option<Self> {
        let rect = window.get_rect();
        Self::with_rect(
            display,
            window,
            Rect {
                x: 0,
                y: 0,
                width: rect.width,
                height: rect.height,
            },
        )
    }

    pub fn with_rect(display: &Display, window: &Window, rect: Rect) -> Option<Self> {
        Some(Self {
            data: Image::get_image(
                &display,
                &window,
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                xlib::Z_PIXMAP,
            )?,
            width: rect.width,
            height: rect.height,
        })
    }

    #[inline]
    const fn get_colors(pixel: u64) -> [u8; 3] {
        let red = (pixel & RED_MASK) >> 16;
        let green = (pixel & GREEN_MASK) >> 8;
        let blue = pixel & BLUE_MASK;
        [red as u8, green as u8, blue as u8]
    }

    pub fn save<P: AsRef<Path>>(self, path: P) -> io::Result<()> {
        let image: RgbImage = ImageBuffer::from_fn(self.width, self.height, |x, y| {
            let pixel = self.data.get_pixel(x as i32, y as i32);
            let colors = Self::get_colors(pixel);
            image::Rgb(colors)
        });

        image.save(path)
    }
}
