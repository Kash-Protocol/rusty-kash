//!
//! Address scanner implementation, responsible for
//! aggregating UTXOs from multiple addresses and
//! building corresponding balances.
//!

use crate::derivation::AddressManager;
use crate::imports::*;
use crate::utxo::balance::AtomicBalance;
use crate::utxo::{UtxoContext, UtxoEntryReference, UtxoEntryReferenceExtension};
use kash_consensus_core::asset_type::AssetType;
use std::cmp::max;

pub const DEFAULT_WINDOW_SIZE: usize = 8;

#[derive(Default, Clone, Copy)]
pub enum ScanExtent {
    /// Scan until an empty range is found
    #[default]
    EmptyWindow,
    /// Scan until a specific depth (a particular derivation index)
    Depth(u32),
}

enum Provider {
    AddressManager(Arc<AddressManager>),
    AddressSet(HashSet<Address>),
}

pub struct Scan {
    provider: Provider,
    window_size: Option<usize>,
    extent: Option<ScanExtent>,
    ksh_balance: Arc<AtomicBalance>,
    kusd_balance: Arc<AtomicBalance>,
    krv_balance: Arc<AtomicBalance>,
    current_daa_score: u64,
}
impl Scan {
    pub fn new_with_address_manager(
        address_manager: Arc<AddressManager>,
        ksh_balance: &Arc<AtomicBalance>,
        kusd_balance: &Arc<AtomicBalance>,
        krv_balance: &Arc<AtomicBalance>,
        current_daa_score: u64,
        window_size: Option<usize>,
        extent: Option<ScanExtent>,
    ) -> Scan {
        Scan {
            provider: Provider::AddressManager(address_manager),
            window_size, //: Some(DEFAULT_WINDOW_SIZE),
            extent,      //: Some(ScanExtent::EmptyWindow),
            ksh_balance: ksh_balance.clone(),
            kusd_balance: kusd_balance.clone(),
            krv_balance: krv_balance.clone(),
            current_daa_score,
        }
    }
    pub fn new_with_address_set(
        addresses: HashSet<Address>,
        ksh_balance: &Arc<AtomicBalance>,
        kusd_balance: &Arc<AtomicBalance>,
        krv_balance: &Arc<AtomicBalance>,
        current_daa_score: u64,
    ) -> Scan {
        Scan {
            provider: Provider::AddressSet(addresses),
            window_size: None,
            extent: None,
            ksh_balance: ksh_balance.clone(),
            kusd_balance: kusd_balance.clone(),
            krv_balance: krv_balance.clone(),
            current_daa_score,
        }
    }

    pub async fn scan(&self, utxo_context: &UtxoContext) -> Result<()> {
        match &self.provider {
            Provider::AddressManager(address_manager) => self.scan_with_address_manager(address_manager, utxo_context).await,
            Provider::AddressSet(addresses) => self.scan_with_address_set(addresses, utxo_context).await,
        }
    }

    pub async fn scan_with_address_manager(&self, address_manager: &Arc<AddressManager>, utxo_context: &UtxoContext) -> Result<()> {
        let window_size = self.window_size.unwrap_or(DEFAULT_WINDOW_SIZE) as u32;
        let extent = self.extent.expect("address manager requires an extent");

        let mut cursor: u32 = 0;
        let mut last_address_index = address_manager.index();

        'scan: loop {
            let first = cursor;
            let last = if cursor == 0 { max(last_address_index + 1, window_size) } else { cursor + window_size };
            cursor = last;

            let addresses = address_manager.get_range(first..last)?;
            utxo_context.register_addresses(&addresses).await?;

            let ts = Instant::now();
            let resp = utxo_context.processor().rpc_api().get_utxos_by_addresses(addresses).await?;
            let elapsed_msec = ts.elapsed().as_secs_f32();
            if elapsed_msec > 1.0 {
                log_warning!("get_utxos_by_address() fetched {} entries in: {} msec", resp.len(), elapsed_msec);
            }
            yield_executor().await;

            if !resp.is_empty() {
                let refs: Vec<UtxoEntryReference> = resp.into_iter().map(UtxoEntryReference::from).collect();
                for utxo_ref in refs.iter() {
                    if let Some(address) = utxo_ref.utxo.address.as_ref() {
                        if let Some(utxo_address_index) = address_manager.inner().address_to_index_map.get(address) {
                            if last_address_index < *utxo_address_index {
                                last_address_index = *utxo_address_index;
                            }
                        } else {
                            panic!("Account::scan_address_manager() has received an unknown address: `{address}`");
                        }
                    }
                }
                // Initialize separate balances for each currency
                let mut ksh_balance = Balance::default();
                let mut kusd_balance = Balance::default();
                let mut krv_balance = Balance::default();

                // Process each UTXO reference and accumulate balances for each asset type
                refs.iter().for_each(|utxo_ref| {
                    let entry_balance = utxo_ref.balance(self.current_daa_score);
                    match utxo_ref.utxo.entry.asset_type {
                        AssetType::KSH => update_balance(&mut ksh_balance, entry_balance),
                        AssetType::KUSD => update_balance(&mut kusd_balance, entry_balance),
                        AssetType::KRV => update_balance(&mut krv_balance, entry_balance),
                    }
                });

                utxo_context.extend_from_scan(refs, self.current_daa_score).await?;

                if !ksh_balance.is_empty() {
                    self.ksh_balance.add(ksh_balance);
                }
                if !kusd_balance.is_empty() {
                    self.kusd_balance.add(kusd_balance);
                }
                if !krv_balance.is_empty() {
                    self.krv_balance.add(krv_balance);
                }
            } else {
                match &extent {
                    ScanExtent::EmptyWindow => {
                        if cursor > last_address_index + window_size {
                            break 'scan;
                        }
                    }
                    ScanExtent::Depth(depth) => {
                        if &cursor > depth {
                            break 'scan;
                        }
                    }
                }
            }
            yield_executor().await;
        }

        address_manager.set_index(last_address_index)?;

        Ok(())
    }

    pub async fn scan_with_address_set(&self, address_set: &HashSet<Address>, utxo_context: &UtxoContext) -> Result<()> {
        let address_vec = address_set.iter().cloned().collect::<Vec<_>>();

        utxo_context.register_addresses(&address_vec).await?;
        let resp = utxo_context.processor().rpc_api().get_utxos_by_addresses(address_vec).await?;
        let refs: Vec<UtxoEntryReference> = resp.into_iter().map(UtxoEntryReference::from).collect();

        // Structs to hold the balances for each asset type
        let mut ksh_total_balance = Balance::default();
        let mut kusd_total_balance = Balance::default();
        let mut krv_total_balance = Balance::default();

        // Process each UTXO reference and accumulate balances for each asset type
        for r in refs.iter() {
            let entry_balance = r.balance(self.current_daa_score);
            match r.utxo.entry.asset_type {
                AssetType::KSH => update_balance(&mut ksh_total_balance, entry_balance),
                AssetType::KUSD => update_balance(&mut kusd_total_balance, entry_balance),
                AssetType::KRV => update_balance(&mut krv_total_balance, entry_balance),
            }
        }
        yield_executor().await;

        utxo_context.extend_from_scan(refs, self.current_daa_score).await?;

        // Update the Scan struct balances
        if !ksh_total_balance.is_empty() {
            self.ksh_balance.add(ksh_total_balance);
        }
        if !kusd_total_balance.is_empty() {
            self.kusd_balance.add(kusd_total_balance);
        }
        if !krv_total_balance.is_empty() {
            self.krv_balance.add(krv_total_balance);
        }

        Ok(())
    }
}

// Function to update balance
fn update_balance(total_balance: &mut Balance, entry_balance: Balance) {
    total_balance.mature += entry_balance.mature;
    total_balance.pending += entry_balance.pending;
    total_balance.mature_utxo_count += entry_balance.mature_utxo_count;
    total_balance.pending_utxo_count += entry_balance.pending_utxo_count;
    total_balance.stasis_utxo_count += entry_balance.stasis_utxo_count;
}
