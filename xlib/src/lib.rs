#![allow(dead_code)]
extern crate x11;

mod consts;
mod display;
mod events;
mod gcontext;
mod image;
mod window;

use x11::xlib;
pub(crate) type XDisplay = *mut xlib::Display;
pub(crate) type XEvent = *mut xlib::XEvent;
pub(crate) type XGC = xlib::GC;
pub(crate) type XGCValues = *mut xlib::XGCValues;
pub(crate) type XImage = *mut xlib::XImage;
pub(crate) type XWindow = xlib::Window;

pub use consts::*;
pub use display::{CursorInfo, Display};
pub use events::{Event, EventKind};
pub use gcontext::{GCValues, GContext};
pub use image::{Image, Visual, VisualInfo};
pub use window::{SetWindowAttributes, Window};
pub type XResult<T> = std::result::Result<T, XError>;

#[derive(Debug)]
pub enum XError {
    BadAlloc,
    BadMatch,
    BadValue,
    BadWindow,
    ConnectionError,
}

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Default for Rect {
    fn default() -> Self {
        Rect {
            x: 0,
            y: 0,
            width: 250,
            height: 250,
        }
    }
}
