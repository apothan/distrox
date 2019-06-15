use clap::ArgMatches;
use gtk::prelude::*;
use gtk::ButtonExt;
use gtk::BuilderExt;
use gtk::Cast;
use futures::Future;

use crate::repository::Repository;
use crate::configuration::Configuration;
use crate::types::util::IPFSHash;

mod constants;
mod debug;
mod main_menu;
mod main_window;

pub fn gui_main(matches: &ArgMatches, config: Configuration, repo: Repository) -> i32 {
    if gtk::init().is_err() {
        error!("Error initializing gtk");
        return 1;
    }

    let builder = gtk::Builder::new_from_string(crate::gui::constants::GLADE_SOURCE);
    debug::print_all_objects(&builder);
    main_menu::connect_events_main_menu(&builder);

    let window = main_window::setup(&builder);
    window.show_all();

    let (sender, receiver) = std::sync::mpsc::channel();

    let readme_hash = IPFSHash::from("QmTumTjvcYCAvRRwQ8sDRxh8ezmrcr88YFU7iYNroGGTBZ");
    hyper::rt::run({
        repo.resolve_plain(&readme_hash)
            .map_err(|e| {
                let ignore_err = is_match!(e.downcast_ref(), Some(&ipfs_api::response::Error::Api(..)));

                if !ignore_err {
                    error!("Error running: {:?}", e);
                    crate::print_error_details(e);
                }

                ::std::process::exit(1)
            })
            .map(move |bytes| {
                let text = String::from_utf8(bytes).unwrap();
                sender.send(text).unwrap() // send text back to main thread
            })
            .map(|_| ())
    });

    let main_text_view : gtk::TextView = builder.get_object("main-text-view").unwrap();
    main_text_view.set_editable(false);
    let ttt = gtk::TextTagTable::new();
    let buffer = gtk::TextBuffer::new(Some(&ttt));
    let text = receiver.recv().unwrap();
    debug!("Received from IPFS: {}", text);
    buffer.set_text(&text);
    main_text_view.set_buffer(&buffer);

    gtk::main();

    // Gives GTK control over program execution:
    //gtk::main();

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
