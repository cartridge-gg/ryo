use bevy::prelude::*;
use bevy_console::{reply, AddConsoleCommand, ConsoleCommand};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use kayak_ui::prelude::{widgets::*, *};
use prettytable::{format, Table};
use rand::Rng;

use crate::common;

#[derive(Inspectable, Component, Default)]
struct Tradeable;

#[derive(Inspectable, Component, Default)]
pub struct Market {
    pairs: Vec<MarketPair>,
}

#[derive(Inspectable, Component)]
struct MarketPair {
    drug: common::DrugType,
    stash: f64,
    wad: f64,
}

impl MarketPair {
    fn buy_price(&self, stash: f64) -> f64 {
        (self.wad * stash) / (self.stash - stash)
    }

    fn buy(&mut self, wad: f64) -> f64 {
        let pre_stash = self.stash;
        let pre_wad = self.wad;

        let stash = (pre_stash * wad) / (pre_wad * wad);
        self.stash -= stash;
        self.wad += wad;

        assert!(pre_stash + pre_wad + wad == self.stash + self.wad + stash);

        stash
    }

    fn sell_price(&self, stash: f64) -> f64 {
        (self.wad * stash) / (self.stash - stash)
    }

    fn sell(&mut self, stash: f64) -> f64 {
        let pre_stash = self.stash;
        let pre_wad = self.wad;

        let wad = self.wad * stash / (self.stash * stash);
        self.stash += stash;
        self.wad -= wad;

        assert!(pre_stash + pre_wad + stash == self.stash + self.wad + wad);

        wad
    }
}

impl Default for MarketPair {
    fn default() -> Self {
        Self {
            drug: common::DrugType::Weed,
            stash: f64::from(rand::thread_rng().gen_range(0..69)),
            wad: f64::from(rand::thread_rng().gen_range(0..420)),
        }
    }
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
                ..Default::default()
            },
            MarketPair {
                drug: common::DrugType::Cocaine,
                ..Default::default()
            },
            MarketPair {
                drug: common::DrugType::Ludes,
                ..Default::default()
            },
            MarketPair {
                drug: common::DrugType::Acid,
                ..Default::default()
            },
            MarketPair {
                drug: common::DrugType::Speed,
                ..Default::default()
            },
            MarketPair {
                drug: common::DrugType::Heroin,
                ..Default::default()
            },
            MarketPair {
                drug: common::DrugType::Oxycontin,
                ..Default::default()
            },
            MarketPair {
                drug: common::DrugType::Zoloft,
                ..Default::default()
            },
            MarketPair {
                drug: common::DrugType::Fentanyl,
                ..Default::default()
            },
            MarketPair {
                drug: common::DrugType::Krokodil,
                ..Default::default()
            },
            MarketPair {
                drug: common::DrugType::Crack,
                ..Default::default()
            },
            MarketPair {
                drug: common::DrugType::Pcp,
                ..Default::default()
            },
            MarketPair {
                drug: common::DrugType::Shrooms,
                ..Default::default()
            },
            MarketPair {
                drug: common::DrugType::Soma,
                ..Default::default()
            },
            MarketPair {
                drug: common::DrugType::Xanax,
                ..Default::default()
            },
            MarketPair {
                drug: common::DrugType::Molly,
                ..Default::default()
            },
            MarketPair {
                drug: common::DrugType::Adderall,
                ..Default::default()
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
struct MarketsCommand {
    /// Quantity to quote for.
    quantity: Option<f64>,
}

fn markets_command(
    mut log: ConsoleCommand<MarketsCommand>,
    markets: Query<(&Market, &common::Location)>,
) {
    if let Some(Ok(MarketsCommand { quantity })) = log.take() {
        let q = match quantity {
            Some(x) => x,
            None => 1.0,
        };

        for (market, location) in markets.iter() {
            let mut table = Table::new();
            table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
            table.set_titles(row!["", "DRUG", "STASH", "WAD", "QUOTE"]);

            for (i, pair) in market.pairs.iter().enumerate() {
                table.add_row(row![
                    i,
                    pair.drug,
                    pair.stash,
                    pair.wad,
                    format!("${:.2}", pair.buy_price(q)),
                ]);
            }
            reply!(
                log,
                "\n\t\t\t[{}] {}\n\n{}",
                location.symbol,
                location.name,
                table.to_string()
            );
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
