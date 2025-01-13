use anyhow::Result;
use kjspkg_migrator::migrator::run;

#[tokio::main]
pub async fn main() -> Result<()> {
    run().await
}
