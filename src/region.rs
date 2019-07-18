use crate::overlay::Overlay;
use xlib::{Display, EventKind, Rect};

const MOUSE_LEFT: u32 = 1;
const MOUSE_RIGHT: u32 = 3;

pub struct Region<'a> {
    overlay: Overlay<'a>,
}

impl<'a> Region<'a> {
    pub fn new(display: &'a Display) -> Self {
        let overlay = Overlay::new(display);
        Self { overlay }
    }

    /// Helper function for turning tuples into `Rect` structures.
    #[inline]
    fn to_rect(start: (i32, i32), end: (i32, i32)) -> Rect {
        Rect {
            x: i32::min(start.0, end.0),
            y: i32::min(start.1, end.1),
            width: (end.0 - start.0).abs() as u32,
            height: (end.1 - start.1).abs() as u32,
        }
    }

    pub fn show(&mut self) -> Option<Rect> {
        self.overlay.show(true);
        let mut start = (0, 0);

        loop {
            let event = self.overlay.next_event();

            match event.get_kind() {
                // Either the primary or secondary mouse button was pressed
                EventKind::ButtonPress(event) => match event.button {
                    MOUSE_LEFT => start = (event.x_root, event.y_root),
                    MOUSE_RIGHT => break,
                    _ => (),
                },

                // The left mouse button was released; check the coordinates
                // and return a `Rect` structure containing them.
                EventKind::ButtonRelease(event) => {
                    if event.button == MOUSE_LEFT {
                        let rect = Self::to_rect(start, (event.x_root, event.y_root));

                        if rect.width == 0 || rect.height == 0 {
                            break;
                        }
                        return Some(rect);
                    }
                }

                // A key event that we monitor was triggered.
                EventKind::KeyPress(event) => match event.keycode {
                    // escape, q
                    9 | 24 => break,

                    // ignore the rest
                    _ => (),
                },

                // The mouse moved while the primary button was being held.
                // Re-draw the rectangle and update the end position.
                EventKind::Motion(event) => {
                    let rect = Self::to_rect(start, (event.x_root, event.y_root));
                    self.overlay.clear();
                    self.overlay.draw_rect(&rect);
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
