use crate::overlay::Overlay;
use xlib::{Display, EventKind, Rect, Window};

const MOUSE_LEFT: u32 = 1;

pub struct WindowCapture<'a> {
    display: &'a Display,
    overlay: Overlay<'a>,
}

impl<'a> WindowCapture<'a> {
    pub fn new(display: &'a Display) -> Self {
        let overlay = Overlay::new(display);
        Self { display, overlay }
    }

    /// Checks if the given window is visible, i.e. whether it is shown on
    /// any of the monitors. Returns `None` if the window is either
    /// completely covered by another window, or if the window is visible
    /// on an underlying workspace/virtual desktop.
    // ? other WMs may have a different ordering of the wm_state fields(?)
    fn is_visible(&self, window: &Window) -> bool {
        let atom = self.display.intern_atom("WM_STATE", false);
        let mut actual_type = 0;
        let mut format = 0;
        let mut length = 0;
        let mut bytes_after_return = 0;
        let mut ptr = std::ptr::null_mut();
        unsafe {
            x11::xlib::XGetWindowProperty(
                self.display.as_raw(),
                window.as_raw(),
                atom.0,
                0,    // offset
                1024, // 32-bit multiples of data to be read
                0,    // delete
                0,    // req type
                &mut actual_type,
                &mut format,
                &mut length,
                &mut bytes_after_return,
                &mut ptr,
            );

            let slice = std::slice::from_raw_parts(ptr as *mut i8, length as usize);
            let visible = if length > 0 { slice[0] == 1 } else { false };
            x11::xlib::XFree(ptr as *mut std::ffi::c_void);
            visible
        }
    }

    /// Returns a list of all children windows that are visible.
    /// Windows that do not have this attribute are not managed by the wm, 
    /// or is often a container of some sort owned by the window manager. 
    fn get_children(&self, window: &Window) -> Vec<Window> {
        let mut root = 0;
        let mut parent = 0;
        let mut ptr = std::ptr::null_mut();
        let mut length = 0;

        let array = unsafe {
            x11::xlib::XQueryTree(
                self.display.as_raw(),
                window.as_raw(),
                &mut root,
                &mut parent,
                &mut ptr,
                &mut length,
            );

            std::slice::from_raw_parts(ptr, length as usize)
        };

        let windows: Vec<Window> = array
            .iter()
            .rev()
            .map(|w| Window::from_raw(self.display, *w))
            .collect();

        unsafe {
            x11::xlib::XFree(ptr as *mut std::ffi::c_void);
        }

        windows
    }

    /// Scans through the first two depth layers of all windows (i.e. parents and
    /// their respective children). Filters out windows that are not visible on
    /// the current workspace/virtual desktop.
    fn get_all_windows(&self) -> Vec<Window> {
        let mut windows: Vec<Window> = Vec::new();
        let parents = self.get_children(&self.overlay.root);

        for w in parents {
            for c in self.get_children(&w) {
                if self.is_visible(&c) {
                    windows.push(c);
                }
            }
        }

        windows
    }

    /// This function is responsible for drawing the rectangle that highlights
    /// the selected window. It also handles the main event loop of the UI.
    /// It loops through all open windows (from bottom to top) and compares the
    /// coordinates of said windows to that of the cursor.
    /// Returns the `Window` structure of the selected window.
    /// May return `None` if the capture was aborted.
    // ? Focus event? May generate if we don't own the mouse input events
    pub fn show(&mut self) -> Option<Window> {
        self.overlay.show(false);
        let mut window = self.overlay.root;
        let windows = self.get_all_windows();
        let rects: Vec<_> = windows
            .iter()
            .map(|w| {
                let rect = w.get_rect();
                let (x, y) = self.display.translate_coordinates(&w, 0, 0);
                (
                    w,
                    Rect {
                        width: rect.width,
                        height: rect.height,
                        x,
                        y,
                    },
                )
            })
            .collect();

        loop {
            let event = self.overlay.next_event();

            match event.get_kind() {
                // Either the primary or secondary mouse button was pressed
                EventKind::ButtonPress(event) => {
                    if event.button == MOUSE_LEFT {
                        return Some(window);
                    }
                }

                // A key event that we monitor was triggered.
                EventKind::KeyPress(event) => match event.keycode {
                    // escape, q
                    9 | 24 => break,

                    // ignore the rest
                    _ => (),
                },

                // Cursor moved; check its position and redraw the overlay.
                EventKind::Motion(cursor) => {
                    for (w, r) in &rects {
                        if cursor.x > r.x
                            && cursor.x < (r.x + r.width as i32)
                            && cursor.y > r.y
                            && cursor.y < (r.y + r.height as i32)
                        {
                            window = **w;
                            self.overlay.clear();
                            self.overlay.draw_rect(&r);
                            break;
                        }
                    }
                }

                // The window was destroyed by external means.
                EventKind::DestroyWindow(_) => {
                    break;
                }
                _ => (),
            }
        }

        None
    }
}
