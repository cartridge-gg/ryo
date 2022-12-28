use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

#[derive(Inspectable, Component, Default)]
pub struct Name(pub String);

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Inspectable, Component, Default)]
pub struct Location {
    pub name: String,
    pub symbol: String,
}

#[derive(Inspectable, Default)]
pub enum DrugType {
    #[default]
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

impl std::fmt::Display for DrugType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            DrugType::Weed => write!(f, "Weed"),
            DrugType::Cocaine => write!(f, "Cocaine"),
            DrugType::Ludes => write!(f, "Ludes"),
            DrugType::Acid => write!(f, "Acid"),
            DrugType::Speed => write!(f, "Speed"),
            DrugType::Heroin => write!(f, "Heroin"),
            DrugType::Oxycontin => write!(f, "Oxycontin"),
            DrugType::Zoloft => write!(f, "Zoloft"),
            DrugType::Fentanyl => write!(f, "Fentanyl"),
            DrugType::Krokodil => write!(f, "Krokodil"),
            DrugType::Crack => write!(f, "Crack"),
            DrugType::Pcp => write!(f, "Pcp"),
            DrugType::Shrooms => write!(f, "Shrooms"),
            DrugType::Soma => write!(f, "Soma"),
            DrugType::Xanax => write!(f, "Xanax"),
            DrugType::Molly => write!(f, "Molly"),
            DrugType::Adderall => write!(f, "Adderall"),
        }
    }
}

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<Name>()
            .register_inspectable::<Location>()
            .register_inspectable::<DrugType>();
    }
}
