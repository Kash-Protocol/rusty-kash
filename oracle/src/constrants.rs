/// The acceptable time difference (in seconds) between the block timestamp and the
/// pricing record timestamp for the pricing record to be considered valid.
pub const PRICING_RECORD_VALID_TIME_DIFF_FROM_BLOCK: u64 = 120 * 1000; // 2 minutes

pub const MAINNET_ORACLE_URLS: &'static [&'static str] = &["https://oracle-1.kashnet.org", "https://oracle-2.kashnet.org"];

pub const TESTNET_ORACLE_URLS: &'static [&'static str] =
    &["https://oracle-1.testnet.kashnet.org", "https://oracle-2.testnet.kashnet.org"];

pub const ORACLE_URLS_PUBKEYS: &'static str = "-----BEGIN PUBLIC KEY-----\n\
     UNFILLED\n\
     -----END PUBLIC KEY-----";
