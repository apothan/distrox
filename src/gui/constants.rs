//! Constants for the gui

pub const GLADE_SOURCE : &'static str  = include_str!("../../static/gui.glade");
pub const GLADE_CSS    : &'static [u8] = include_bytes!("../../static/gui.css");

pub mod menu {
    pub const QUIT_BUTTON_NAME : &'static str = "main-file-quit";
}

