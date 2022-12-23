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
            MarketPair{drug: DrugType::Weed},
            MarketPair{drug: DrugType::Cocaine},
            MarketPair{drug: DrugType::Ludes},
            MarketPair{drug: DrugType::Acid},
            MarketPair{drug: DrugType::Speed},
            MarketPair{drug: DrugType::Heroin},
            MarketPair{drug: DrugType::Oxycontin},
            MarketPair{drug: DrugType::Zoloft},
            MarketPair{drug: DrugType::Fentanyl},
            MarketPair{drug: DrugType::Krokodil},
            MarketPair{drug: DrugType::Crack},
            MarketPair{drug: DrugType::Pcp},
            MarketPair{drug: DrugType::Shrooms},
            MarketPair{drug: DrugType::Soma},
            MarketPair{drug: DrugType::Xanax},
            MarketPair{drug: DrugType::Molly},
            MarketPair{drug: DrugType::Adderall}
        ]},
    ));
}
