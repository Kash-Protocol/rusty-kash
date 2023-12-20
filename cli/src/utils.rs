use crate::error::Error;
use crate::result::Result;
use kash_consensus_core::constants::SOMPI_PER_KASH;
use std::fmt::Display;

pub fn try_parse_required_nonzero_kash_as_sompi_u64<S: ToString + Display>(kash_amount: Option<S>) -> Result<u64> {
    if let Some(kash_amount) = kash_amount {
        let sompi_amount = kash_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_| Error::custom(format!("Supplied Kasapa amount is not valid: '{kash_amount}'")))?
            * SOMPI_PER_KASH as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied Kash amount is not valid: '{kash_amount}'"))
        } else {
            let sompi_amount = sompi_amount as u64;
            if sompi_amount == 0 {
                Err(Error::custom("Supplied required kash amount must not be a zero: '{kash_amount}'"))
            } else {
                Ok(sompi_amount)
            }
        }
    } else {
        Err(Error::custom("Missing Kash amount"))
    }
}

pub fn try_parse_required_kash_as_sompi_u64<S: ToString + Display>(kash_amount: Option<S>) -> Result<u64> {
    if let Some(kash_amount) = kash_amount {
        let sompi_amount = kash_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_| Error::custom(format!("Supplied Kasapa amount is not valid: '{kash_amount}'")))?
            * SOMPI_PER_KASH as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied Kash amount is not valid: '{kash_amount}'"))
        } else {
            Ok(sompi_amount as u64)
        }
    } else {
        Err(Error::custom("Missing Kash amount"))
    }
}

pub fn try_parse_optional_kash_as_sompi_i64<S: ToString + Display>(kash_amount: Option<S>) -> Result<Option<i64>> {
    if let Some(kash_amount) = kash_amount {
        let sompi_amount = kash_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_e| Error::custom(format!("Supplied Kasapa amount is not valid: '{kash_amount}'")))?
            * SOMPI_PER_KASH as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied Kash amount is not valid: '{kash_amount}'"))
        } else {
            Ok(Some(sompi_amount as i64))
        }
    } else {
        Ok(None)
    }
}
