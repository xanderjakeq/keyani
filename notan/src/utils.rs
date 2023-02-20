use device_query::Keycode as QueryKeycode;
use notan::prelude::*;
use rand::{thread_rng, Rng};

use crate::State;

pub fn rand(min: usize, max: usize) -> usize {
    return thread_rng().gen_range(min..max);
}

pub fn should_rerender(app: &mut App, state: &mut State, last_key: &Option<QueryKeycode>) -> bool {
    let window_width = app.window().width();
    let window_height = app.window().height();

    return *last_key != state.last_key
        || window_width != state.window_width
        || window_height != state.window_height;
}
