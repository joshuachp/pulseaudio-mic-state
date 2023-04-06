use std::{borrow::BorrowMut, ops::ControlFlow};

use clap::Parser;
use cli::Cli;
use color_eyre::{
    eyre::{ensure, eyre, ContextCompat, WrapErr},
    Result,
};
use pulse::{
    callbacks::ListResult,
    context::{introspect::SourceInfo, Context, FlagSet},
    mainloop::standard::{IterateResult, Mainloop},
    proplist::Proplist,
};

mod cli;

const APP_NAME: &str = "pulseaudio-mic-state";

enum Source {
    Index(u32),
    Name(String),
    Default,
}

struct MainIter<'a> {
    main_loop: &'a mut Mainloop,
}

impl<'a> From<&'a mut Mainloop> for MainIter<'a> {
    fn from(main_loop: &'a mut Mainloop) -> Self {
        Self { main_loop }
    }
}

impl Iterator for MainIter<'_> {
    type Item = IterateResult;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.main_loop.iterate(false))
    }
}

impl MainIter<'_> {
    fn try_loop<F, B>(&mut self, mut callback: F) -> Result<B>
    where
        F: FnMut() -> ControlFlow<Result<B>>,
    {
        let value = self.try_for_each(|iter_result| {
            if !iter_result.is_success() {
                return ControlFlow::Break(Err(eyre!(
                    "Iterate state was not success, quitting..."
                )));
            }

            callback()
        });

        match value {
            ControlFlow::Continue(_) => unreachable!("Should never return continue"),
            ControlFlow::Break(value) => value,
        }
    }
}

fn main() -> Result<()> {
    let Cli {
        index,
        name,
        muted,
        unmuted,
    } = Cli::parse();

    color_eyre::install().unwrap();

    let source = index
        .map(Source::Index)
        .or_else(|| name.map(Source::Name))
        .unwrap_or(Source::Default);

    let mut prop_list = Proplist::new().unwrap();
    let res = prop_list.set_str(pulse::proplist::properties::APPLICATION_NAME, APP_NAME);
    ensure!(res.is_ok(), "Failed to set application name property");

    let mut main_loop = Mainloop::new().wrap_err("Failed to create new main loop")?;

    let mut context = Context::new_with_proplist(&main_loop, "PulseaudioMicContext", &prop_list)
        .wrap_err("Failed to create new context")?;

    context
        .connect(None, FlagSet::NOAUTOSPAWN, None)
        .wrap_err("Failed to connect context")?;

    let mut iter: MainIter = main_loop.borrow_mut().into();

    // Wait for context to be ready
    iter.try_loop(|| match context.get_state() {
        pulse::context::State::Ready => ControlFlow::Break(Ok(())),
        pulse::context::State::Failed | pulse::context::State::Terminated => {
            ControlFlow::Break(Err(eyre!("Context state failed or terminated")))
        }
        _ => ControlFlow::Continue(()),
    })?;

    let source_information_callback = move |list: ListResult<&SourceInfo>| {
        if let ListResult::Item(item) = list {
            if item.mute {
                println!("{muted}");
            } else {
                println!("{unmuted}");
            }
        }
    };

    // Get source state
    let introspect = context.introspect();
    let state = match source {
        Source::Index(index) => {
            introspect.get_source_info_by_index(index, source_information_callback)
        }
        Source::Name(name) => {
            introspect.get_source_info_by_name(&name, source_information_callback)
        }
        Source::Default => {
            introspect.get_source_info_by_name("@DEFAULT_SOURCE@", source_information_callback)
        }
    };

    // Wait for results
    iter.try_loop(|| match state.get_state() {
        pulse::operation::State::Done => ControlFlow::Break(Ok(())),
        pulse::operation::State::Cancelled => {
            ControlFlow::Break(Err(eyre!("The operation has been cancelled!")))
        }
        pulse::operation::State::Running => ControlFlow::Continue(()),
    })
}
