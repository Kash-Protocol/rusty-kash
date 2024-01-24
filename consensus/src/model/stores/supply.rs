use serde::{Deserialize, Serialize};
use std::sync::Arc;

use kash_database::{
    prelude::{CachedDbItem, StoreResult, DB},
    registry::DatabaseStorePrefixes,
};

/// Reader API for `CirculatingSupplyStore`.
pub trait CirculatingSupplyStoreReader {
    fn get(&self) -> StoreResult<AssetCirculatingSupply>;
}

/// A DB + cache implementation of `UtxoIndexTipsStore` trait
#[derive(Clone)]
pub struct DbCirculatingSupplyStore {
    db: Arc<DB>,
    access: CachedDbItem<AssetCirculatingSupply>,
}

impl DbCirculatingSupplyStore {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db: Arc::clone(&db), access: CachedDbItem::new(db, DatabaseStorePrefixes::CirculatingSupply.into()) }
    }
}

impl CirculatingSupplyStoreReader for DbCirculatingSupplyStore {
    fn get(&self) -> StoreResult<AssetCirculatingSupply> {
        self.access.read()
    }
}

/// Represents the circulating supply for each asset type.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct AssetCirculatingSupply {
    pub ksh_supply: u64,
    pub kusd_supply: u64,
    pub krv_supply: u64,
}
