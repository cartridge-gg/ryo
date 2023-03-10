#[macro_use]
extern crate prettytable;

use bevy::{prelude::*, winit::WinitSettings};
use bevy_console::{ConsoleConfiguration, ConsolePlugin, PrintConsoleLine};
use bevy_inspector_egui::WorldInspectorPlugin;
use kayak_ui::prelude::{widgets::*, *};

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
        .add_plugin(KayakContextPlugin)
        .add_plugin(KayakWidgets)
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(startup)
        .add_plugin(ConsolePlugin)
        .insert_resource(ConsoleConfiguration {
            // override config here
            ..Default::default()
        })
        .add_plugin(common::CommonPlugin)
        .add_plugin(hustlers::HustlersPlugin)
        .add_plugin(locations::LocationsPlugin)
        .add_startup_system(write_to_console)
        .run();
}

fn write_to_console(mut console_line: EventWriter<PrintConsoleLine>) {
    console_line.send(PrintConsoleLine::new("Welcome hustler".to_string()));
}

fn startup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    font_mapping.set_default(asset_server.load("roboto.kayak_font"));

    let mut widget_context = KayakRootContext::new();
    widget_context.add_plugin(KayakWidgetsContextPlugin);

    widget_context.add_widget_data::<button::PrimaryButtonProps, EmptyState>();
    widget_context.add_widget_system(
        button::PrimaryButtonProps::default().get_name(),
        widget_update::<button::PrimaryButtonProps, EmptyState>,
        button::primary_button_render,
    );

    widget_context.add_widget_data::<locations::LocationProps, EmptyState>();
    widget_context.add_widget_system(
        locations::LocationProps::default().get_name(),
        widget_update::<locations::LocationProps, EmptyState>,
        locations::render_location,
    );

    let parent_id = None;
    rsx! {
        <KayakAppBundle
            styles={KStyle{
                background_color: StyleProp::Value(Color::hex("202221").unwrap()),
                ..default()
            }}
        >
            // <ElementBundle
            //     styles={KStyle{
            //         layout_type: LayoutType::Column.into(),
            //         ..default()
            //     }}
            // >
            //     <locations::MarketBundle
            //         props={locations::LocationProps{
            //             name: "Brooklyn".into()
            //         }}
            //     />
            //     <button::PrimaryButtonBundle
            //         props={button::PrimaryButtonProps{
            //             content: "Button".into()
            //         }}
            //         styles={KStyle{
            //             bottom: StyleProp::Value(Units::Pixels(0.0)),
            //             ..default()
            //         }}
            //     />
            // </ElementBundle>
        </KayakAppBundle>
    };
    commands
        .spawn(UICameraBundle::new(widget_context))
        .insert(Name::new("Camera"));
}
