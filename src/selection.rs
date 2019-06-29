use std::mem;
use xlib::{Display, EventKind, Rect, Window, WindowAttributes};

pub struct Select<'a>{
    display: &'a Display,
    parent: Window,
    // rect: Rect,
}

impl<'a> Select<'a>{
    pub fn create_parent(display: &'a Display) -> Self{
        let width = display.get_width(0) as u32;
        let height = display.get_height(0) as u32;
        let window = Window::new(&display, Rect{
            x: 0,
            y: 0,
            width,
            height
        });

        Self{
            display,
            parent: window,
        }
    }

    fn reframe(&self, window: &Window){}

    pub fn grab_events(&mut self){
        // escape
        self.display.grab_key(&self.parent, '\u{FF1B}', None);
        // left & right mouse button
        self.display.grab_button(&self.parent, 1, None);
        self.display.grab_button(&self.parent, 3, None);
    }

    pub fn set_attrs(&mut self){
        let mut attr: WindowAttributes = unsafe{ mem::zeroed() };
        let cursor = self.display.create_font_cursor(34);
        attr.background_pixel = 0x8080_8080;
        attr.cursor = cursor;
        self.parent.set_attributes(&mut attr, 0x0002 | 0x4000);
    }

    pub fn show(&self) {
        self.display.map_window(&self.parent);
        self.display.sync(false);

        loop {
            let event = self.display.next_event();
            match event.get_kind() {
                EventKind::ButtonPress(_) => {
                    println!("1");
                }
                EventKind::ButtonRelease(_) => {}
                EventKind::KeyPress(_) => {
                    break;
                }
                _ => (),
            }
        }
    }
}
