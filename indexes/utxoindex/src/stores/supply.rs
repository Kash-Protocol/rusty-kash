use std::sync::Arc;

use kash_database::{
    prelude::{CachedDbItem, DirectDbWriter, StoreResult, DB},
    registry::DatabaseStorePrefixes,
};

use crate::model::AssetCirculatingSupply;
use crate::model::AssetCirculatingSupplyDiffs;

/// Reader API for `CirculatingSupplyStore`.
pub trait CirculatingSupplyStoreReader {
    fn get(&self) -> StoreResult<AssetCirculatingSupply>;
}

pub trait CirculatingSupplyStore: CirculatingSupplyStoreReader {
    fn update_circulating_supply(&mut self, to_add: AssetCirculatingSupplyDiffs) -> StoreResult<AssetCirculatingSupply>;
    fn insert(&mut self, circulating_supply: AssetCirculatingSupply) -> StoreResult<()>;
    fn remove(&mut self) -> StoreResult<()>;
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

impl CirculatingSupplyStore for DbCirculatingSupplyStore {
    fn update_circulating_supply(&mut self, supply_diff: AssetCirculatingSupplyDiffs) -> StoreResult<AssetCirculatingSupply> {
        let new_supplies = self.access.update(DirectDbWriter::new(&self.db), move |mut current_supplies: AssetCirculatingSupply| {
            // Apply the changes directly to the mutable value
            current_supplies += supply_diff;
            current_supplies
        });

        new_supplies
    }

    fn insert(&mut self, circulating_supply: AssetCirculatingSupply) -> StoreResult<()> {
        self.access.write(DirectDbWriter::new(&self.db), &circulating_supply)
    }

    fn remove(&mut self) -> StoreResult<()> {
        self.access.remove(DirectDbWriter::new(&self.db))
    }
}
