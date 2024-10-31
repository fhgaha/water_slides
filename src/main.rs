mod game;
mod road_segment;
mod my_ui;
mod fps;

use bevy::prelude::*;

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");

    App::new()
        .add_plugins((
            game::GamePlugin, 
        ))
        .run();
}

