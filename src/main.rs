mod game;
mod road_segment;
mod my_ui;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            game::GamePlugin, 
        ))
        .run();
}

