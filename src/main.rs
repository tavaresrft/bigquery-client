mod gcloud;

use tokio;
use gcloud::GCloud;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    
    let gcloud_client = GCloud::default();
    
    println!("{}", gcloud_client.header_value());
    Ok(())
}

