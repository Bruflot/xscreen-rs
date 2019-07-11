// Although std::env::home_dir() is deprecated, it functions as you'd expect
// for Unix environments. AFAIK it was deprecated because the functionality
// in Windows was not what you'd expect.

// The `windowcapture` and `region` modules will soon be rewritten as they
// share a lot of their functionality.

#![allow(deprecated)]
extern crate chrono;
extern crate clap;
extern crate image;
extern crate xlib;

mod region;
mod screenshot;
mod windowcapture;

use chrono::Local;
use clap::{App, Arg};
use region::Region;
use screenshot::Screenshot;
use std::path::PathBuf;
use std::time::Duration;
use std::{env, io, thread};
use windowcapture::WindowCapture;
use xlib::Display;

/// Checks if a compositor is present
fn has_compositor(display: &Display) -> bool {
    let atom = display.intern_atom("_NET_WM_CM_S0", false);
    display.get_selection_owner(atom) != 0
}

/// Sleeps for the specified duration before resuming execution
fn delay(matches: Option<&str>) {
    if let Some(dur) = matches {
        let secs = dur.parse().expect("Invalid duration");
        let duration = Duration::from_secs(secs);
        thread::sleep(duration);
    }
}

/// Determines whether the given string is a valid filename or filepath.
/// Generates a filename if necessary.
fn filename(matches: Option<&str>) -> Option<PathBuf> {
    let mut path = match matches {
        Some(p) => PathBuf::from(p),
        None => env::home_dir()?,
    };

    if path.is_dir() {
        let time = Local::now()
            .format("Screenshot %Y-%m-%d %H-%M-%S.png")
            .to_string();
        path.push(time);
    }

    path = path.canonicalize().unwrap_or(path);

    Some(path)
}

fn main() {
    let matches = App::new("xscreen")
        .version("0.2")
        .author("Bruflot <git@bruflot.com>")
        .about("Simple X11 screenshot utility")
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

    let result = || -> Option<_> {
        let path = filename(matches.value_of("output"))?;
        let display = Display::connect(None).ok()?;

        if !has_compositor(&display){
            return None;
        };

        let screenshot = if matches.is_present("window") {
            let window = WindowCapture::new(&display).show()?;
            Screenshot::window(&display, &window)
        } else if matches.is_present("region") {
            let rect = Region::new(&display).show()?;
            Screenshot::with_rect(&display, &display.default_window(), rect)
        } else {
            Screenshot::fullscreen(&display)
        };

        thread::sleep_ms(2);
        screenshot?.save(&path).ok()?;
        Some(path)
    };

    match result() {
        Some(path) => println!(
            "   \x1b[1;32mSuccess:\x1b[0m Saved to {}",
            path.to_string_lossy()
        ),
        None => println!("   \x1b[1;31mError:\x1b[0m {}", io::Error::last_os_error()),
    }
}
