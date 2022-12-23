use bevy::prelude::*;

mod markets;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(markets::MarketsPlugin)
        .run();
}
