use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use kayak_ui::prelude::{widgets::*, *};

use crate::common;

#[derive(Inspectable, Component, Default)]
struct Tradeable;

#[derive(Inspectable, Component, Default)]
struct Market {
    pairs: Vec<MarketPair>,
}

#[derive(Inspectable, Component, Default)]
struct MarketPair {
    drug: common::DrugType,
    amount: usize,
}

pub struct MarketsPlugin;

impl Plugin for MarketsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_markets)
            .register_inspectable::<Tradeable>()
            .register_inspectable::<Market>()
            .register_inspectable::<MarketPair>();
    }
}

fn add_markets(mut commands: Commands) {
    commands.spawn((
        common::Location,
        common::Name("Brooklyn".to_string()),
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
        },
    )).insert(Name::new("Location"));
}

#[derive(Component, Default, Clone, PartialEq)]
pub struct MarketProps {
    pub name: String,
}

// In the future this will tell Kayak that these Props belongs to a widget.
// For now it's use to get the `WidgetName` component.
impl Widget for MarketProps {}

// Now we need a widget bundle this can represent a collection of components our widget might have
// Note: You can include custom data here. Just don't expect it to get diffed during update!
#[derive(Bundle)]
pub struct MarketBundle {
    pub props: MarketProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub children: KChildren,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}

impl Default for MarketBundle {
    fn default() -> Self {
        Self {
            props: MarketProps::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            children: KChildren::default(),
            on_event: OnEvent::default(),
            widget_name: MarketProps::default().get_name(),
        }
    }
}

pub fn market_render(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    query: Query<&MarketProps>,
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
