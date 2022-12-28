use bevy::{ecs::query, prelude::*};
use bevy_console::{
    reply, AddConsoleCommand, ConsoleCommand, ConsoleConfiguration, ConsolePlugin, PrintConsoleLine,
};
use bevy_inspector_egui::WorldInspectorPlugin;
use common::Location;

mod button;
mod common;
mod hustlers;
mod locations;

pub const HEIGHT: f32 = 900.;
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("202221").unwrap()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1200.0,
                height: 720.0,
                title: "HUSTLE.EXE".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ConsolePlugin)
        .insert_resource(ConsoleConfiguration {
            // override config here
            ..Default::default()
        })
        .add_plugin(common::CommonPlugin)
        .add_plugin(hustlers::HustlersPlugin)
        .add_plugin(locations::LocationsPlugin)
        .add_startup_system(write_to_console)
        .add_console_command::<LogCommand, _>(log_command)
        .run();
}

fn write_to_console(mut console_line: EventWriter<PrintConsoleLine>) {
    console_line.send(PrintConsoleLine::new("Welcome hustler".to_string()));
}

/// Prints out the locations.
#[derive(ConsoleCommand)]
#[console_command(name = "locations")]
struct LogCommand {
    ///
    
}

fn log_command(
    mut log: ConsoleCommand<LogCommand>,
    locations: Query<(&common::Name, &common::Location, &locations::Market)>,
) {
    if let Some(Ok(LogCommand {})) = log.take() {
        for (i, e) in locations.iter().enumerate() {
            let name = e.0;
            reply!(log, "\t{i}. {name}");
        }

        log.ok();
    }
}
