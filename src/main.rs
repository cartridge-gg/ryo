use bevy::prelude::*;

mod hustlers;
mod markets;
mod common;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(hustlers::HustlersPlugin)
        .add_plugin(markets::MarketsPlugin)
        .run();
}
