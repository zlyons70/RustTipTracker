use anyhow::Result;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    // Use eprintln! for immediate output
    eprintln!("Test starting...");
    
    // Wait for server to be ready
    for i in 0..3 {
        eprintln!("Attempt {} to connect...", i + 1);
        
        if let Ok(hc) = httpc_test::new_client("http://localhost:8080") {
            eprintln!("Successfully connected to server");
            
            let resp = hc.do_get("/hello2/same").await?;
            eprintln!("Status: {}", resp.status());
            
            let body = resp.text().await?;
            eprintln!("Body: {}", body);
            
            return Ok(());
        }
        sleep(Duration::from_millis(300)).await;
    }
    
    anyhow::bail!("Failed to connect to server after 3 attempts")
}