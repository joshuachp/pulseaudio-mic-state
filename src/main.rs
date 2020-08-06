extern crate libpulse_binding as pulse;

use clap::{crate_description, crate_name, crate_version, value_t_or_exit, App, Arg, ArgGroup};
use pulse::callbacks::ListResult;
use pulse::context::introspect::SourceInfo;
use pulse::context::Context;
use pulse::mainloop::standard::{IterateResult, Mainloop};
use pulse::proplist::Proplist;

enum Source {
    Index(u32),
    Name(String),
    Default,
}

fn main() {
    let source = get_arguments();

    let mut prop_list = Proplist::new().unwrap();
    prop_list
        .set_str(
            pulse::proplist::properties::APPLICATION_NAME,
            "PulseaudioMic",
        )
        .unwrap();

    let mut main_loop = Mainloop::new().expect("Failed to create mainloop");

    let mut context = Context::new_with_proplist(&main_loop, "PulseaudioMicContext", &prop_list)
        .expect("Failed to create new context");

    context
        .connect(None, pulse::context::flags::NOAUTOSPAWN, None)
        .expect("Failed to connect context");

    // Wait for context to be ready
    loop {
        match main_loop.iterate(false) {
            IterateResult::Quit(_) | IterateResult::Err(_) => {
                eprintln!("Iterate state was not success, quitting...");
                return;
            }
            IterateResult::Success(_) => {}
        }
        match context.get_state() {
            pulse::context::State::Ready => {
                break;
            }
            pulse::context::State::Failed | pulse::context::State::Terminated => {
                eprintln!("Context state failed/terminated, quitting...");
                return;
            }
            _ => {}
        }
    }

    // Get source state
    let introspect = context.introspect();
    let state;
    match source {
        Source::Index(index) => {
            state = introspect.get_source_info_by_index(index, source_information_callback);
        }
        Source::Name(name) => {
            state = introspect.get_source_info_by_name(&name, source_information_callback);
        }
        Source::Default => {
            state =
                introspect.get_source_info_by_name("@DEFAULT_SOURCE@", source_information_callback);
        }
    }

    // Wait for results
    loop {
        match main_loop.iterate(false) {
            IterateResult::Quit(_) | IterateResult::Err(_) => {
                eprintln!("Iterate state was not success, quitting...");
                return;
            }
            IterateResult::Success(_) => {}
        }
        match state.get_state() {
            pulse::operation::State::Done => {
                break;
            }
            pulse::operation::State::Cancelled => {
                eprintln!("The operation has been cancelled!");
                return;
            }
            pulse::operation::State::Running => {}
        }
    }
}

fn get_arguments() -> Source {
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .after_help(
            "If an id or name is not specified, it will return the state of the default source.",
        )
        .version(crate_version!())
        .arg(
            Arg::with_name("index")
                .short("i")
                .long("index")
                .takes_value(true)
                .help("Index of the source"),
        )
        .arg(
            Arg::with_name("name")
                .long("name")
                .takes_value(true)
                .help("Name of the source"),
        )
        .group(ArgGroup::with_name("SOURCE").args(&["index", "name"]))
        .get_matches();

    if matches.is_present("index") {
        let index: u32 = value_t_or_exit!(matches.value_of("index"), u32);
        return Source::Index(index);
    } else if matches.is_present("name") {
        let name = matches.value_of("name").unwrap();
        return Source::Name(String::from(name));
    }
    Source::Default
}

fn source_information_callback(list: ListResult<&SourceInfo>) {
    if let ListResult::Item(item) = list {
        println!("{}", item.mute)
    }
}
