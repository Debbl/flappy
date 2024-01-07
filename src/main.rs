mod obstacle;
mod player;
mod state;

use crate::state::State;
use bracket_lib::terminal::{main_loop, BError, BTermBuilder};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

fn main() -> BError {
    let context = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)?
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}
