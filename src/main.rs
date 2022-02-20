mod core;
use std::rc::Rc;
use std::cell::RefCell;
use std::ptr::null;
use bracket_lib::prelude::*;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, core::state::State::new())
}
