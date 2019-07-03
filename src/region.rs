use xlib::{
    CursorInfo, Display, EventKind, GCValues, GContext, Rect, SetWindowAttributes, VisualInfo,
    Window,
};

// const SPACE: char = '\u{0020}';
const ESCAPE: char = '\u{FF1B}';
const KEY_Q: char = 'q';
const MOUSE_LEFT: u32 = 1;
const MOUSE_RIGHT: u32 = 3;
const BACKGROUND: u64 = 0x82000000;
const FOREGROUND: u64 = 0; // 0x73284;

pub struct Region<'a> {
    display: &'a Display,
    root: Window,
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
            root,
            window,
            gc,
            active: true,
        }
    }

    /// Registers the key- and button-related events we want to receive,
    /// along with any display changes.
    fn grab_events(&mut self) {
        self.display.grab_key(&self.window, ESCAPE, None);
        self.display.grab_key(&self.window, KEY_Q, None);
        self.display.grab_button(&self.window, MOUSE_LEFT, None);
        self.display.grab_button(&self.window, MOUSE_RIGHT, None);
        self.display
            .select_input(&self.window, xlib::STRUCTURE_NOTIFY_MASK);
    }

    /// Draws the rectangle that represents the highlighted region.
    /// Only used for region capture.
    fn draw_rect(&self, rect: Rect) {
        self.display
            .draw_rectangle(self.window.as_raw(), &self.gc, rect);
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

    fn grab_keyboard(&self) {
        let ret = self
            .window
            .grab_keyboard(false, xlib::GRAB_MODE_ASYNC, xlib::GRAB_MODE_ASYNC);
        println!("{}", ret);
    }

    fn ungrab_keyboard(&self) {
        self.window.ungrab_keyboard();
    }

    /// Helper function for extracting the coordinates from `CursorInfo` into a `Rect`.
    #[inline]
    fn to_rect(start: &CursorInfo, end: &CursorInfo) -> Rect {
        Rect {
            x: i32::min(start.x, end.x),
            y: i32::min(start.y, end.y),
            width: (end.x - start.x).abs() as u32,
            height: (end.y - start.y).abs() as u32,
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

        let mut start_pos = CursorInfo::default();
        let mut end_pos;

        loop {
            let event = self.display.next_event();

            match event.get_kind() {
                // Either the primary or secondary mouse button was pressed
                EventKind::ButtonPress(event) => match event.button {
                    MOUSE_LEFT => {
                        self.grab_pointer();
                        start_pos = self.display.query_pointer(&self.root);
                    }
                    MOUSE_RIGHT => break,
                    _ => (),
                },

                // The left mouse button was released; check the coordinates
                // and return a `Rect` structure containing them.
                EventKind::ButtonRelease(event) => {
                    if event.button == MOUSE_LEFT {
                        self.ungrab_pointer();
                        end_pos = self.display.query_pointer(&self.root);
                        let rect = Self::to_rect(&start_pos, &end_pos);

                        if rect.width == 0 || rect.height == 0 {
                            break;
                        }
                        return Some(rect);
                    }
                }

                // The mouse moved while the primary button was being held.
                // Re-draw the rectangle and update the end position.
                EventKind::Motion(_) => {
                    end_pos = self.display.query_pointer(&self.root);
                    let rect = Self::to_rect(&start_pos, &end_pos);
                    self.window.clear();
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
