use crate::imports::*;
use kash_consensus_core::tx::TransactionAction;
use kash_wallet_core::tx::PaymentDestination;

#[derive(Default, Handler)]
#[help("Estimate the fees for a transaction of a given amount and transaction action")]
pub struct Estimate;

impl Estimate {
    async fn main(self: Arc<Self>, ctx: &Arc<dyn Context>, argv: Vec<String>, _cmd: &str) -> Result<()> {
        let ctx = ctx.clone().downcast_arc::<KashCli>()?;

        let account = ctx.wallet().account()?;

        // Expecting at least two arguments: amount and transaction action
        if argv.len() < 2 {
            tprintln!(ctx, "usage: estimate <transaction action> <amount> [<priority fee>]");
            return Ok(());
        }

        let tx_action_str = &argv[0];
        let tx_action = TransactionAction::try_from(tx_action_str)
            .map_err(|_| Error::custom(format!("Invalid transaction action: {}", tx_action_str)))?;

        let amount_sompi = try_parse_required_nonzero_kash_as_sompi_u64(argv.get(1))?;
        let priority_fee_sompi = try_parse_optional_kash_as_sompi_i64(argv.get(2))?.unwrap_or(0);
        let abortable = Abortable::default();

        // Just use any address for an estimate (change address)
        let change_address = account.change_address()?;
        let destination_asset_type = tx_action.asset_transfer_types().1;
        let destination =
            PaymentDestination::PaymentOutputs(PaymentOutputs::from((change_address.clone(), amount_sompi, destination_asset_type)));
        let estimate = account.estimate(tx_action, destination, priority_fee_sompi.into(), None, &abortable).await?;

        tprintln!(ctx, "Estimate - {estimate}");

        Ok(())
    }
}
