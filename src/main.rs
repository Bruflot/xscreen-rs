#![allow(dead_code)]
extern crate chrono;
extern crate clap;
extern crate image;
extern crate xlib;

mod screenshot;
mod selection;

use chrono::Local;
use clap::{App, Arg};
use screenshot::Screenshot;
use selection::Select;
use std::env;
use std::path::{Path, PathBuf};
use xlib::Display;

// ? Window Builder
// ? window PID
// todo: parse cmd arguments and act accordingly

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
                .help("Captures a specific window")
                .conflicts_with("region"),
        )
        .arg(
            Arg::with_name("output")
                .help("Specifies the directory in which the screenshot will be saved")
                .index(1),
        )
        .get_matches();

    let time = Local::now()
        .format("Screenshot %Y-%m-%d %H-%M-%S.png")
        .to_string();
    let mut path = match matches.value_of("output") {
        Some(p) => PathBuf::from(p),
        None => env::home_dir().unwrap(),
    };
    path.push(time);

    let display = Display::connect(None).expect("Failed to connect to X");
    let select = Select::create_parent(&display);
    // todo: reframe child to parent

    Screenshot::fullscreen(&display).save(path).unwrap();
}
