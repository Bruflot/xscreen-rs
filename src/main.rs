#![allow(dead_code)]
extern crate image;
extern crate xlib;

mod selection;
mod screenshot;

use selection::Select;
use screenshot::Screenshot;
use xlib::Display;

// ? Window Builder
// todo: parse cmd arguments and act accordingly

fn main() {
    let display = Display::connect(None).expect("Failed to connect to X");
    let select = Select::create_parent(&display);
    // todo: reframe child to parent

    Screenshot::fullscreen(&display).save("test.png").unwrap();
}
