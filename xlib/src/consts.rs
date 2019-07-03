// events
pub const KEY_PRESS_MASK: i64 = 0x0000_0001;
pub const KEY_PRESS_RELEASE: i64 = 0x0000_0002;
pub const BUTTON_PRESS_MASK: i64 = 0x0000_0004;
pub const BUTTON_RELEASE_MASK: i64 = 0x0000_0008;
pub const STRUCTURE_NOTIFY_MASK: i64 = 0x0002_0000;
pub const SUBSTRUCTURE_NOTIFY_MASK: i64 = 0x0008_0000;
pub const SUBSTRUCTURE_REDIRECT_MASK: i64 = 0x0010_0000;
pub const POINTER_MOTION_MASK: i64 = 0x0000_0040;
pub const BUTTON_MOTION_MASK: i64 = 0x0000_2000;
pub const BUTTON1_MOTION_MASK: i64 = 0x0000_0100;

// grab modes
pub const GRAB_MODE_SYNC: i32 = 0;
pub const GRAB_MODE_ASYNC: i32 = 1;

// image formats
pub const XY_BITMAP: i32 = 0;
pub const XY_PIXMAP: i32 = 1;
pub const Z_PIXMAP: i32 = 2;

// allocate colormap
pub const ALLOC_NONE: i32 = 0;
pub const ALLOC_ALL: i32 = 1;

// window classes
pub const INPUT_OUTPUT: u32 = 1;
pub const INPUT_ONLY: u64 = 2;

// window attributes
pub const CW_BACK_PIXMAP: u64 = 0x0001;
pub const CW_BACK_PIXEL: u64 = 0x0002;
pub const CW_BORDER_PIXMAP: u64 = 0x0004;
pub const CW_BORDER_PIXEL: u64 = 0x0008;
pub const CW_BIT_GRAVITY: u64 = 0x0010;
pub const CW_WIN_GRAVITY: u64 = 0x0020;
pub const CW_BACKING_STORE: u64 = 0x0040;
pub const CW_BACKING_PLANES: u64 = 0x0080;
pub const CW_BACKING_PIXEL: u64 = 0x0100;
pub const CW_OVERRIDE_REDIRECT: u64 = 0x0200;
pub const CW_SAVE_UNDER: u64 = 0x0400;
pub const CW_EVENT_MASK: u64 = 0x0800;
pub const CW_DONT_PROPAGATE: u64 = 0x1000;
pub const CW_COLORMAP: u64 = 0x2000;
pub const CW_CURSOR: u64 = 0x4000;

// visual class
pub const STATIC_GRAY: i32 = 0;
pub const GRAY_SCALE: i32 = 1;
pub const STATIC_COLOR: i32 = 2;
pub const PSEUDO_COLOR: i32 = 3;
pub const TRUE_COLOR: i32 = 4;
pub const DIRECT_COLOR: i32 = 5;

// Used in SetInputFocus, GetInputFocus
pub const REVERT_TO_NONE: i32 = 0;
pub const REVERT_TO_POINTER_ROOT: i32 = 1;
pub const REVERT_TO_PARENT: i32 = 2;
