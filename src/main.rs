use bevy::{prelude::*, winit::WinitSettings};
use bevy_inspector_egui::WorldInspectorPlugin;
use kayak_ui::prelude::{widgets::*, *};

mod button;
mod common;
mod hustlers;
mod markets;

pub const HEIGHT: f32 = 900.;
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("202221").unwrap()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 375.0,
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
        .add_plugin(common::CommonPlugin)
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

    widget_context.add_widget_data::<button::PrimaryButtonProps, EmptyState>();
    widget_context.add_widget_system(
        button::PrimaryButtonProps::default().get_name(),
        widget_update::<button::PrimaryButtonProps, EmptyState>,
        button::primary_button_render,
    );

    widget_context.add_widget_data::<markets::MarketProps, EmptyState>();
    widget_context.add_widget_system(
        markets::MarketProps::default().get_name(),
        widget_update::<markets::MarketProps, EmptyState>,
        markets::market_render,
    );

    let parent_id = None;
    rsx! {
        <KayakAppBundle
            styles={KStyle{
                background_color: StyleProp::Value(Color::hex("202221").unwrap()),
                ..default()
            }}
        >
            <ElementBundle
                styles={KStyle{
                    layout_type: LayoutType::Column.into(),
                    ..default()
                }}
            >
                <markets::MarketBundle
                    props={markets::MarketProps{
                        name: "Brooklyn".into()
                    }}
                />
                <button::PrimaryButtonBundle
                    props={button::PrimaryButtonProps{
                        content: "Button".into()
                    }}
                    styles={KStyle{
                        bottom: StyleProp::Value(Units::Pixels(0.0)),
                        ..default()
                    }}
                />
            </ElementBundle>
        </KayakAppBundle>
    };

    commands.spawn(UICameraBundle::new(widget_context));
}
