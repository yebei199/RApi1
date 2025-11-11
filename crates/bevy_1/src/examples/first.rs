//! bevy first example

use bevy::prelude::*;
use bevy::DefaultPlugins;
mod bevy_vector_shapes;
mod hello;
mod muti_move_2d;
mod my_move;
mod three_d_1;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(hello::HelloPlugin)
        // .add_plugins(my_move::MyMovePlugin)
        .add_plugins(three_d_1::MutiMovePlugin)

        .run();
}
