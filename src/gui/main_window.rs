use gtk::prelude::*;

use crate::gui::constants::GLADE_CSS;

/// Setup the main window
pub fn setup(builder: &gtk::Builder) -> gtk::ApplicationWindow {
    let window: gtk::ApplicationWindow = builder.get_object("main").unwrap();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let screen = window.get_screen().unwrap();
    let style  = gtk::CssProvider::new();

    style.load_from_data(GLADE_CSS).unwrap();

    gtk::StyleContext::add_provider_for_screen(&screen, &style, gtk::STYLE_PROVIDER_PRIORITY_USER);

    window
}
