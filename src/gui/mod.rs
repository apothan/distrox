use clap::ArgMatches;
use gtk::prelude::*;

const GLADE_SOURCE : &'static str  = include!("static/gui.glade");
const GLADE_CSS    : &'static [u8] = include_bytes!("static/gui.css");

pub fn gui_main(matches: &ArgMatches) -> i32 {
    if gtk::init().is_err() {
        error!("Error initializing gtk");
        return 1;
    }


    let builder = gtk::Builder::new_from_string(GLADE_SOURCE);
    let window: gtk::ApplicationWindow = builder.get_object("main").unwrap();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let screen = window.get_screen().unwrap();
    let style  = gtk::CssProvider::new();

    style.load_from_data(GLADE_CSS).unwrap();


    gtk::StyleContext::add_provider_for_screen(&screen, &style, gtk::STYLE_PROVIDER_PRIORITY_USER);

    window.show_all();

    // Gives GTK control over program execution:
    gtk::main();

    // Alternatively, to have Rust code control your program and its GTK+ parts, use:
    // loop {
    //     // do some work here, e.g.
    //     while let Ok(ev) = rx.try_recv() {
    //         // ...
    //     }
    //     // then:
    //     gtk::main_iteration_do(false);
    //     sleep_ms(4); // May be unnecessary in your use case.
    //                  // 4ms for consistency - some OS round values to multiplies of 4
    //                  // with e.g. 1ms turned into 4ms
    // }

    0
}
