mod game;

use bevy::prelude::*;
use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(GamePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, system)
        .run();
}

fn setup() {}

fn system() {}
