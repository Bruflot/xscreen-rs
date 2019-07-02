use xlib::{Display, EventKind, GCValues, GContext, Rect, SetWindowAttributes, VisualInfo, Window};

// const SPACE: char = '\u{0020}';
const ESCAPE: char = '\u{FF1B}';
const KEY_Q: char = 'q';
const MOUSE_LEFT: u32 = 1;
const MOUSE_RIGHT: u32 = 3;
const BACKGROUND: u64 = 0x6600_0000;
const FOREGROUND: u64 = 0x73284;

pub struct Region<'a> {
    display: &'a Display,
    window: Window,
    gc: GContext<'a>,
    active: bool,
}

// TODO: restructure create_parent; should be separate functions.
impl<'a> Region<'a> {
    /// Sets up the window by calling the relevant member functions.
    pub fn new(display: &'a Display) -> Self {
        let mut window = Self::create_parent(display);
        window.grab_events();
        window
    }

    // fn create_compositor_window(display: &'a Display) -> Self{
    // }

    /// Creates the parent window that covers the entire screen. This is a pop-up window that
    /// will not be manged by your window manager.
    fn create_parent(display: &'a Display) -> Self {
        let width = display.get_width(0) as u32;
        let height = display.get_height(0) as u32;
        let visual = VisualInfo::from(display, 0, 32, xlib::TRUE_COLOR);
        let rect = Rect {
            x: 0,
            y: 0,
            width,
            height,
        };

        let mut attr = SetWindowAttributes::default();
        attr.0.background_pixel = BACKGROUND;
        attr.0.border_pixel = 0;
        attr.0.cursor = display.create_font_cursor(34);
        attr.0.colormap = display.create_colormap(
            &display.default_window(),
            visual.as_raw().visual,
            xlib::ALLOC_NONE,
        );
        attr.0.override_redirect = 1;
        attr.0.border_pixel = 16_000_000;

        let window = Window::new(
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
        let gc = GContext::new(&display, &window, 0, values);
        gc.set_foreground(FOREGROUND);

        Self {
            display,
            window,
            gc,
            active: true,
        }
    }

    /// Checks if a compositor is present.
    pub fn has_compositor(&self) -> bool {
        unsafe {
            let c_str = std::ffi::CString::new("_NET_WM_CM_S0").unwrap();
            let test = x11::xlib::XInternAtom(self.display.as_raw(), c_str.as_ptr(), 0);
            x11::xlib::XGetSelectionOwner(self.display.as_raw(), test) != 0
        }
    }

    /// Registers the key- and button-related events we want to receive,
    /// along with any display changes.
    fn grab_events(&mut self) {
        self.display.grab_button(&self.window, MOUSE_LEFT, None);
        self.display.grab_button(&self.window, MOUSE_RIGHT, None);
        self.display
            .select_input(&self.window, xlib::STRUCTURE_NOTIFY_MASK);
    }

    /// Draws the rectangle that represents the highlighted region.
    /// Only used for region capture.
    fn draw_rect(&self, rect: Rect) {
        self.display.draw_rectangle(self.window.as_raw(), &self.gc, rect);
    }

    /// Grabs the pointer - this is necessary to receive motion events while
    /// any of the mouse buttons are being held down.
    fn grab_pointer(&self) {
        self.display.grab_pointer(
            &self.window,
            true,
            xlib::BUTTON1_MOTION_MASK | xlib::BUTTON_RELEASE_MASK,
            xlib::GRAB_MODE_ASYNC,
            xlib::GRAB_MODE_ASYNC,
            None,
            0,
            0,
        );
    }

    /// Ungrabs the pointer.
    fn ungrab_pointer(&self) {
        self.display.ungrab_pointer();
    }

    fn grab_keyboard(&self){
        self.window.grab_keyboard(false, xlib::GRAB_MODE_ASYNC, xlib::GRAB_MODE_ASYNC);
    }

    fn ungrab_keyboard(&self){
        self.window.ungrab_keyboard();
    }

    /// Helper function for creating `Rect`s from pairs of tuples.
    #[inline]
    fn to_rect(start: (i32, i32), end: (i32, i32)) -> Rect {
        Rect {
            x: i32::min(start.0, end.0), 
            y: i32::min(start.1, end.1),
            width: (end.0 - start.0).abs() as u32,
            height: (end.1 - start.1).abs() as u32,
        }
    }

    /// This function is responsible for drawing both the parent window and the
    /// rectangle that highlights the masked region. It also handles the main
    /// event loop of the UI.
    /// Returns a `Rect` which holds the coordinates of the masked region.
    /// May return `None` if the region capture was cancelled or a region of
    /// 0x0px was selected.
    pub fn show(&mut self) -> Option<Rect> {
        self.display.map_window(&self.window);
        self.grab_keyboard();
        self.display.sync(false);


        let mut start_pos = (0, 0);
        let mut end_pos;

        loop {
            let event = self.display.next_event();

            match event.get_kind() {
                // Either the primary or secondary mouse button was pressed
                EventKind::ButtonPress(event) => match event.button {
                    MOUSE_LEFT => {
                        self.grab_pointer();
                        start_pos = self.display.query_pointer(&self.display.default_window());
                    }
                    MOUSE_RIGHT => break,
                    _ => (),
                },

                // The left mouse button was released; check the coordinates
                // and return a `Rect` structure containing them.
                EventKind::ButtonRelease(event) => {
                    if event.button == MOUSE_LEFT {
                        self.ungrab_pointer();
                        end_pos = self.display.query_pointer(&self.display.default_window());
                        let rect = Self::to_rect(start_pos, end_pos);

                        if rect.width == 0 || rect.height == 0 {
                            break;
                        }
                        return Some(rect);
                    }
                }

                // The mouse moved while the primary button was being held.
                // Re-draw the rectangle and update the end position.
                EventKind::Motion(_) => {
                    end_pos = self.display.query_pointer(&self.display.default_window());
                    self.window.clear();
                    let rect = Self::to_rect(start_pos, end_pos);
                    self.draw_rect(rect);
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
impl<'a> Drop for Region<'a> {
    fn drop(&mut self) {
        if self.active {
            self.ungrab_keyboard();
            self.ungrab_pointer();
            self.window.destroy();
        }
    }
}
