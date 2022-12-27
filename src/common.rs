use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

#[derive(Inspectable, Component, Default)]
pub struct Name(pub String);

#[derive(Inspectable, Component, Default)]
pub struct Location;

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

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<Name>()
            .register_inspectable::<Location>()
            .register_inspectable::<DrugType>();
    }
}
