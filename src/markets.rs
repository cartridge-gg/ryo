use bevy::prelude::*;

use crate::common;

#[derive(Component)]
struct Tradeable;

#[derive(Component)]
struct Market {
    pairs: Vec<MarketPair>,
}

#[derive(Component)]
struct MarketPair {
    drug: common::DrugType,
    amount: usize
}

pub struct MarketsPlugin;

impl Plugin for MarketsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_markets);
    }
}

fn add_markets(mut commands: Commands) {
    commands.spawn((
        common::Location,
        common::Name("Brooklyn".to_string()),
        Market { pairs: vec![
            MarketPair{drug: common::DrugType::Weed, amount: 100},
            MarketPair{drug: common::DrugType::Cocaine, amount: 100},
            MarketPair{drug: common::DrugType::Ludes, amount: 100},
            MarketPair{drug: common::DrugType::Acid, amount: 100},
            MarketPair{drug: common::DrugType::Speed, amount: 100},
            MarketPair{drug: common::DrugType::Heroin, amount: 100},
            MarketPair{drug: common::DrugType::Oxycontin, amount: 100},
            MarketPair{drug: common::DrugType::Zoloft, amount: 100},
            MarketPair{drug: common::DrugType::Fentanyl, amount: 100},
            MarketPair{drug: common::DrugType::Krokodil, amount: 100},
            MarketPair{drug: common::DrugType::Crack, amount: 100},
            MarketPair{drug: common::DrugType::Pcp, amount: 100},
            MarketPair{drug: common::DrugType::Shrooms, amount: 100},
            MarketPair{drug: common::DrugType::Soma, amount: 100},
            MarketPair{drug: common::DrugType::Xanax, amount: 100},
            MarketPair{drug: common::DrugType::Molly, amount: 100},
            MarketPair{drug: common::DrugType::Adderall, amount: 100}
        ]},
    ));
}
