#[derive(Component)]
struct Inventory {
    drugs: Vec<InventoryEntry>,
}

struct InventoryEntry {
    item: DrugType,
    count: usize,
}
