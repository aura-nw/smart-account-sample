use cw_storage_plus::Item;

pub const RECOVER_KEY: Item<Vec<u8>> = Item::new("recover_key");