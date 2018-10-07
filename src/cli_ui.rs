use clap::{Arg, App, SubCommand};

pub fn build_ui<'a>() -> App<'a, 'a> {
    App::new("distrox")
        .version("0.1.0")
        .author("Matthias Beyer <mail@beyermatthias.de>")
        .about("Distributed Social Network Application")

        .subcommand(SubCommand::with_name("gui")
                    .about("Start the GUI application")
                    .version("0.1.0")
                   )

        //
        // "Plumbing" commands
        //

        .subcommand(SubCommand::with_name("is-block")
                    .about("Check whether hash is-block")
                    .version("0.1.0")
                    .arg(Arg::with_name("HASH")
                         .index(1)
                         .required(true)
                         .takes_value(true)
                         .multiple(false)
                         .help("")
                         )
                   )

        // Checker commands

        .subcommand(SubCommand::with_name("is-content")
                    .about("Check whether hash is-content")
                    .version("0.1.0")
                    .arg(Arg::with_name("HASH")
                         .index(1)
                         .required(true)
                         .takes_value(true)
                         .multiple(false)
                         .help("")
                         )
                    )
        .subcommand(SubCommand::with_name("is-post")
                    .about("Check whether hash is-post")
                    .version("0.1.0")
                    .arg(Arg::with_name("HASH")
                         .index(1)
                         .required(true)
                         .takes_value(true)
                         .multiple(false)
                         .help("")
                         )
                    )
        .subcommand(SubCommand::with_name("is-reply")
                    .about("Check whether hash is-reply")
                    .version("0.1.0")
                    .arg(Arg::with_name("HASH")
                         .index(1)
                         .required(true)
                         .takes_value(true)
                         .multiple(false)
                         .help("")
                         )
                    )
        .subcommand(SubCommand::with_name("is-profile")
                    .about("Check whether hash is-profile")
                    .version("0.1.0")
                    .arg(Arg::with_name("HASH")
                         .index(1)
                         .required(true)
                         .takes_value(true)
                         .multiple(false)
                         .help("")
                         )
                    )

        // block commands

        .subcommand(SubCommand::with_name("get-parent-blocks")
                    .about("Get parent blocks of a block")
                    .version("0.1.0")
                    .arg(Arg::with_name("HASH")
                         .index(1)
                         .required(true)
                         .takes_value(true)
                         .multiple(false)
                         .help("")
                         )
                    )
        .subcommand(SubCommand::with_name("get-content-of-block")
                    .about("Get content of block")
                    .version("0.1.0")
                    .arg(Arg::with_name("HASH")
                         .index(1)
                         .required(true)
                         .takes_value(true)
                         .multiple(false)
                         .help("")
                         )
                    )

        // content commands

        .subcommand(SubCommand::with_name("get-devices")
                    .about("Get devices of content blob identified by hash")
                    .version("0.1.0")
                    .arg(Arg::with_name("HASH")
                         .index(1)
                         .required(true)
                         .takes_value(true)
                         .multiple(false)
                         .help("")
                         )
                    )
        .subcommand(SubCommand::with_name("get-payload-type")
                    .about("Get payload type of content blob identified by hash")
                    .version("0.1.0")
                    .arg(Arg::with_name("HASH")
                         .index(1)
                         .required(true)
                         .takes_value(true)
                         .multiple(false)
                         .help("")
                         )
                    )
        .subcommand(SubCommand::with_name("get-payload")
                    .about("Get payload of content blob identified by hash")
                    .version("0.1.0")
                    .arg(Arg::with_name("HASH")
                         .index(1)
                         .required(true)
                         .takes_value(true)
                         .multiple(false)
                         .help("")
                         )
                    )

        // content-post commands

        .subcommand(SubCommand::with_name("get-post-content")
                    .about("Get content of post identified by hash")
                    .version("0.1.0")
                    .arg(Arg::with_name("HASH")
                         .index(1)
                         .required(true)
                         .takes_value(true)
                         .multiple(false)
                         .help("")
                         )
                    )
        .subcommand(SubCommand::with_name("get-post-content-format")
                    .about("Get content format of post identified by hash")
                    .version("0.1.0")
                    .arg(Arg::with_name("HASH")
                         .index(1)
                         .required(true)
                         .takes_value(true)
                         .multiple(false)
                         .help("")
                         )
                    )
        .subcommand(SubCommand::with_name("get-post-reply-to")
                    .about("Get 'reply-to' hash of post identified by hash")
                    .version("0.1.0")
                    .arg(Arg::with_name("HASH")
                         .index(1)
                         .required(true)
                         .takes_value(true)
                         .multiple(false)
                         .help("")
                         )
                    )
        .subcommand(SubCommand::with_name("get-post-metadata")
                    .about("Get metadata of post identified by hash")
                    .version("0.1.0")
                    .arg(Arg::with_name("HASH")
                         .index(1)
                         .required(true)
                         .takes_value(true)
                         .multiple(false)
                         .help("")
                         )
                    )

        // content-profile commands

        .subcommand(SubCommand::with_name("get-profile-names")
                    .about("Get profile names of profile blob identified by hash")
                    .version("0.1.0")
                    .arg(Arg::with_name("HASH")
                         .index(1)
                         .required(true)
                         .takes_value(true)
                         .multiple(false)
                         .help("")
                         )
                    )
        .subcommand(SubCommand::with_name("get-profile-picture")
                    .about("Get profile picture hash of profile blob identified by hash")
                    .version("0.1.0")
                    .arg(Arg::with_name("HASH")
                         .index(1)
                         .required(true)
                         .takes_value(true)
                         .multiple(false)
                         .help("")
                         )
                    )
        .subcommand(SubCommand::with_name("get-profile-more")
                    .about("Get 'more' profile data from profile blob identified by hash")
                    .version("0.1.0")
                    .arg(Arg::with_name("HASH")
                         .index(1)
                         .required(true)
                         .takes_value(true)
                         .multiple(false)
                         .help("")
                         )
                    )

        // Writer commands

        .subcommand(SubCommand::with_name("create-post-blob")
                    .about("")
                    .version("0.1.0")
                    .arg(Arg::with_name("device")
                         .long("device")
                         .required(true)
                         .takes_value(true)
                         .multiple(true)
                         .help("")
                         )
                    .arg(Arg::with_name("timestamp")
                         .long("timestamp")
                         .required(false)
                         .takes_value(true)
                         .multiple(false)
                         .help("")
                         )
                    .arg(Arg::with_name("content-format")
                         .long("format")
                         .required(true)
                         .takes_value(true)
                         .multiple(false)
                         .help("Mimetype of the content pointed to by the hash from --content")
                         )
                    .arg(Arg::with_name("content-hash")
                         .long("content")
                         .required(true)
                         .takes_value(true)
                         .multiple(false)
                         .help("IPFS hash of the content")
                         )
                    .arg(Arg::with_name("replyto-hash")
                         .long("reply-to")
                         .required(false)
                         .takes_value(true)
                         .multiple(false)
                         .help("IPFS hash of post which this post replies to")
                         )
                    .arg(Arg::with_name("comments-will-be-propagated")
                         .long("propagate-comments")
                         .required(false)
                         .takes_value(false)
                         .multiple(false)
                         .help("Add flag that comments will be propagated")
                         )
                    .arg(Arg::with_name("comments-will-be-propagated-until")
                         .long("propagate-comments-until")
                         .required(false)
                         .takes_value(true)
                         .multiple(false)
                         .help("Add flag that comments will be propagated until a date")
                         )
                    )
        .subcommand(SubCommand::with_name("create-attached-post-comments-blob")
                    .about("")
                    .version("0.1.0")
                    .arg(Arg::with_name("device")
                         .long("device")
                         .required(true)
                         .takes_value(true)
                         .multiple(true)
                         .help("")
                         )
                    .arg(Arg::with_name("timestamp")
                         .long("timestamp")
                         .required(false)
                         .takes_value(true)
                         .multiple(false)
                         .help("")
                         )
                    .arg(Arg::with_name("comments-for")
                         .long("comments-for")
                         .short("f")
                         .required(true)
                         .takes_value(true)
                         .multiple(false)
                         .help("IPFS hash of the post the comments are propagated for")
                         )
                    .arg(Arg::with_name("refs")
                         .long("ref")
                         .short("r")
                         .required(true) // would work without, but is senseless
                         .takes_value(true)
                         .multiple(true)
                         .help("IPFS hashes of top-level comments to this post")
                         )
                    )
        .subcommand(SubCommand::with_name("create-profile-blob")
                    .about("")
                    .version("0.1.0")
                    .arg(Arg::with_name("device")
                         .long("device")
                         .required(true)
                         .takes_value(true)
                         .multiple(true)
                         .help("")
                         )
                    .arg(Arg::with_name("timestamp")
                         .long("timestamp")
                         .required(false)
                         .takes_value(true)
                         .multiple(false)
                         .help("")
                         )
                    .arg(Arg::with_name("names")
                         .long("name")
                         .short("n")
                         .required(false)
                         .takes_value(true)
                         .multiple(true)
                         .help("")
                         )
                    .arg(Arg::with_name("picture")
                         .long("picture")
                         .short("p")
                         .required(false)
                         .takes_value(true)
                         .multiple(false)
                         .help("IPFS hash of a picture which shall be associated with that profile")
                         )
                    .arg(Arg::with_name("more")
                         .long("more")
                         .required(false)
                         .takes_value(true)
                         .multiple(true)
                         .help("More data to be associated with that profile. JSON blob assumed with {'key': {'mimetype':'<mimetype>','data':'<ipfshash>'}}")
                         )
                    )
        .subcommand(SubCommand::with_name("create-block-blob")
                    .about("")
                    .version("0.1.0")
                    .arg(Arg::with_name("version")
                         .long("version")
                         .short("v")
                         .required(true)
                         .takes_value(true)
                         .multiple(false)
                         .help("")
                         )
                    .arg(Arg::with_name("parents")
                         .long("parent")
                         .short("p")
                         .required(false)
                         .takes_value(true)
                         .multiple(true)
                         .help("")
                         )
                    .arg(Arg::with_name("content")
                         .long("content")
                         .short("c")
                         .required(true)
                         .takes_value(true)
                         .multiple(false)
                         .help("")
                         )
                    )

        // High-level commands

        .subcommand(SubCommand::with_name("create-profile")
                    .about("Create a new profile")
                    .version("0.1.0")
                    .arg(Arg::with_name("name")
                         .index(1)
                         .required(true)
                         .takes_value(true)
                         .multiple(false)
                         .help("The name of the profile. Also the name of the IPFS Key used for this profile.")
                         )
                    )

        .subcommand(SubCommand::with_name("post")
                    .about("Write a new post")
                    .version("0.1.0")
                    .arg(Arg::with_name("editor")
                         .long("editor")
                         .required(false)
                         .takes_value(true)
                         .multiple(false)
                         .help("Set the editor")
                         )
                    )

}


