extern crate libc;
use crate::{Event, GContext, Rect, Window, XDisplay, XError, XEvent, XResult, XWindow};
use std::ffi::CString;
use std::mem;
use std::ptr;
use x11::xlib;

#[derive(Debug)]
pub struct CursorInfo {
    pub parent: Option<Window>,
    pub child: Option<Window>,
    pub x: i32,
    pub y: i32,
}

impl Default for CursorInfo {
    fn default() -> Self {
        Self {
            parent: None,
            child: None,
            x: 0,
            y: 0,
        }
    }
}

#[derive(Debug)]
pub struct Atom(pub u64);

#[derive(Debug)]
pub struct Display {
    inner: XDisplay,
}

impl Display {
    // XOpenDisplay
    pub fn connect(display_name: Option<&str>) -> XResult<Display> {
        let display_name = match display_name {
            Some(name) => {
                let c_str = CString::new(name).unwrap();
                c_str.as_ptr()
            }
            None => ptr::null(),
        };
        let display = unsafe { xlib::XOpenDisplay(display_name) };

        if display.is_null() {
            return Err(XError::ConnectionError);
        }
        Ok(Self { inner: display })
    }

    // XDefaultRootWindow
    pub fn default_window(&self) -> Window {
        let window = unsafe { xlib::XDefaultRootWindow(self.inner) };
        Window::from_raw(self, window)
    }

    pub fn create_font_cursor(&self, cursor: u32) -> u64 {
        unsafe { xlib::XCreateFontCursor(self.inner, cursor) }
    }

    pub fn clear_area<T: Into<i32>>(
        &self,
        window: &Window,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        exposures: T,
    ) {
        unsafe {
            xlib::XClearArea(
                self.inner,
                window.as_raw(),
                x,
                y,
                width,
                height,
                exposures.into(),
            );
        }
    }

    pub fn define_cursor(&self, window: &Window, cursor: u64) {
        unsafe {
            xlib::XDefineCursor(self.inner, window.as_raw(), cursor);
        }
    }

    pub const fn as_raw(&self) -> XDisplay {
        self.inner
    }

    pub fn create_colormap(&self, window: &Window, visual: *mut xlib::Visual, alloc: i32) -> u64 {
        unsafe { xlib::XCreateColormap(self.inner, window.as_raw(), visual, alloc) }
    }

    // XFlush
    pub fn flush(&self) {
        unsafe {
            xlib::XFlush(self.inner);
        }
    }

    // XTranslateCoordinates
    pub fn translate_coordinates(&self, window: &Window, x: i32, y: i32) -> (i32, i32) {
        let mut ret_x = 0;
        let mut ret_y = 0;
        let mut ret_child = 0;

        unsafe {
            xlib::XTranslateCoordinates(
                self.inner,
                window.as_raw(),
                self.default_window().as_raw(),
                x,
                y,
                &mut ret_x,
                &mut ret_y,
                &mut ret_child,
            );
        }

        (ret_x, ret_y)
    }

    // XInternAtom
    pub fn intern_atom<S: AsRef<str>, B: Into<i32>>(
        &self,
        atom_name: S,
        only_if_exists: B,
    ) -> Atom {
        let c_str = CString::new(atom_name.as_ref()).unwrap();
        Atom(unsafe { xlib::XInternAtom(self.inner, c_str.as_ptr(), only_if_exists.into()) })
    }

    // XGetSelectionOwner
    pub fn get_selection_owner(&self, atom: Atom) -> u64 {
        unsafe { xlib::XGetSelectionOwner(self.inner, atom.0) }
    }

    // XSync
    pub fn sync<T: Into<i32>>(&self, discard: T) {
        unsafe {
            xlib::XSync(self.inner, discard.into());
        }
    }

    // XReparentWindow
    pub fn reparent_window(&self, window: &Window, parent: &Window) {
        unsafe {
            xlib::XReparentWindow(self.inner, window.as_raw(), parent.as_raw(), 0, 0);
        }
    }

    // XMapWindow
    pub fn map_window(&self, window: &Window) {
        unsafe {
            xlib::XMapWindow(self.inner, window.as_raw());
        }
    }

    pub fn unmap_window(&self, window: &Window) {
        unsafe {
            xlib::XUnmapWindow(self.inner, window.as_raw());
        }
    }

    // XSelectInput
    pub fn select_input(&self, window: &Window, event_mask: i64) {
        unsafe {
            xlib::XSelectInput(self.inner, window.as_raw(), event_mask);
        }
    }

    // XGrabButton
    pub fn grab_button(&self, window: &Window, button: u32, modifier: Option<u32>) {
        let modifier = modifier.unwrap_or(xlib::AnyModifier);

        unsafe {
            xlib::XGrabButton(
                self.inner,
                button,
                modifier,
                window.as_raw(),
                0,
                (xlib::ButtonPressMask | xlib::ButtonReleaseMask) as u32,
                xlib::GrabModeAsync,
                xlib::GrabModeAsync,
                0,
                0,
            );
        }
    }

    // XGrabKey
    pub fn grab_key(&self, window: &Window, key: char, modifier: Option<u32>) {
        let modifier = modifier.unwrap_or(xlib::AnyModifier);

        unsafe {
            let code = xlib::XKeysymToKeycode(self.inner, key as u64) as i32;
            xlib::XGrabKey(
                self.inner,
                code,
                modifier,
                window.as_raw(),
                0,
                xlib::GrabModeAsync,
                xlib::GrabModeAsync,
            );
        }
    }

    // XWarpPointer
    pub fn warp_pointer(
        &self,
        src_window: Option<&Window>,
        dest_window: Option<&Window>,
        src_rect: Rect,
        dest_x: i32,
        dest_y: i32,
    ) {
        let src_window = match src_window {
            Some(w) => w.as_raw(),
            None => 0,
        };
        let dest_window = match dest_window {
            Some(w) => w.as_raw(),
            None => 0,
        };

        unsafe {
            xlib::XWarpPointer(
                self.inner,
                src_window,
                dest_window,
                src_rect.x,
                src_rect.y,
                src_rect.width,
                src_rect.height,
                dest_x,
                dest_y,
            );
        }
    }

    pub fn query_pointer(&self, window: &Window) -> CursorInfo {
        let mut root_window: XWindow = 0;
        let mut child_window: XWindow = 0;
        let mut root_x = 0;
        let mut root_y = 0;
        let mut child_x = 0;
        let mut child_y = 0;
        let mut mask = 0;

        unsafe {
            xlib::XQueryPointer(
                self.inner,
                window.as_raw(),
                &mut root_window,
                &mut child_window,
                &mut root_x,
                &mut root_y,
                &mut child_x,
                &mut child_y,
                &mut mask,
            );
        }

        CursorInfo {
            parent: Some(self.default_window()),
            child: Some(Window::from_raw(&self, child_window)),
            x: root_x,
            y: root_y,
        }
    }

    pub fn fill_rectangle<T: Into<u64>>(&self, drawable: T, gc: &GContext, rect: &Rect) {
        unsafe {
            xlib::XFillRectangle(
                self.inner,
                drawable.into(),
                gc.as_raw(),
                rect.x,
                rect.y,
                rect.width,
                rect.height,
            );
        }
    }

    pub fn grab_pointer(
        &self,
        window: &Window,
        owner_events: bool,
        event_mask: i64,
        pointer_mode: i32,
        keyboard_mode: i32,
        confine_to: Option<&Window>,
        cursor: u64,
        time: u64,
    ) -> Option<()> {
        let confine_win = match confine_to {
            Some(x) => x.as_raw(),
            None => 0,
        };

        let ret = unsafe {
            xlib::XGrabPointer(
                self.inner,
                window.as_raw(),
                owner_events as i32,
                event_mask as u32,
                pointer_mode,
                keyboard_mode,
                confine_win,
                cursor,
                time,
            )
        };

        if ret == 0{
            return Some(());
        }
        None
    }

    pub fn ungrab_pointer(&self) {
        unsafe {
            xlib::XUngrabPointer(self.inner, 0);
        }
    }

    pub fn get_width(&self, screen: i32) -> i32 {
        unsafe { xlib::XDisplayWidth(self.inner, screen) }
    }

    pub fn get_height(&self, screen: i32) -> i32 {
        unsafe { xlib::XDisplayHeight(self.inner, screen) }
    }

    // XNextEvent
    pub fn next_event(&self) -> Event {
        unsafe {
            let event = libc::malloc(mem::size_of::<xlib::XEvent>()) as XEvent;
            xlib::XNextEvent(self.inner, event);
            Event::from_raw(event)
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            xlib::XCloseDisplay(self.inner);
        }
    }
}
