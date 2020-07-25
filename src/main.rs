extern crate libpulse_binding as pulse;

use pulse::context::Context;
use pulse::mainloop::standard::IterateResult;
use pulse::mainloop::standard::Mainloop;
use pulse::proplist::Proplist;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

fn main() {
    let mut prop_list = Proplist::new().unwrap();
    prop_list
        .set_str(
            pulse::proplist::properties::APPLICATION_NAME,
            "PulseaudioMic",
        )
        .unwrap();

    let main_loop = Rc::new(RefCell::new(
        Mainloop::new().expect("Failed to create mainloop"),
    ));

    let context = Rc::new(RefCell::new(
        Context::new_with_proplist(
            main_loop.borrow().deref(),
            "PulseaudioMicContext",
            &prop_list,
        )
        .expect("Failed to create new context"),
    ));

    context
        .borrow_mut()
        .connect(None, pulse::context::flags::NOAUTOSPAWN, None)
        .expect("Failed to connect context");

    // Wait for context to be ready
    loop {
        match main_loop.borrow_mut().iterate(false) {
            IterateResult::Quit(_) | IterateResult::Err(_) => {
                eprintln!("Iterate state was not success, quitting...");
                return;
            }
            IterateResult::Success(_) => {}
        }
        match context.borrow().get_state() {
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
    let state = context
        .borrow()
        .introspect()
        .get_source_info_by_index(1, |list| {
            if let pulse::callbacks::ListResult::Item(item) = list {
                println!("{}", item.mute)
            }
        });

    loop {
        match main_loop.borrow_mut().iterate(false) {
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
                println!("Sike");
                break;
            }
            pulse::operation::State::Running => {}
        }
    }
}
