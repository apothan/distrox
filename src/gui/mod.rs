use clap::ArgMatches;
use gtk::prelude::*;
use gtk::ButtonExt;
use gtk::BuilderExt;
use gtk::Cast;

mod constants;
mod debug;
mod main_menu;
mod main_window;

pub fn gui_main(matches: &ArgMatches) -> i32 {
    if gtk::init().is_err() {
        error!("Error initializing gtk");
        return 1;
    }

    let builder = gtk::Builder::new_from_string(crate::gui::constants::GLADE_SOURCE);
    debug::print_all_objects(&builder);
    main_menu::connect_events_main_menu(&builder);

    let window = main_window::setup(&builder);
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
