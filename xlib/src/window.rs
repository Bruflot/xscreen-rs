use crate::{Display, Rect, XDisplay, XWindow};
use std::mem;
use x11::xlib;

pub struct SetWindowAttributes(pub xlib::XSetWindowAttributes);

impl SetWindowAttributes {
    pub fn as_raw(&self) -> xlib::XSetWindowAttributes {
        self.0
    }
}

impl Default for SetWindowAttributes {
    fn default() -> Self {
        unsafe { Self(mem::zeroed()) }
    }
}

#[derive(Debug)]
pub struct Window {
    display: XDisplay,
    bounds: Rect,
    _inner: XWindow,
}

impl Window {
    pub fn new(
        display: &Display,
        bounds: Rect,
        depth: i32,
        visual: *mut xlib::Visual,
        value_mask: u64,
        attributes: &mut SetWindowAttributes,
    ) -> Self {
        let window = unsafe {
            xlib::XCreateWindow(
                display.as_raw(),
                display.default_window()._inner,
                bounds.x,
                bounds.y,
                bounds.width,
                bounds.height,
                0, // border width
                depth,
                crate::INPUT_OUTPUT, // class
                visual,
                value_mask,
                &mut attributes.as_raw(),
            )
        };

        Self {
            display: display.as_raw(),
            bounds,
            _inner: window,
        }
    }

    pub fn new_simple(display: &Display, bounds: Rect) -> Self {
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

    pub const fn as_raw(&self) -> XWindow {
        self._inner
    }

    pub fn grab_keyboard<T: Into<i32>>(
        &self,
        owner_events: T,
        pointer_mode: i32,
        keyboard_mode: i32,
    ) -> i32 {
        unsafe {
            xlib::XGrabKeyboard(
                self.display,
                self._inner,
                owner_events.into(),
                pointer_mode,
                keyboard_mode,
                0,
            )
        }
    }

    pub fn ungrab_keyboard(&self) {
        unsafe {
            xlib::XUngrabKeyboard(self.display, 0);
        }
    }

    pub fn focus(&self, revert_to: i32) {
        unsafe {
            xlib::XSetInputFocus(self.display, self._inner, revert_to, 0);
        }
    }

    pub fn clear(&self) {
        unsafe {
            xlib::XClearWindow(self.display, self._inner);
        }
    }

    // XGetGeometry
    pub fn get_rect(&self) -> Rect {
        let mut window = 0;
        let mut x = 0;
        let mut y = 0;
        let mut width = 0;
        let mut height = 0;
        let mut border_width = 0;
        let mut depth_return = 0;

        unsafe {
            xlib::XGetGeometry(
                self.display,
                self._inner,
                &mut window,
                &mut x,
                &mut y,
                &mut width,
                &mut height,
                &mut border_width,
                &mut depth_return,
            );
        }

        Rect {
            x,
            y,
            width,
            height,
        }
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

    pub fn set_attributes(&self, attributes: &mut SetWindowAttributes, mask: u64) {
        unsafe {
            xlib::XChangeWindowAttributes(
                self.display,
                self._inner,
                mask,
                &mut attributes.as_raw(),
            );
        }
    }

    // TODO: get window attributes
    // pub fn get_width(&self){}

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
