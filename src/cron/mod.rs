mod bundle;
mod error;
mod validate;
mod contract;
mod slasher;
mod transactions;
mod leader;
pub mod arweave;

use std::time::Duration;

use futures::{Future, join};
use paris::{info, error};

use self::error::ValidatorCronError;

// Update contract state
pub async fn run_crons() {
    info!("Validator starting ...");
    join!(
        //create_cron("update contract", contract::update_contract, 30),
        create_cron("validate bundler", validate::validate, 2 * 60),
        create_cron("validate transactions", validate::validate_transactions , 30),
        create_cron("send transactions to leader", leader::send_txs_to_leader, 60)
    );
}

async fn create_cron<F>(description: &'static str, f: impl Fn() -> F + 'static, sleep: u64) 
where
    F: Future<Output = Result<(), ValidatorCronError>> + 'static,
    F::Output: 'static
{
        loop {
            info!("Task running - {}", description);
            match f().await {
                Ok(_) => info!("Task finished - {}", description),
                Err(e) => error!("Task error - {} with {}", description, e),
            };

            info!("Task sleeping for {} seconds - {}", sleep, description);
            tokio::time::sleep(Duration::from_secs(sleep)).await;
        };
}