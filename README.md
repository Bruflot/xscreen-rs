# xscreen-rs
A simple and fast screenshot utility written in Rust. Uses `xlib` and `image-png`.  
Supports region and fullscreen capture. Window capture is a WIP.

Note: The overlay that darkens the background and highlights the selected region only works if you
are using a window compositor; otherwise, only an outline will be drawn that highlights the
selected area. The colors can be edited in `src/region.rs`.

## Usage

Global keybinds must be handled by your window manager.  
Window capture can be toggled from region capture by hitting space.

```
xscreen [FLAGS] [OPTIONS] <output>

FLAGS:
    -h, --help       Prints help information
    -r, --region     Captures a region of the screen
    -V, --version    Prints version information
    -w, --window     Captures a specific window

OPTIONS:
    -d, --delay <SECONDS>    Delay the screenshot by the specified duration

ARGS:
    <output>    Specifies the directory in which the screenshot will be saved. Default is $HOME.
```

## Todo
- [ ] Window capture
- [ ] Copy the produced image directly to the clipboard
