use crate::asset_type::AssetType;
use crate::tx::{Transaction, TransactionAction};
use consensus_core::tx::asset_conversion::AssetConversionSerializer;
use kash_oracle::pricing_record::PricingRecord;
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
pub struct ReserveRatioState {
    ksh_supply: u64,
    kusd_supply: u64,
    krv_supply: u64,
    pr: PricingRecord,
    transaction_count: u64,
}

#[derive(Debug)]
pub enum ReserveRatioError {
    MissingRates,
    NegativeReserveValue,
    NegativeLiabilities,
    NegativeTotalReserveCoins,
    ZeroAssetsAndLiabilities,
    CalculationError,
    NegativeReserveRatio,
    RatioBelowMinimum(f64),
    RatioAboveMaximum(f64),
    Other(String),
}

impl ReserveRatioState {
    // Initialize a new instance of ReserveRatioState
    pub fn new(ksh_supply: u64, kusd_supply: u64, krv_supply: u64, pr: PricingRecord) -> Self {
        Self { ksh_supply, kusd_supply, krv_supply, pr, transaction_count: 0 }
    }

    pub fn get_reserve_rate(&self) -> f64 {
        let total_reserve_value = self.pr.ksh as f64 * self.krv_supply as f64;

        if self.kusd_supply == 0 {
            return 0.0;
        }

        total_reserve_value / self.kusd_supply as f64
    }

    // Method to check if the reserve ratio is satisfied for a given transaction
    pub fn reserve_ratio_satisfied(&self, tx_type: TransactionAction) -> Result<bool, ReserveRatioError> {
        if self.pr.has_unset_field() {
            return Err(ReserveRatioError::MissingRates);
        }

        // Early exit if no KSH in the reserve for certain transaction types
        if self.ksh_supply == 0 && tx_type != TransactionAction::MintKUSD {
            return Err(ReserveRatioError::Other("No KSH in reserve.".to_string()));
        }

        // Converting assets, liabilities and reserves
        let reserve_value = self.ksh_supply as f64 * self.pr.ksh as f64;
        let reserve_value_ma = self.ksh_supply as f64 * self.pr.ksh_ma as f64;
        let liabilities = self.kusd_supply as f64;
        let total_reserve_coins = self.krv_supply as f64;

        // Check for negative values
        if reserve_value < 0.0 || reserve_value_ma < 0.0 {
            return Err(ReserveRatioError::NegativeReserveValue);
        }
        if liabilities < 0.0 {
            return Err(ReserveRatioError::NegativeLiabilities);
        }
        if total_reserve_coins < 0.0 {
            return Err(ReserveRatioError::NegativeTotalReserveCoins);
        }

        // Handling zero assets and liabilities
        if reserve_value == 0.0 && liabilities == 0.0 {
            return Err(ReserveRatioError::ZeroAssetsAndLiabilities);
        }

        let reserve_ratio = reserve_value / liabilities;
        let reserve_ratio_ma = reserve_value_ma / liabilities;

        // Check for NaN and negative ratios
        if reserve_ratio.is_nan() || reserve_ratio < 0.0 || reserve_ratio_ma.is_nan() || reserve_ratio_ma < 0.0 {
            return Err(ReserveRatioError::NegativeReserveRatio);
        }

        match tx_type {
            // KSH <--> KRV
            TransactionAction::RedeemViaKRV => {
                // No specific reserve ratio requirement for redeeming KSH
                if reserve_ratio < 4.0 || reserve_ratio_ma < 4.0 {
                    return Ok(false);
                }
            }
            TransactionAction::StakeKSH => {
                // Ensure the reserve ratio does not exceed 8.0
                if reserve_ratio >= 8.0 || reserve_ratio_ma >= 8.0 {
                    return Ok(false);
                }
            }

            // KSH <--> KUSD
            TransactionAction::MintKUSD => {
                // No specific reserve ratio requirement for redeeming KSH
                if reserve_ratio < 4.0 || reserve_ratio_ma < 4.0 {
                    return Ok(false);
                }
            }
            TransactionAction::RedeemViaKUSD => {
                // No specific reserve ratio requirement for redeeming KSH
            }

            TransactionAction::TransferKSH | TransactionAction::TransferKUSD | TransactionAction::TransferKRV => {
                // No specific reserve ratio requirement for regular transfers
            }
        }
        Ok(true)
    }

    // Method to check if a transaction can be added to the state
    pub fn can_add_transaction(&mut self, tx: Arc<Transaction>) -> bool {
        let details = AssetConversionSerializer::deserialize(&tx.payload);

        // Save the current state for rollback
        let original_ksh_supply = self.ksh_supply;
        let original_kusd_supply = self.kusd_supply;
        let original_krv_supply = self.krv_supply;

        // Temporarily update the state based on the transaction type
        match details.from_asset_type {
            AssetType::KSH => self.ksh_supply -= details.from_amount,
            AssetType::KUSD => self.kusd_supply -= details.from_amount,
            AssetType::KRV => self.krv_supply -= details.from_amount,
        }

        match details.to_asset_type {
            AssetType::KSH => self.ksh_supply += details.to_amount,
            AssetType::KUSD => self.kusd_supply += details.to_amount,
            AssetType::KRV => self.krv_supply += details.to_amount,
        }

        // Check if the updated state satisfies the reserve ratio
        let is_satisfied = self.reserve_ratio_satisfied(tx.action).unwrap_or(false);

        // Rollback to the original state regardless of the outcome
        self.ksh_supply = original_ksh_supply;
        self.kusd_supply = original_kusd_supply;
        self.krv_supply = original_krv_supply;

        is_satisfied
    }

    // Method to maybe add a transaction to the state
    pub fn maybe_add_transaction(&mut self, tx: Arc<Transaction>) -> Result<(), ReserveRatioError> {
        let details = AssetConversionSerializer::deserialize(&tx.payload);

        // Save the current state for rollback
        let original_ksh_supply = self.ksh_supply;
        let original_kusd_supply = self.kusd_supply;
        let original_krv_supply = self.krv_supply;

        // Temporarily update the state based on the transaction type
        match details.from_asset_type {
            AssetType::KSH => self.ksh_supply -= details.from_amount,
            AssetType::KUSD => self.kusd_supply -= details.from_amount,
            AssetType::KRV => self.krv_supply -= details.from_amount,
        }

        match details.to_asset_type {
            AssetType::KSH => self.ksh_supply += details.to_amount,
            AssetType::KUSD => self.kusd_supply += details.to_amount,
            AssetType::KRV => self.krv_supply += details.to_amount,
        }

        if tx.action.is_transfer() {
            // Update the transaction count
            self.transaction_count += 1;
            return Ok(());
        }

        // Check if the updated state satisfies the reserve ratio
        let is_satisfied = self.reserve_ratio_satisfied(tx.action)?;

        // Rollback to the original state if the transaction cannot be added
        if !is_satisfied {
            self.ksh_supply = original_ksh_supply;
            self.kusd_supply = original_kusd_supply;
            self.krv_supply = original_krv_supply;
            return Err(ReserveRatioError::Other("Reserve ratio not satisfied.".to_string()));
        }

        // Update the transaction count
        self.transaction_count += 1;

        Ok(())
    }
}
