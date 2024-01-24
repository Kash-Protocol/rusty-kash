use crate::convert::error::ConversionError;
use crate::convert::option::TryFromOptionEx;
use crate::pb as protowire;
use kash_oracle::pricing_record;

impl From<pricing_record::PricingRecord> for protowire::PricingRecord {
    fn from(record: pricing_record::PricingRecord) -> Self {
        Self {
            ksh: record.ksh,
            ksh_ma: record.ksh_ma,
            kusd: record.kusd,
            kusd_ma: record.kusd_ma,
            krv: record.krv,
            krv_ma: record.krv_ma,
            timestamp: record.timestamp,
            signature: record.signature,
        }
    }
}

impl From<protowire::PricingRecord> for pricing_record::PricingRecord {
    fn from(record: protowire::PricingRecord) -> Self {
        Self {
            ksh: record.ksh,
            ksh_ma: record.ksh_ma,
            kusd: record.kusd,
            kusd_ma: record.kusd_ma,
            krv: record.krv,
            krv_ma: record.krv_ma,
            timestamp: record.timestamp,
            signature: record.signature,
        }
    }
}

impl TryFromOptionEx<Option<protowire::PricingRecord>> for pricing_record::PricingRecord {
    type Error = ConversionError;

    fn try_from_ex(value: Option<protowire::PricingRecord>) -> Result<Self, Self::Error> {
        match value {
            Some(proto_record) => Ok(pricing_record::PricingRecord {
                ksh: proto_record.ksh,
                ksh_ma: proto_record.ksh_ma,
                kusd: proto_record.kusd,
                kusd_ma: proto_record.kusd_ma,
                krv: proto_record.krv,
                krv_ma: proto_record.krv_ma,
                timestamp: proto_record.timestamp,
                signature: proto_record.signature,
            }),
            None => Err(ConversionError::NoneValue),
        }
    }
}
