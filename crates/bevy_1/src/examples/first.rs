//! bevy first example

use bevy::prelude::*;
use bevy::DefaultPlugins;
mod bevy_vector_shapes;
mod hello;
mod my_move;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(hello::HelloPlugin)
        .add_plugins(my_move::MyMovePlugin)
        .run();
}
