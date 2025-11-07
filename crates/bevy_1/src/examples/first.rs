//! bevy first example

use bevy::prelude::*;
use bevy::DefaultPlugins;
mod hello;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(hello::HelloPlugin)
        .run();
}
