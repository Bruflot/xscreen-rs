use std::time::Instant;
use xlib::{Display, EventKind, GCValues, GContext, Rect, SetWindowAttributes, VisualInfo, Window};

const MOUSE_LEFT: u32 = 1;
const MOUSE_RIGHT: u32 = 3;

// alpha-premultiplied colors
const BACKGROUND: u64 = 0; //0x82000000;
const FOREGROUND: u64 = 0x82145482; // 0x73284;

// refresh rate for drawing the highlighted area
const REFRESH_RATE: u128 = 1000/60;

pub struct WindowCapture<'a> {
    display: &'a Display,
    root: Window,
    overlay: Window,
    gc: GContext<'a>,
    active: bool,
}

// TODO: restructure create_parent; should be separate functions.
impl<'a> WindowCapture<'a> {
    /// Sets up the window by calling the relevant member functions.
    pub fn new(display: &'a Display) -> Self {
        let mut window = Self::create_parent(display);
        window.grab_events();
        window
    }

    /// Creates the parent window that covers the entire screen. This is a pop-up window that
    /// will not be manged by your window manager.
    fn create_parent(display: &'a Display) -> Self {
        let width = display.get_width(0) as u32;
        let height = display.get_height(0) as u32;
        let visual = VisualInfo::from(display, 0, 32, xlib::TRUE_COLOR);
        let root = display.default_window();
        let rect = Rect {
            x: 0,
            y: 0,
            width,
            height,
        };

        let mut attr = SetWindowAttributes::default();
        attr.0.background_pixel = BACKGROUND;
        attr.0.border_pixel = 2;
        attr.0.cursor = display.create_font_cursor(34);
        attr.0.colormap = display.create_colormap(&root, visual.as_raw().visual, xlib::ALLOC_NONE);
        attr.0.override_redirect = 1;
        attr.0.border_pixel = 16_000_000;

        let overlay = Window::new(
            &display,
            rect,
            32,
            visual.as_raw().visual,
            xlib::CW_COLORMAP
                | xlib::CW_BORDER_PIXEL
                | xlib::CW_BACK_PIXEL
                | xlib::CW_CURSOR
                | xlib::CW_OVERRIDE_REDIRECT
                | xlib::CW_DONT_PROPAGATE,
            &mut attr,
        );

        let values = GCValues::default();
        let gc = GContext::new(&display, &overlay, 0, values);
        gc.set_foreground(FOREGROUND);

        Self {
            display,
            root,
            overlay,
            gc,
            active: true,
        }
    }

    /// Registers the key- and button-related events we want to receive,
    /// along with any display changes.
    fn grab_events(&mut self) {
        self.display.grab_button(&self.overlay, MOUSE_LEFT, None);
        self.display.grab_button(&self.overlay, MOUSE_RIGHT, None);
        self.display
            .select_input(&self.overlay, xlib::STRUCTURE_NOTIFY_MASK);
    }

    /// Grabs the pointer - this is necessary to receive motion events while
    /// any of the mouse buttons are being held down.
    fn grab_pointer(&self) -> Option<()> {
        let ret = self.display.grab_pointer(
            &self.overlay,
            true,
            xlib::BUTTON_PRESS_MASK | xlib::POINTER_MOTION_MASK,
            xlib::GRAB_MODE_ASYNC,
            xlib::GRAB_MODE_ASYNC,
            None,
            0,
            0,
        );

        if ret == 0 {
            return Some(());
        }
        None
    }

    /// Ungrabs the pointer.
    fn ungrab_pointer(&self) {
        self.display.ungrab_pointer();
    }

    /// Grabs the keyboard. Prevents the user from launching or interfering
    /// with other applications intentionally or unintentionally.
    fn grab_keyboard(&self) -> Option<()> {
        let ret = self
            .overlay
            .grab_keyboard(true, xlib::GRAB_MODE_ASYNC, xlib::GRAB_MODE_ASYNC);

        if ret == 0 {
            return Some(());
        }
        None
    }

    fn ungrab_keyboard(&self) {
        self.overlay.ungrab_keyboard();
    }

    // ? Not sure if "_NET_CURRENT_DESKTOP" can have several values
    // ? for multi-monitor setups
    // ? screens?
    /// Returns the current workspace/virtual desktop.
    fn get_workspace(&self) -> u8 {
        let atom = self.display.intern_atom("_NET_CURRENT_DESKTOP", false);
        let mut actual_type = 0;
        let mut format = 0;
        let mut length = 0;
        let mut bytes_after_return = 0;
        let mut ptr = std::ptr::null_mut();
        unsafe {
            x11::xlib::XGetWindowProperty(
                self.display.as_raw(),
                self.root.as_raw(),
                atom.0,
                0, // offset
                1, // 32-bit multiples of data to be read
                0, // delete
                0, // req type
                &mut actual_type,
                &mut format,
                &mut length,
                &mut bytes_after_return,
                &mut ptr,
            );

            // if ptr.is_null(){ }
            let workspace = *ptr;
            x11::xlib::XFree(ptr as *mut std::ffi::c_void);
            workspace as u8
        }
    }

    /// Checks if the given window is "visible," i.e. whether it is shown
    /// on the active workspace or not.
    fn is_visible(&self, window: &Window) -> Option<u8> {
        let atom = self.display.intern_atom("_NET_WM_DESKTOP", false);
        let mut actual_type = 0;
        let mut format = 0;
        let mut bytes_after_return = 0;
        let mut ptr = std::ptr::null_mut();

        unsafe {
            let mut length = std::mem::uninitialized();

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

            let val = if length != 0 { Some(*ptr) } else { None };

            x11::xlib::XFree(ptr as *mut std::ffi::c_void);
            // println!("{}: {}", window.as_raw(), length);
            return val;
        }
    }

    /// Returns a list of all children windows that has the _NET_WM_DESKTOP
    /// attribute. Windows that do not have this attribute are not managed
    /// by the window manager, or is often a container of some sort owned
    /// by the window manager. The attribute is necessary to determine
    /// whether the window is visible or not.
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
        let parents = self.get_children(&self.root);

        for w in parents {
            for c in self.get_children(&w) {
                windows.push(c);
            }
            windows.push(w);
        }

        let workspace = self.get_workspace();

        windows
            .into_iter()
            .filter(|w| self.is_visible(&w) == Some(workspace))
            .collect()
    }

    /// Draws the rectangle that represents the highlighted region.
    /// Only used for region capture.
    fn draw_rect(&self, rect: &Rect) {
        self.display
            .draw_rectangle(self.overlay.as_raw(), &self.gc, &rect);
    }

    /// This function is responsible for drawing both the parent window and the
    /// rectangle that highlights the selected window. It also handles the main
    /// event loop of the UI.
    /// It loops through all open windows (from bottom to top) and compares the
    /// coordinates of said windows to that of the cursor.
    /// Returns the `Window` structure of the selected window..
    /// May return `None` if the capture was aborted.
    // ? Focus event? May generate if we don't own the mouse input events
    pub fn show(&mut self) -> Option<Window> {
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
        self.display.map_window(&self.overlay);
        self.grab_keyboard();
        self.grab_pointer();
        let mut window = self.display.default_window();
        let mut time = Instant::now();

        loop {
            let event = self.display.next_event();

            match event.get_kind() {
                // Cursor moved; check its position and redraw the rect if necessary.
                EventKind::Motion(cursor) => {
                    if time.elapsed().as_millis() > REFRESH_RATE{
                        self.overlay.clear();

                        for (w, r) in &rects {
                            if cursor.x > r.x
                                && cursor.x < (r.x + r.width as i32)
                                && cursor.y > r.y
                                && cursor.y < (r.y + r.height as i32)
                            {
                                window = **w;
                                self.draw_rect(&r);
                                break;
                            }
                        }

                        time = Instant::now();
                    }                    
                }

                // A mouse button was clicked - either abort or return a Rect.
                EventKind::ButtonPress(event) => {
                    if event.button == MOUSE_LEFT {
                        return Some(window);
                    }
                }

                // A key event that we monitor was triggered.
                EventKind::KeyPress(_) => break,

                // The window was destroyed by external means.
                EventKind::DestroyWindow(_) => {
                    self.active = false;
                    break;
                }
                _ => (),
            }
        }

        None
    }
}

/// Destroys the window when dropped unless it's already destroyed.
impl<'a> Drop for WindowCapture<'a> {
    fn drop(&mut self) {
        if self.active {
            self.ungrab_keyboard();
            self.ungrab_pointer();
            self.overlay.destroy();
            self.display.flush();
        }
    }
}
