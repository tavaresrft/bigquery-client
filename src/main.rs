mod gcloud;

use gcloud::{GCloud, bigquery::BigQueryClient};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct TesteRow {
    id: u64,
    nome: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let project_id = std::env::var("GOOGLE_CLOUD_PROJECT").unwrap();
    
    let gcloud_factory = Box::new(||{GCloud::default()});

    let client = BigQueryClient::new(gcloud_factory, project_id.as_str());
    let table = client.table("_temporary", "teste");
    
    let row = &TesteRow {id: 3, nome: "teste".to_owned(),};
    let _ = table.insert(row).await;

    println!("Done");
    
    Ok(())
}

