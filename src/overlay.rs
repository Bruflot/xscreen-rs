use std::time::Instant;
use xlib::{
    Display, Event, EventKind, GCValues, GContext, Rect, SetWindowAttributes, VisualInfo, Window,
};

const BACKGROUND: u64 = 0; //0x82000000;
const FOREGROUND: u64 = 0x8214_5482; // 0x73284;
const REFRESH_RATE: u128 = 1_000_000_000 / 60;

pub struct Overlay<'a> {
    pub(super) display: &'a Display,
    pub(super) root: Window,
    overlay: Window,
    gc: GContext<'a>,
    time: Instant,
    active: bool,
}

impl<'a> Overlay<'a> {
    pub(super) fn new(display: &'a Display) -> Self {
        let width = display.get_width(0) as u32;
        let height = display.get_height(0) as u32;
        let visual = VisualInfo::from(display, 0, 32, xlib::TRUE_COLOR);
        let root = display.default_window();
        let mut attr = Self::set_attributes(&display, &root, &visual);
        let rect = Rect {
            x: 0,
            y: 0,
            width,
            height,
        };

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
            time: Instant::now(),
            active: true,
        }
    }

    /// Sets the attributes for the overlay window.
    fn set_attributes(
        display: &Display,
        window: &Window,
        visual: &VisualInfo,
    ) -> SetWindowAttributes {
        let mut attr = SetWindowAttributes::default();
        attr.0.background_pixel = BACKGROUND;
        attr.0.border_pixel = 2;
        attr.0.cursor = display.create_font_cursor(34);
        attr.0.colormap =
            display.create_colormap(&window, visual.as_raw().visual, xlib::ALLOC_NONE);
        attr.0.override_redirect = 1;
        attr.0.border_pixel = 16_000_000;
        attr
    }

    /// Grabs the keyboard. Prevents the user from launching or interfering
    /// with other applications.
    fn grab_keyboard(&self) -> Option<()> {
        let ret = self
            .overlay
            .grab_keyboard(true, xlib::GRAB_MODE_SYNC, xlib::GRAB_MODE_ASYNC);

        if ret == 0 {
            return Some(());
        }
        None
    }

    /// Grabs the pointer - this is necessary to receive motion events while
    /// any of the mouse buttons are being held down.
    pub fn grab_pointer(&self, motion: bool) -> Option<()> {
        let mask = if motion {
            xlib::BUTTON1_MOTION_MASK | xlib::BUTTON_RELEASE_MASK | xlib::BUTTON_PRESS_MASK
        } else {
            xlib::BUTTON_PRESS_MASK | xlib::POINTER_MOTION_MASK
        };

        self.display.grab_pointer(
            &self.overlay,
            true,
            mask,
            xlib::GRAB_MODE_ASYNC,
            xlib::GRAB_MODE_ASYNC,
            None,
            0,
            0,
        )
    }

    /// Ungrabs the keyboard.
    fn ungrab_keyboard(&self) {
        self.overlay.ungrab_keyboard();
    }

    /// Ungrabs the pointer.
    // todo: should be self.window.grab_pointer
    fn ungrab_pointer(&self) {
        self.display.ungrab_pointer();
    }

    pub fn clear(&mut self) {
        self.overlay.clear();
    }

    /// Draws the rectangle that represents the highlighted region.
    pub fn draw_rect(&mut self, rect: &Rect) {
        self.display
            .fill_rectangle(self.overlay.as_raw(), &self.gc, &rect);
    }

    pub fn show(&self, motion: bool) {
        self.display.map_window(&self.overlay);
        self.grab_keyboard();
        self.grab_pointer(motion);
    }

    pub fn next_event(&mut self) -> Event {
        loop {
            let event = self.display.next_event();

            match event.get_kind() {
                EventKind::Motion(_) => {
                    if self.time.elapsed().as_nanos() < REFRESH_RATE {
                        continue;
                    }
                }
                EventKind::DestroyWindow(_) => {
                    self.active = false;
                }
                _ => (),
            }

            self.time = Instant::now();
            return event;
        }
    }
}

impl<'a> Drop for Overlay<'a> {
    fn drop(&mut self) {
        if self.active {
            self.ungrab_keyboard();
            self.ungrab_pointer();
            self.overlay.destroy();
            self.display.flush();
        }
    }
}
