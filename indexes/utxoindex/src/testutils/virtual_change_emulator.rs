use crate::model::{AssetCirculatingSupply, AssetCirculatingSupplyDiffs};
use kash_consensus::test_helpers::*;
use kash_consensus_core::asset_type::AssetType;
use kash_consensus_core::{
    tx::ScriptPublicKey,
    utxo::{utxo_collection::UtxoCollection, utxo_diff::UtxoDiff},
    BlockHashSet, HashMapCustomHasher,
};
use kash_hashes::Hash;
#[cfg(test)]
use rand::Rng;
use rand::{rngs::SmallRng, SeedableRng};
use std::sync::Arc;

pub struct VirtualChangeEmulator {
    pub utxo_collection: UtxoCollection,
    pub tips: BlockHashSet,
    pub circulating_supply: AssetCirculatingSupply,
    pub accumulated_utxo_diff: Arc<UtxoDiff>,
    pub virtual_parents: Arc<Vec<Hash>>,
    pub selected_parent_blue_score: u64,
    pub daa_score: u64,
    pub script_public_key_pool: Vec<ScriptPublicKey>,
}

impl VirtualChangeEmulator {
    pub fn new() -> Self {
        Self {
            utxo_collection: UtxoCollection::new(),
            tips: BlockHashSet::new(),
            circulating_supply: AssetCirculatingSupply::default(),
            accumulated_utxo_diff: Arc::new(UtxoDiff::default()),
            virtual_parents: Arc::new(vec![]),
            selected_parent_blue_score: 0,
            daa_score: 0,
            script_public_key_pool: vec![],
        }
    }

    pub fn fill_utxo_collection(&mut self, amount: usize, script_public_key_pool_size: usize) {
        let rng = &mut SmallRng::seed_from_u64(43);
        self.script_public_key_pool.extend((0..script_public_key_pool_size).map(|_| generate_random_p2pk_script_public_key(rng)));
        self.utxo_collection = generate_random_utxos_from_script_public_key_pool(rng, amount, &self.script_public_key_pool);
        for (_, utxo_entry) in self.utxo_collection.clone() {
            match utxo_entry.asset_type {
                AssetType::KSH => self.circulating_supply.ksh_supply += utxo_entry.amount,
                AssetType::KUSD => self.circulating_supply.kusd_supply += utxo_entry.amount,
                AssetType::KRV => self.circulating_supply.krv_supply += utxo_entry.amount,
            }
        }
        self.tips = BlockHashSet::from_iter(generate_random_hashes(rng, 1));
    }

    pub fn change_virtual_state(&mut self, remove_amount: usize, add_amount: usize, tip_amount: usize) {
        let rng = &mut SmallRng::seed_from_u64(42);

        let mut new_circulating_supply_diff = AssetCirculatingSupplyDiffs::default();
        self.accumulated_utxo_diff = Arc::new(UtxoDiff::new(
            UtxoCollection::from_iter(
                generate_random_utxos_from_script_public_key_pool(rng, add_amount, &self.script_public_key_pool).into_iter().map(
                    |(k, v)| {
                        match v.asset_type {
                            AssetType::KSH => new_circulating_supply_diff.ksh_supply_diff += v.amount as i64,
                            AssetType::KUSD => new_circulating_supply_diff.kusd_supply_diff += v.amount as i64,
                            AssetType::KRV => new_circulating_supply_diff.krv_supply_diff += v.amount as i64,
                        }
                        (k, v)
                    },
                ),
            ),
            UtxoCollection::from_iter(self.utxo_collection.iter().take(remove_amount).map(|(k, v)| {
                match v.asset_type {
                    AssetType::KSH => new_circulating_supply_diff.ksh_supply_diff -= v.amount as i64,
                    AssetType::KUSD => new_circulating_supply_diff.kusd_supply_diff -= v.amount as i64,
                    AssetType::KRV => new_circulating_supply_diff.krv_supply_diff -= v.amount as i64,
                }
                (*k, v.clone())
            })),
        ));

        self.utxo_collection.retain(|k, _| !self.accumulated_utxo_diff.remove.contains_key(k));
        self.utxo_collection.extend(self.accumulated_utxo_diff.add.iter().map(move |(k, v)| (*k, v.clone())));

        let new_tips = Arc::new(generate_random_hashes(rng, tip_amount));

        self.virtual_parents = new_tips.clone();
        self.tips = BlockHashSet::from_iter(new_tips.iter().cloned());

        self.circulating_supply += new_circulating_supply_diff;

        self.selected_parent_blue_score = rng.gen();
        self.daa_score = rng.gen();
    }

    pub fn clear_virtual_state(&mut self) {
        self.accumulated_utxo_diff = Arc::new(UtxoDiff::new(UtxoCollection::new(), UtxoCollection::new()));

        self.virtual_parents = Arc::new(vec![]);
        self.selected_parent_blue_score = 0;
        self.daa_score = 0;
    }
}

impl Default for VirtualChangeEmulator {
    fn default() -> Self {
        Self::new()
    }
}
