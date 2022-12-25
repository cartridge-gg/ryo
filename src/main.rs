use bevy::{prelude::*, winit::WinitSettings};
use kayak_ui::prelude::{widgets::*, *};

mod common;
mod hustlers;
mod markets;

pub const HEIGHT: f32 = 900.;
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 375.0,
                height: 720.0,
                title: "HUSTLE.EXE".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(KayakContextPlugin)
        .add_plugin(KayakWidgets)
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(startup)
        .add_plugin(hustlers::HustlersPlugin)
        .add_plugin(markets::MarketsPlugin)
        .run();
}

fn startup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    font_mapping.set_default(asset_server.load("roboto.kayak_font"));

    let mut widget_context = KayakRootContext::new();
    widget_context.add_plugin(KayakWidgetsContextPlugin);
    let parent_id = None;
    rsx! {
        <KayakAppBundle>
            <TextWidgetBundle
                text={TextProps {
                    content: "Hello World".into(),
                    size: 20.0,
                    ..Default::default()
                }}
            />
        </KayakAppBundle>
    };

    commands.spawn(UICameraBundle::new(widget_context));
}
