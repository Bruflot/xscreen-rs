use std::io;
use std::{error, fmt};
use xlib;

#[derive(Debug)]
pub enum Error {
    ConnectionError,
    CompositorError,
    ImageError,
    InvalidRect,
    InvalidPath,
    Cancelled,
    WindowDestroyed,
    IOError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match &self {
            ConnectionError => write!(f, "ConnectionError"),
            CompositorError => write!(f, "CompositorError"),
            ImageError => write!(f, "ImageError"),
            InvalidRect => write!(f, "InvalidRect"),
            InvalidPath => write!(f, "InvalidPath"),
            Cancelled => write!(f, "Aborted"),
            WindowDestroyed => write!(f, "WindowDestroyed"),
            IOError(_) => write!(f, "IOError"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use Error::*;

        match self {
            ConnectionError => "Failed to connect to X",
            CompositorError => "A composite manager is required",
            ImageError => "Unable to get frame buffer from X",
            InvalidRect => "Invalid region: width or height cannot be 0px",
            InvalidPath => "Invalid path",
            Cancelled => "Operation aborted by user",
            WindowDestroyed => "Window destroyed by external means",
            IOError(e) => e,
        }
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IOError(e.to_string())
    }
}

impl From<xlib::XError> for Error {
    fn from(_: xlib::XError) -> Self {
        Error::ConnectionError
    }
}
