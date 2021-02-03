mod dataset;
use super::GCloud;

use dataset::Dataset;

pub type GCloudGeneric = Box<dyn GCloud + 'static>;

pub struct BigQueryClient {
    gcloud_client: GCloudGeneric,
    project_id: String,
}

impl BigQueryClient {
    
    pub fn new(gcloud_client: GCloudGeneric, project_id: &str) -> BigQueryClient {
        BigQueryClient{
            gcloud_client,
            project_id: project_id.to_owned(),
        }
    }

    pub fn dataset(dataset_name: &str) -> dataset::Dataset {
        
    }

}