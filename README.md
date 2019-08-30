# `xscreen-rs`

[![Build Status](https://travis-ci.com/Bruflot/xscreen-rs.svg?token=cwgWL8QUNVpLfj8cDgps&branch=master)](https://travis-ci.com/Bruflot/xscreen-rs)
[![](https://tokei.rs/b1/github/bruflot/xscreen-rs)](https://tokei.rs/b1/github/bruflot/xscreen-rs)

> A simple and fast screenshot utility for X11 that aims to emulate macOS' screenshot tool.   
> Supports region, window, and fullscreen capture.

A compositor is required. Colors of the overlay can be edited in `src/overlay.rs`.

**Note:** Be sure to compile the project with the `--release` flag, as the speedup is exponential!

## Usage
Global keybinds must be handled by your window manager.  
Window capture can be toggled from region capture by hitting space.  

```
xscreen [FLAGS] [OPTIONS] <output>

FLAGS:
    -h, --help         Prints help information
    -r, --region       Captures a region of the screen
    -V, --version      Prints version information
    -w, --window       Captures a specific window

OPTIONS:
    -d, --delay <SECONDS>    Delay the screenshot by the specified duration

ARGS:
    <output>    Specifies the directory in which the screenshot will be saved. Default is $HOME.
```

## Clipboard
Copying the image to your clipboard can be done through e.g. `xclip`:
```
$ xscreen /tmp/xscreen.png
$ xclip -selection clipboard -t image/png -i /tmp/xscreen.png
```
