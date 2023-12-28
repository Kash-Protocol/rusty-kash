use crate::imports::*;
use kash_consensus_core::asset_type::AssetType;
use std::convert::TryFrom;

#[derive(Default, Handler)]
#[help("Send a Kash transaction to a public address")]
pub struct Send;

impl Send {
    async fn main(self: Arc<Self>, ctx: &Arc<dyn Context>, argv: Vec<String>, _cmd: &str) -> Result<()> {
        let ctx = ctx.clone().downcast_arc::<KashCli>()?;
        let account = ctx.wallet().account()?;

        // Checking minimum argument length
        if argv.len() < 3 {
            tprintln!(ctx, "usage: send <asset type(KSH/KUSD/KRV)> <address> <amount> <priority fee>");
            return Ok(());
        }

        // Parsing asset type, address, and amounts
        let asset_type = AssetType::from(argv.first().unwrap().as_str());
        let address = Address::try_from(argv.get(1).unwrap().as_str())?;
        let amount_sompi = try_parse_required_nonzero_kash_as_sompi_u64(argv.get(2))?;
        let priority_fee_sompi = try_parse_optional_kash_as_sompi_i64(argv.get(3))?.unwrap_or(0);

        let outputs = PaymentOutputs::from((address.clone(), amount_sompi, asset_type));
        let abortable = Abortable::default();
        let (wallet_secret, payment_secret) = ctx.ask_wallet_secret(Some(&account)).await?;

        let (summary, _ids) = account
            .send(
                asset_type,
                outputs.into(),
                priority_fee_sompi.into(),
                None,
                wallet_secret,
                payment_secret,
                &abortable,
                Some(Arc::new(move |_ptx| {
                    // tprintln!(ctx_, "Sending transaction: {}", ptx.id());
                })),
            )
            .await?;

        tprintln!(ctx, "Send - {summary}");

        Ok(())
    }
}
