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

fn delay(matches: Option<&str>) {
    if let Some(dur) = matches {
        let secs = dur.parse().expect("Invalid duration");
        let duration = Duration::from_secs(secs);
        thread::sleep(duration);
    }
}

fn filename(matches: Option<&str>) -> PathBuf {
    let time = Local::now()
        .format("Screenshot %Y-%m-%d %H-%M-%S.png")
        .to_string();
    let mut path = match matches {
        Some(p) => PathBuf::from(p),
        None => env::home_dir().unwrap(),
    };
    path.push(time);
    path
}

fn main() {
    let matches = App::new("xscreen")
        .version("0.1")
        .author("Bruflot <git@bruflot.com>")
        .about("Simple X11 screenshot utility")
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
            Arg::with_name("delay")
                .short("d")
                .long("delay")
                .value_name("SECONDS")
                .help("Delay the screenshot by the specified duration")
                .conflicts_with_all(&["window", "region"]),
        )
        .arg(
            Arg::with_name("output")
                .help("Specifies the directory in which the screenshot will be saved. Default is $HOME.")
                .index(1),
        )
        .get_matches();

    delay(matches.value_of("delay"));
    let path = filename(matches.value_of("output"));
    let display = Display::connect(None).expect("Failed to connect to X");
    let root = display.default_window();

    if matches.is_present("window") {
        unimplemented!()
    } else if matches.is_present("region") {
        let region = Region::new(&display).show();
        if let Some(x) = region{
            Screenshot::with_rect(&display, &root, x)
                .save(path)
                .unwrap();
        }
    } else {
        Screenshot::fullscreen(&display).save(path).unwrap();
    }
}
