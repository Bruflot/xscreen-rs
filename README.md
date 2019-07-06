# xscreen-rs
A simple and fast screenshot utility for X11 that aims to emulate macOS' screenshot tool.   
Supports region, window, and fullscreen capture.

A compositor is required for region and window capture for the time being. Colors of the overlay 
can be edited in `src/region.rs`.

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

## Todo
- [ ] Fix visible overlay in screenshots
- [ ] Support systems without compositors
- [ ] Copy the produced image directly to the clipboard
- [ ] Multiple screens
- [ ] Proper error handling
- [ ] Selection border
- [ ] Make screenshots appear in 'recent'