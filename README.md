# xscreen-rs
A simple screenshot utility written in Rust. Uses `xlib` and `image-png`.  
Supports region, window, and fullscreen capture.

For those using a compositor, there is a feature for adding a translucent
overlay that darkens the unmasked region in case of region capture. This feature
must be enabled by compiling with the `--features overlay` flag.

```
cargo build --features overlay
```

## Usage

Any global keybinds must be handled/configured by your window manager.

```
xscreen [FLAGS] [OPTIONS] <output>

FLAGS:
    -h, --help       Prints help information
    -r, --region     Captures a region of the screen
    -V, --version    Prints version information
    -w, --window     Captures a specific window; can be toggled by launching region capture and hitting space

OPTIONS:
    -d, --delay <SECONDS>    Delay the screenshot by the specified duration

ARGS:
    <output>    Specifies the directory in which the screenshot will be saved. Default is $HOME.
```


## Todo
- [ ] Copy the produced image directly to the clipboard