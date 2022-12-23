use bevy::prelude::*;

use crate::common;

#[derive(Component)]
struct Inventory {
    item: Vec<InventoryEntry>,
}

struct InventoryEntry {
    item: common::DrugType,
    count: usize,
}

pub struct HustlersPlugin;

impl Plugin for HustlersPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_hustlers)
            .add_startup_system(add_player);
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
    ));
}

fn add_hustlers(mut commands: Commands) {
    commands.spawn((
        common::Location,
        common::Name("Hippie Fugitive".to_string()),
        Inventory {
            item: vec![InventoryEntry {
                item: common::DrugType::Cocaine,
                count: 420,
            }],
        },
    ));
}
