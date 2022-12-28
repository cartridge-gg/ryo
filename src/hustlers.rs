use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use crate::common;

#[derive(Inspectable, Component, Default)]
struct Inventory {
    item: Vec<InventoryEntry>,
}

#[derive(Inspectable, Component, Default)]
struct InventoryEntry {
    item: common::DrugType,
    count: usize,
}

pub struct HustlersPlugin;

impl Plugin for HustlersPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_hustlers)
            .add_startup_system(add_player)
            .register_inspectable::<Inventory>()
            .register_inspectable::<InventoryEntry>();
    }
}

fn add_player(mut commands: Commands) {
    commands.spawn((
        common::Location,
        common::Name("Hippie Fugitive Player".to_string()),
        Inventory {
            item: vec![InventoryEntry {
                item: common::DrugType::Cocaine,
                count: 420,
            }],
        },
    )).insert(Name::new("Player"));
}

fn add_hustlers(mut commands: Commands) {
    commands.spawn((
        common::Location,
        common::Name("Hippie Fugitive".to_string()),
        Inventory {
            item: vec![InventoryEntry {
                item: common::DrugType::Weed,
                count: 69,
            }],
        },
    )).insert(Name::new("Hustler"));
}
