use crate::constrants::{ORACLE_URLS_PUBKEYS, PRICING_RECORD_VALID_TIME_DIFF_FROM_BLOCK, TESTNET_ORACLE_URLS};
use crate::errors::OracleError;
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use hex;
use openssl::{hash::MessageDigest, sign::Verifier, x509::X509};
use rand::rngs::SmallRng;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use std::error::Error;
use std::fmt;
use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Debug, Default, BorshSerialize, BorshDeserialize, BorshSchema, PartialEq)]
pub struct PricingRecord {
    pub ksh: u64,
    pub ksh_ma: u64,
    pub kusd: u64,
    pub kusd_ma: u64,
    pub krv: u64,
    pub krv_ma: u64,
    pub timestamp: u64,
    #[wasm_bindgen(skip)]
    pub signature: Vec<u8>,
}

#[derive(Deserialize)]
pub struct PricingRecordResponse {
    success: bool,
    error: String,
    data: PricingRecord,
}

impl fmt::Display for PricingRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PricingRecord {{ ksh: {}, ksh_ma: {}, kusd: {}, kusd_ma: {}, krv: {}, krv_ma: {}, timestamp: {}, signature: {:?} }}",
            self.ksh, self.ksh_ma, self.kusd, self.kusd_ma, self.krv, self.krv_ma, self.timestamp, self.signature
        )
    }
}

impl FromStr for PricingRecord {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 8 {
            return Err("Incorrect number of fields".to_string());
        }

        let ksh = parts[0].parse().map_err(|_| "Invalid ksh value".to_string())?;
        let ksh_ma = parts[1].parse().map_err(|_| "Invalid ksh_ma value".to_string())?;
        let kusd = parts[2].parse().map_err(|_| "Invalid kusd value".to_string())?;
        let kusd_ma = parts[3].parse().map_err(|_| "Invalid kusd_ma value".to_string())?;
        let krv = parts[4].parse().map_err(|_| "Invalid krv value".to_string())?;
        let krv_ma = parts[5].parse().map_err(|_| "Invalid krv_ma value".to_string())?;
        let timestamp = parts[6].parse().map_err(|_| "Invalid timestamp value".to_string())?;
        let signature = hex::decode(parts[7]).map_err(|_| "Invalid signature format".to_string())?;

        Ok(PricingRecord { ksh, ksh_ma, kusd, kusd_ma, krv, krv_ma, timestamp, signature })
    }
}

impl PricingRecord {
    pub fn new() -> Self {
        Self::default()
    }

    /// Synchronously loads a pricing record.
    ///
    /// This method uses the synchronous version of HTTP requests to load the pricing record.
    /// It iterates through each Oracle URL and uses the first valid response.
    ///
    /// Parameters:
    /// - `timestamp`: The block timestamp.
    /// - `past_median_time`: The median timestamp of the last blocks.
    ///
    /// Returns:
    /// - A `Result` containing `PricingRecord` if successful, or an error if the operation fails.
    pub fn load_sync(timestamp: u64) -> Result<Self, Box<dyn Error>> {
        let client = reqwest::blocking::Client::new();
        let mut last_error = String::new();

        for url in TESTNET_ORACLE_URLS.iter() {
            let request_url = format!("{}/price/?timestamp={}", url, timestamp);

            // Attempt to send request to the Oracle.
            match client.get(&request_url).send() {
                Ok(resp) => {
                    // Attempt to parse the JSON response.
                    match resp.json::<PricingRecordResponse>() {
                        Ok(response) if response.success => {
                            return Ok(response.data);
                        }
                        Ok(response) => last_error = format!("Oracle error at {}: {}", url, response.error),
                        Err(err) => last_error = format!("Failed to parse response from {}: {}", url, err),
                    }
                }
                Err(err) => {
                    last_error = format!("Error sending request to Oracle at {}: {}", url, err);
                }
            }
        }

        // If all requests fail, return an accumulated error.
        Err(Box::new(OracleError::DataLoadError(last_error)))
    }

    /// Asynchronously loads a pricing record.
    ///
    /// This method uses asynchronous HTTP requests to load the pricing record.
    /// It iterates through each Oracle URL and uses the first valid response.
    ///
    /// Parameters:
    /// - `timestamp`: The block timestamp.
    /// - `past_median_time`: The median timestamp of the last blocks.
    ///
    /// Returns:
    /// - A `Result` containing `PricingRecord` if successful, or an error if the operation fails.
    pub async fn load(timestamp: u64) -> Result<Self, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let mut last_error = String::new();

        for url in TESTNET_ORACLE_URLS.iter() {
            let request_url = format!("{}/price/?timestamp={}", url, timestamp);

            // Attempt to send request to the Oracle.
            match client.get(&request_url).send().await {
                Ok(resp) => {
                    // Attempt to parse the JSON response.
                    match resp.json::<PricingRecordResponse>().await {
                        Ok(response) if response.success => {
                            return Ok(response.data);
                        }
                        Ok(response) => last_error = format!("Oracle error at {}: {}", url, response.error),
                        Err(err) => last_error = format!("Failed to parse response from {}: {}", url, err),
                    }
                }
                Err(err) => {
                    last_error = format!("Error sending request to Oracle at {}: {}", url, err);
                }
            }
        }

        // If all requests fail, return an accumulated error.
        Err(Box::new(OracleError::DataLoadError(last_error)))
    }
    pub fn equal(&self, other: &Self) -> bool {
        self.ksh == other.ksh
            && self.ksh_ma == other.ksh_ma
            && self.kusd == other.kusd
            && self.kusd_ma == other.kusd_ma
            && self.krv == other.krv
            && self.krv_ma == other.krv_ma
            && self.timestamp == other.timestamp
            && self.signature == other.signature
    }

    /// Checks if the record is empty.
    pub fn empty(&self) -> bool {
        *self == Self::default()
    }

    /// Verifies the signature against a public key.
    pub fn verify_signature(&self, public_key: &str) -> Result<bool, OracleError> {
        if public_key.is_empty() {
            return Err(OracleError::InvalidPublicKey);
        }

        let public_key = X509::from_pem(public_key.as_bytes())
            .map_err(|_| OracleError::InvalidPublicKey)?
            .public_key()
            .map_err(|_| OracleError::InvalidPublicKey)?;

        let message =
            format!("{},{},{},{},{},{},{}", self.ksh, self.ksh_ma, self.kusd, self.kusd_ma, self.krv, self.krv_ma, self.timestamp);

        let mut verifier =
            Verifier::new(MessageDigest::sha256(), &public_key).map_err(|_| OracleError::SignatureVerificationFailed)?;
        verifier.update(message.as_bytes()).map_err(|_| OracleError::SignatureVerificationFailed)?;
        verifier.verify(&self.signature).map_err(|_| OracleError::SignatureVerificationFailed)
    }

    /// Checks if any of the rates are missing.
    pub fn has_unset_field(&self) -> bool {
        self.ksh == 0 || self.ksh_ma == 0 || self.kusd == 0 || self.kusd_ma == 0 || self.krv == 0 || self.krv_ma == 0
    }

    /// Validates the record based on network type, hard fork version, block timestamp, and last block timestamp.
    pub fn valid(&self, block_timestamp: u64, past_median_time: u64) -> Result<bool, OracleError> {
        if self.empty() {
            return Ok(true);
        }

        if self.has_unset_field() {
            return Err(OracleError::InvalidPricingRecord);
        }

        if !self.verify_signature(ORACLE_URLS_PUBKEYS)? {
            return Err(OracleError::SignatureVerificationFailed);
        }

        // Validate the timestamp
        if self.timestamp > block_timestamp + PRICING_RECORD_VALID_TIME_DIFF_FROM_BLOCK {
            return Err(OracleError::TimestampTooFarInFuture);
        }

        if self.timestamp <= past_median_time {
            return Err(OracleError::TimestampTooOld);
        }

        Ok(true)
    }
}

impl TryFrom<JsValue> for PricingRecord {
    type Error = OracleError;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        from_value(value).map_err(|_| OracleError::DeserializationError)
    }
}

// Helper function to create a random, but reasonable PricingRecord
pub fn create_random_pricing_record(rng: &mut SmallRng) -> PricingRecord {
    let random_signature: Vec<u8> = (0..64).map(|_| rng.gen::<u8>()).collect();

    PricingRecord {
        ksh: rng.gen_range(80..100),
        ksh_ma: rng.gen_range(75..95),
        kusd: rng.gen_range(1..100),
        kusd_ma: rng.gen_range(1..100),
        krv: rng.gen_range(1..100),
        krv_ma: rng.gen_range(1..100),
        timestamp: rng.gen_range(1_600_000_000..1_700_000_000), // Example timestamp range
        signature: random_signature,                            // Randomly generated, may not be a valid signature
    }
}
