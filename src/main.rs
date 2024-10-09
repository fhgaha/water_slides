mod game;
mod road_segment;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            game::GamePlugin, 
            ))
        .run();
}

