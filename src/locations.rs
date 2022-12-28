use bevy::prelude::*;
use bevy_console::{reply, AddConsoleCommand, ConsoleCommand};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use kayak_ui::prelude::{widgets::*, *};

use crate::common;

#[derive(Inspectable, Component, Default)]
struct Tradeable;

#[derive(Inspectable, Component, Default)]
pub struct Market {
    pairs: Vec<MarketPair>,
}

#[derive(Inspectable, Component, Default)]
struct MarketPair {
    drug: common::DrugType,
    amount: usize,
}

pub struct LocationsPlugin;

impl Plugin for LocationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_locations)
            .add_console_command::<LocationsCommand, _>(locations_command)
            .add_console_command::<MarketsCommand, _>(markets_command)
            .register_inspectable::<Tradeable>()
            .register_inspectable::<Market>()
            .register_inspectable::<MarketPair>();
    }
}

fn build_market() -> Market {
    Market {
        pairs: vec![
            MarketPair {
                drug: common::DrugType::Weed,
                amount: 100,
            },
            MarketPair {
                drug: common::DrugType::Cocaine,
                amount: 100,
            },
            MarketPair {
                drug: common::DrugType::Ludes,
                amount: 100,
            },
            MarketPair {
                drug: common::DrugType::Acid,
                amount: 100,
            },
            MarketPair {
                drug: common::DrugType::Speed,
                amount: 100,
            },
            MarketPair {
                drug: common::DrugType::Heroin,
                amount: 100,
            },
            MarketPair {
                drug: common::DrugType::Oxycontin,
                amount: 100,
            },
            MarketPair {
                drug: common::DrugType::Zoloft,
                amount: 100,
            },
            MarketPair {
                drug: common::DrugType::Fentanyl,
                amount: 100,
            },
            MarketPair {
                drug: common::DrugType::Krokodil,
                amount: 100,
            },
            MarketPair {
                drug: common::DrugType::Crack,
                amount: 100,
            },
            MarketPair {
                drug: common::DrugType::Pcp,
                amount: 100,
            },
            MarketPair {
                drug: common::DrugType::Shrooms,
                amount: 100,
            },
            MarketPair {
                drug: common::DrugType::Soma,
                amount: 100,
            },
            MarketPair {
                drug: common::DrugType::Xanax,
                amount: 100,
            },
            MarketPair {
                drug: common::DrugType::Molly,
                amount: 100,
            },
            MarketPair {
                drug: common::DrugType::Adderall,
                amount: 100,
            },
        ],
    }
}

#[derive(Bundle)]
struct LocationBundle {
    location: common::Location,
    market: Market,
}

fn spawn_locations(mut commands: Commands) {
    commands
        .spawn(LocationBundle {
            location: common::Location {
                name: "Brooklyn".to_string(),
                symbol: "BK".to_string(),
            },
            market: build_market(),
        })
        .insert(Name::new("Brooklyn"));
    commands
        .spawn(LocationBundle {
            location: common::Location {
                name: "Los Angeles".to_string(),
                symbol: "LA".to_string(),
            },
            market: build_market(),
        })
        .insert(Name::new("Los Angeles"));
}

/// Prints out the locations.
#[derive(ConsoleCommand)]
#[console_command(name = "locations")]
struct LocationsCommand {}

fn locations_command(
    mut log: ConsoleCommand<LocationsCommand>,
    locations: Query<&common::Location>,
) {
    if let Some(Ok(LocationsCommand {})) = log.take() {
        for location in locations.iter() {
            reply!(log, "\t[{}] {}", location.symbol, location.name);
        }

        log.ok();
    }
}

/// Prints out the locations.
#[derive(ConsoleCommand)]
#[console_command(name = "markets")]
struct MarketsCommand {}

fn markets_command(
    mut log: ConsoleCommand<MarketsCommand>,
    markets: Query<(&Market, &common::Location)>,
) {
    if let Some(Ok(MarketsCommand {})) = log.take() {
        for (market, location) in markets.iter() {
            let mut pairs = String::new();
            for (i, pair) in market.pairs.iter().enumerate() {
                pairs += &format!(" {i}. {}\n", pair.drug)
            }
            reply!(log, "[{}] {}\n{}", location.symbol, location.name, pairs);
        }

        log.ok();
    }
}

#[derive(Component, Default, Clone, PartialEq)]
pub struct LocationProps {
    pub name: String,
}

impl Widget for LocationProps {}

#[derive(Bundle)]
pub struct MarketBundle {
    pub props: LocationProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub children: KChildren,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}

impl Default for MarketBundle {
    fn default() -> Self {
        Self {
            props: LocationProps::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            children: KChildren::default(),
            on_event: OnEvent::default(),
            widget_name: LocationProps::default().get_name(),
        }
    }
}

pub fn render_location(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    query: Query<&LocationProps>,
) -> bool {
    if let Ok(props) = query.get(entity) {
        let parent_id = Some(entity);
        let styles = KStyle {
            border_radius: Corner::all(4.0).into(),
            border_color: StyleProp::Value(Color::hex("000000").unwrap()),
            background_color: StyleProp::Value(Color::hex("141011").unwrap()),
            padding: StyleProp::Value(Edge {
                bottom: Units::Pixels(82.0),
                top: Units::Pixels(8.0),
                right: Units::Pixels(12.0),
                left: Units::Pixels(12.0),
            }),
            ..Default::default()
        };

        rsx! {
            <BackgroundBundle
                styles={styles}
            >
                <ElementBundle
                    styles={KStyle{
                        layout_type: LayoutType::Column.into(),
                        ..default()
                    }}
                >
                    <TextWidgetBundle
                        text={TextProps {
                            content: props.name.clone(),
                            size: 14.0,
                            ..Default::default()
                        }}
                    />
                </ElementBundle>
            </BackgroundBundle>
        };
    }
    true
}
