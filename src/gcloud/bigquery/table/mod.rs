mod entities;

use serde::{Serialize};
use super::CrudResult;
use crate::gcloud::{GCloud, GCloudFactory, client::Endpoint};

use entities::{InsertAll};
pub struct Table {
    gcloud_client: GCloud,
    project_id: String,
    dataset_id: String,
    name: String,
}


impl Endpoint for Table {}

impl<'a> Table {

    pub fn insert_many(&self, entities: &Vec<&impl Serialize>) -> CrudResult<()> {
        let header_value = self.gcloud_client.header_value();
        let resource = format!("bigquery/v2/projects/{project_id}/datasets/{dataset_id}/tables/{table_id}/insertAll",
                                     project_id=self.project_id, dataset_id=self.dataset_id, table_id=self.name);
        let endpoint = self.endpoint(resource.as_str());

        let body = InsertAll::new(entities);

        let request_client = reqwest::blocking::Client::new();
        let response = request_client
            .post(endpoint.as_str())
            .json(&body)
            .header("Authorization", header_value)
            .send();

        match response {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err)),
        }
        
    }

    pub fn insert(&self, entity: &impl Serialize) -> CrudResult<()> {
        let entities = &vec![entity];
        self.insert_many(entities)
    }

    pub fn new(gcloud_factory: &'a GCloudFactory, project_id: &str, dataset_id: &str, name: &str) -> Table {
        let gcloud_client = gcloud_factory();
        Table {
            project_id: project_id.to_owned(),
            dataset_id: dataset_id.to_owned(),
            name: name.to_owned(),
            gcloud_client
        }
    }
}