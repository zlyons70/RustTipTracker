use anyhow::Result;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/src/main.rs").await?.print().await?;
    Ok(())
}