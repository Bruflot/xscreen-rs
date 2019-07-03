// Although std::env::home_dir() is deprecated, it functions as you'd expect
// for Unix environments. AFAIK it was deprecated because the functionality
// in Windows was not what you'd expect.

#![allow(deprecated)]
extern crate chrono;
extern crate clap;
extern crate image;
extern crate xlib;

mod region;
mod screenshot;

use chrono::Local;
use clap::{App, Arg};
use region::Region;
use screenshot::Screenshot;
use std::path::PathBuf;
use std::time::Duration;
use std::{env, thread};
use xlib::Display;

/// Checks if a compositor is present
fn has_compositor(display: &Display) -> bool {
    unsafe {
        let c_str = std::ffi::CString::new("_NET_WM_CM_S0").unwrap();
        let test = x11::xlib::XInternAtom(display.as_raw(), c_str.as_ptr(), 0);
        x11::xlib::XGetSelectionOwner(display.as_raw(), test) != 0
    }
}

/// Sleeps for the specified duration before continuing execution
fn delay(matches: Option<&str>) {
    if let Some(dur) = matches {
        let secs = dur.parse().expect("Invalid duration");
        let duration = Duration::from_secs(secs);
        thread::sleep(duration);
    }
}

/// Parses the given directory and generates the filename of the screenshot
fn filename(matches: Option<&str>) -> Option<PathBuf> {
    let time = Local::now()
        .format("Screenshot %Y-%m-%d %H-%M-%S.png")
        .to_string();
    let mut path = match matches {
        Some(p) => PathBuf::from(p),
        None => env::home_dir()?,
    };
    path.push(time);
    Some(path)
}

/// Display the region capture window
fn region(display: &Display, path: PathBuf) {
    if !has_compositor(display) {
        panic!("A compositor is required for region capture!");
    }
    let region = Region::new(&display).show();
    if let Some(x) = region {
        Screenshot::with_rect(&display, &display.default_window(), x)
            .save(path)
            .unwrap();
    }
}

fn main() {
    let matches = App::new("xscreen")
        .version("0.1")
        .author("Bruflot <git@bruflot.com>")
        .about("Simple X11 screenshot utility")
        .arg(
            Arg::with_name("clipboard")
                .short("c")
                .long("clipboard")
                .help("Copies the image directly to your clipboard")
        )
        .arg(
            Arg::with_name("delay")
                .short("d")
                .long("delay")
                .value_name("SECONDS")
                .help("Delay the screenshot by the specified duration")
                .conflicts_with_all(&["window", "region"]),
        )
        .arg(
            Arg::with_name("region")
                .short("r")
                .long("region")
                .help("Captures a region of the screen")
                .conflicts_with("window"),
        )
        .arg(
            Arg::with_name("window")
                .short("w")
                .long("window")
                .help("Captures a specific window; can be toggled by launching region capture and hitting space")
                .conflicts_with("region"),
        )
        .arg(
            Arg::with_name("output")
                .help("Specifies the directory in which the screenshot will be saved. Default is $HOME.")
                .index(1),
        )
        .get_matches();

    delay(matches.value_of("delay"));
    let path = filename(matches.value_of("output")).expect("Invalid file path");
    let display = Display::connect(None).expect("Failed to connect to X");

    if matches.is_present("window") {
        unimplemented!()
    } else if matches.is_present("region") {
        region(&display, path);
    } else {
        Screenshot::fullscreen(&display).save(path).unwrap();
    }
}
