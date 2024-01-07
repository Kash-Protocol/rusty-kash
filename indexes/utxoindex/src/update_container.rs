use kash_consensus_core::asset_type::AssetType;
use kash_consensus_core::{
    tx::{TransactionOutpoint, UtxoEntry},
    utxo::utxo_diff::UtxoDiff,
    BlockHashSet, HashMapCustomHasher,
};
use kash_hashes::Hash;
use kash_utils::hashmap::NestedHashMapExtensions;

use crate::model::{AssetCirculatingSupplyDiffs, CompactUtxoEntry, UtxoChanges, UtxoSetByScriptPublicKey};

/// A struct holding all changes to the utxoindex with on-the-fly conversions and processing.
pub struct UtxoIndexChanges {
    pub utxo_changes: UtxoChanges,
    pub supply_change: AssetCirculatingSupplyDiffs,
    pub tips: BlockHashSet,
}

impl UtxoIndexChanges {
    /// Create a new [`UtxoIndexChanges`] struct
    pub fn new() -> Self {
        Self {
            utxo_changes: UtxoChanges::new(UtxoSetByScriptPublicKey::new(), UtxoSetByScriptPublicKey::new()),
            supply_change: AssetCirculatingSupplyDiffs::default(),
            tips: BlockHashSet::new(),
        }
    }

    /// Add a [`UtxoDiff`] the the [`UtxoIndexChanges`] struct.
    pub fn update_utxo_diff(&mut self, utxo_diff: UtxoDiff) {
        let (to_add, mut to_remove) = (utxo_diff.add, utxo_diff.remove);

        for (transaction_outpoint, utxo_entry) in to_add.into_iter() {
            if to_remove.remove(&transaction_outpoint).is_some() {
                continue;
            }

            match utxo_entry.asset_type {
                AssetType::KSH => self.supply_change.ksh_supply_diff += utxo_entry.amount as i64,
                AssetType::KUSD => self.supply_change.kusd_supply_diff += utxo_entry.amount as i64,
                AssetType::KRV => self.supply_change.krv_supply_diff += utxo_entry.amount as i64,
            }; // TODO: Using `virtual_state.mergeset_rewards` might be a better way to extract this.

            self.utxo_changes.added.insert_into_nested(
                utxo_entry.script_public_key,
                transaction_outpoint,
                CompactUtxoEntry::new(utxo_entry.amount, utxo_entry.block_daa_score, utxo_entry.is_coinbase, utxo_entry.asset_type),
            );
        }

        for (transaction_outpoint, utxo_entry) in to_remove.into_iter() {
            match utxo_entry.asset_type {
                AssetType::KSH => self.supply_change.ksh_supply_diff -= utxo_entry.amount as i64,
                AssetType::KUSD => self.supply_change.kusd_supply_diff -= utxo_entry.amount as i64,
                AssetType::KRV => self.supply_change.krv_supply_diff -= utxo_entry.amount as i64,
            } // TODO: Using `virtual_state.mergeset_rewards` might be a better way to extract this.

            self.utxo_changes.removed.insert_into_nested(
                utxo_entry.script_public_key,
                transaction_outpoint,
                CompactUtxoEntry::new(utxo_entry.amount, utxo_entry.block_daa_score, utxo_entry.is_coinbase, utxo_entry.asset_type),
            );
        }
    }

    /// Add a [`Vec<(TransactionOutpoint, UtxoEntry)>`] the the [`UtxoIndexChanges`] struct
    ///
    /// Note: This is meant to be used when resyncing.
    pub fn add_utxos_from_vector(&mut self, utxo_vector: Vec<(TransactionOutpoint, UtxoEntry)>) {
        for (transaction_outpoint, utxo_entry) in utxo_vector.into_iter() {
            match utxo_entry.asset_type {
                AssetType::KSH => self.supply_change.ksh_supply_diff += utxo_entry.amount as i64,
                AssetType::KUSD => self.supply_change.kusd_supply_diff += utxo_entry.amount as i64,
                AssetType::KRV => self.supply_change.krv_supply_diff += utxo_entry.amount as i64,
            }

            self.utxo_changes.added.insert_into_nested(
                utxo_entry.script_public_key,
                transaction_outpoint,
                CompactUtxoEntry::new(utxo_entry.amount, utxo_entry.block_daa_score, utxo_entry.is_coinbase, utxo_entry.asset_type),
            );
        }
    }

    pub fn set_tips(&mut self, tips: Vec<Hash>) {
        self.tips = BlockHashSet::from_iter(tips);
    }
}
