use bevy::{prelude::*, winit::WinitSettings};

mod common;
mod hustlers;
mod markets;

pub const HEIGHT: f32 = 900.;
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("000000").unwrap()))
        .add_plugins(DefaultPlugins.set(
            // here we configure the main window
            WindowPlugin {
                window: WindowDescriptor {
                    width: 375.0,
                    height: 720.0,
                    title: "HUSTLE.EXE".to_string(),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(setup)
        .add_plugin(hustlers::HustlersPlugin)
        .add_plugin(markets::MarketsPlugin)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}
