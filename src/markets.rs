use bevy::prelude::*;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Tradeable;

enum DrugType {
    Weed,
    Cocaine,
    Ludes,
    Acid,
    Speed,
    Heroin,
    Oxycontin,
    Zoloft,
    Fentanyl,
    Krokodil,
    Crack,
    Pcp,
    Shrooms,
    Soma,
    Xanax,
    Molly,
    Adderall,
}

#[derive(Component)]
struct Market {
    pairs: Vec<MarketPair>,
}

#[derive(Component)]
struct MarketPair {
    drug: DrugType,
    amount: usize
}

#[derive(Component)]
struct Location;

pub struct MarketsPlugin;

impl Plugin for MarketsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_markets);
    }
}

fn add_markets(mut commands: Commands) {
    commands.spawn((
        Location,
        Name("Brooklyn".to_string()),
        Market { pairs: vec![
            MarketPair{drug: DrugType::Weed, amount: 100},
            MarketPair{drug: DrugType::Cocaine, amount: 100},
            MarketPair{drug: DrugType::Ludes, amount: 100},
            MarketPair{drug: DrugType::Acid, amount: 100},
            MarketPair{drug: DrugType::Speed, amount: 100},
            MarketPair{drug: DrugType::Heroin, amount: 100},
            MarketPair{drug: DrugType::Oxycontin, amount: 100},
            MarketPair{drug: DrugType::Zoloft, amount: 100},
            MarketPair{drug: DrugType::Fentanyl, amount: 100},
            MarketPair{drug: DrugType::Krokodil, amount: 100},
            MarketPair{drug: DrugType::Crack, amount: 100},
            MarketPair{drug: DrugType::Pcp, amount: 100},
            MarketPair{drug: DrugType::Shrooms, amount: 100},
            MarketPair{drug: DrugType::Soma, amount: 100},
            MarketPair{drug: DrugType::Xanax, amount: 100},
            MarketPair{drug: DrugType::Molly, amount: 100},
            MarketPair{drug: DrugType::Adderall, amount: 100}
        ]},
    ));
}
