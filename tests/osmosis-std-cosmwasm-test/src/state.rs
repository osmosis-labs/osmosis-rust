use cw_storage_plus::{Item, Map};

pub const DEBUG: Item<bool> = Item::new("debug");

/// for testing cosmwasm vm / storage-plus compatibility
pub const MAP: Map<String, String> = Map::new("map");
