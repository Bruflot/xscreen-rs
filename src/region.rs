use std::mem;
use xlib::{Display, EventKind, GCValues, GContext, Rect, Window, WindowAttributes};

const KEY_Q: char = 'q';
const ESCAPE: char = '\u{FF1B}';
const MOUSE_LEFT: u32 = 1;
const MOUSE_RIGHT: u32 = 3;

pub struct Region<'a> {
    display: &'a Display,
    parent: Window,
    active: bool,
}

impl<'a> Region<'a> {
    /// Sets up the window by calling the relevant member functions.
    pub fn new(display: &'a Display) -> Self {
        let mut parent = Self::create_parent(display);
        parent.grab_events();
        parent.set_attrs();
        parent
    }

    /// Create the parent window that covers the entire screen.
    /// If the `overlay` feature is enabled, the background color will be set
    /// to a translucent color.
    fn create_parent(display: &'a Display) -> Self {
        let width = display.get_width(0) as u32;
        let height = display.get_height(0) as u32;
        let rect = Rect {
            x: 0,
            y: 0,
            width,
            height,
        };
        let window = Window::new(&display, rect);

        Self {
            display,
            parent: window,
            active: true,
        }
    }

    /// Register the key- and button-related events we want to receive,
    /// along with any display changes.
    fn grab_events(&mut self) {
        self.display.grab_key(&self.parent, ESCAPE, None);
        self.display.grab_key(&self.parent, KEY_Q, None);
        self.display.grab_button(&self.parent, MOUSE_LEFT, None);
        self.display.grab_button(&self.parent, MOUSE_RIGHT, None);
        self.display
            .select_input(&self.parent, xlib::STRUCTURE_NOTIFY_MASK);
    }

    /// Set the attributes of the parent window. This includes background
    /// color and cursor.
    fn set_attrs(&mut self) {
        let mut attr: WindowAttributes = unsafe { mem::zeroed() };
        let cursor = self.display.create_font_cursor(34);
        attr.background_pixel = 0x8080_8080;
        attr.cursor = cursor;
        self.parent.set_attributes(&mut attr, 0x0002 | 0x4000);
    }

    /// Draw the rectangle that represents the highlighted region.
    /// Only used for region capture.
    fn draw_rect(&self, rect: Rect) {
        let values = GCValues::default();
        let gc = GContext::new(self.display, self.parent.as_raw(), 0, values);
        self.display.draw_rectangle(self.parent.as_raw(), gc, rect);
    }

    /// Grab the pointer - this is necessary to receive motion events while
    /// any of the mouse buttons are being held down.
    fn grab_pointer(&self) {
        self.display.grab_pointer(
            &self.parent,
            true,
            (xlib::BUTTON1_MOTION_MASK | xlib::BUTTON_RELEASE_MASK) as u32,
            None,
            0,
            0,
        );
    }

    /// Ungrabs the pointer.
    fn ungrab_pointer(&self) {
        self.display.ungrab_pointer(0);
    }

    /// This function is responsible for drawing both the parent window and the
    /// rectangle that highlights the masked region. It also handles the main
    /// event loop of the UI.
    /// Returns a `Rect` which holds the coordinates of the masked region.
    /// May return `None` if the region capture was cancelled or a region of
    /// 0x0px was selected.
    pub fn show(&mut self) -> Option<Rect> {
        self.display.map_window(&self.parent);
        self.display.sync(false);

        let mut start_pos = (0, 0);
        let mut end_pos = (0, 0);

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
                        let width = (end_pos.0 - start_pos.0).abs() as u32;
                        let height = (end_pos.1 - start_pos.1).abs() as u32;

                        if width == 0 || height == 0 {
                            break;
                        }

                        return Some(Rect {
                            x: i32::min(start_pos.0, end_pos.0),
                            y: i32::min(start_pos.1, end_pos.1),
                            width,
                            height,
                        });
                    }
                }

                // The mouse moved while the primary button was being held.
                // Re-draw the rectangle and update the end position.
                EventKind::Motion(_) => {
                    end_pos = self.display.query_pointer(&self.display.default_window());
                    self.draw_rect(Rect {
                        x: i32::min(start_pos.0, end_pos.0),
                        y: i32::min(start_pos.1, end_pos.1),
                        width: (end_pos.0 - start_pos.0).abs() as u32,
                        height: (end_pos.1 - start_pos.1).abs() as u32,
                    });
                }

                // A Keybind for closing the application was triggered.
                EventKind::KeyPress(_) => {
                    break;
                }

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
            self.parent.destroy();
        }
    }
}
