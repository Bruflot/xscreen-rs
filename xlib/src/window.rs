use crate::{Display, Rect, XDisplay, XWindow};
use x11::xlib;

pub type WindowAttributes = xlib::XSetWindowAttributes;

#[derive(Debug)]
pub struct Window {
    display: XDisplay,
    bounds: Rect,
    _inner: XWindow,
}

impl Window {
    pub fn new(display: &Display, bounds: Rect) -> Self {
        let window = unsafe {
            xlib::XCreateSimpleWindow(
                display.as_raw(),
                display.default_window()._inner,
                bounds.x,
                bounds.y,
                bounds.width,
                bounds.height,
                0,          // border width
                0,          // border color
                16_777_215, // bg color (256^3-1 = white)
            )
        };

        Self {
            display: display.as_raw(),
            bounds,
            _inner: window,
        }
    }

    pub fn from_raw(display: &Display, window: XWindow) -> Self {
        Self {
            display: display.as_raw(),
            bounds: Rect::default(),
            _inner: window,
        }
    }

    pub fn as_raw(&self) -> XWindow {
        self._inner
    }

    pub fn get_bounds(&self) -> Rect {
        self.bounds
    }

    pub fn move_resize(&mut self, bounds: Rect) {
        self.bounds = bounds;
        unsafe {
            xlib::XMoveResizeWindow(
                self.display,
                self._inner,
                self.bounds.x,
                self.bounds.y,
                self.bounds.width,
                self.bounds.height,
            );
        }
    }

    pub fn set_attributes(&self, attributes: &mut WindowAttributes, mask: u64) {
        unsafe {
            xlib::XChangeWindowAttributes(self.display, self._inner, mask, attributes);
        }
    }

    pub fn destroy(&mut self) {
        unsafe {
            xlib::XDestroyWindow(self.display, self._inner);
        }
    }
}

// impl Drop for Window {
//     fn drop(&mut self) {
//         // Destroys (and unmaps) the window
//         unsafe {
//             xlib::XUnmapWindow(self.display, self.inner);
//         }
//     }
// }
