use crate::imports::*;
use kash_consensus_core::asset_type::AssetType;

#[derive(Default, Handler)]
#[help("Transfer funds between wallet accounts")]
pub struct Transfer;

impl Transfer {
    async fn main(self: Arc<Self>, ctx: &Arc<dyn Context>, argv: Vec<String>, _cmd: &str) -> Result<()> {
        let ctx = ctx.clone().downcast_arc::<KashCli>()?;

        let account = ctx.wallet().account()?;

        // Checking minimum argument length, now expecting 3 arguments
        if argv.len() < 3 {
            tprintln!(ctx, "usage: transfer <asset type(KSH/KUSD/KRV)> <account> <amount> <priority fee>");
            return Ok(());
        }

        // Parsing asset type
        let asset_type_str = argv.first().unwrap();
        let asset_type = AssetType::from(asset_type_str.as_str());

        let target_account_str = argv.get(1).unwrap();
        let target_account = ctx.find_accounts_by_name_or_id(target_account_str).await?;
        if target_account.id() == account.id() {
            return Err("Cannot transfer to the same account".into());
        }

        // Parsing amount and priority fee
        let amount_sompi = try_parse_required_nonzero_kash_as_sompi_u64(argv.get(2))?;
        let priority_fee_sompi = try_parse_optional_kash_as_sompi_i64(argv.get(3))?.unwrap_or(0);

        let target_address = target_account.receive_address()?;
        let (wallet_secret, payment_secret) = ctx.ask_wallet_secret(Some(&account)).await?;

        let abortable = Abortable::default();
        let outputs = PaymentOutputs::from((target_address.clone(), amount_sompi, asset_type));

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

        tprintln!(ctx, "Transfer - {summary}");

        Ok(())
    }
}
