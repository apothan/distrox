//! Main menu setup helpers
use gtk::Builder;
use gtk::BuilderExt;
use gtk::Cast;
use gtk::GtkMenuItemExt;

pub fn connect_events_main_menu(builder: &gtk::Builder) {
    let quit_button: gtk::ImageMenuItem = builder
        .get_object(crate::gui::constants::menu::QUIT_BUTTON_NAME)
        .unwrap();

    let quit_button = quit_button.upcast::<gtk::MenuItem>();

    quit_button.connect_activate(|_| {
        gtk::main_quit();
    });
}
