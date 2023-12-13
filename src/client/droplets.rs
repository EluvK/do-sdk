use std::{fmt::Display, sync::Arc};

use async_trait::async_trait;
use reqwest::StatusCode;

use crate::error::{DoError, DoResult};

use super::{
    model::{CreateOneDropletReq, CreateOneDropletResponse, Droplet, ListAllDropletsResponse},
    DoBaseClient,
};

pub struct DropletsClient {
    client: Arc<DoBaseClient>,
}

impl DropletsClient {
    pub fn new(client: Arc<DoBaseClient>) -> Self {
        Self { client }
    }

    // create one droplet
    pub async fn create(&self, req: CreateOneDropletReq) -> DoResult<CreateOneDropletResponse> {
        println!("DropletsClient::create");
        let resp = self.client.post("/v2/droplets").json(&req).send().await?;
        match resp.status() {
            StatusCode::ACCEPTED => Ok(resp.json().await?),
            code => Err(DoError::ApiRespError(format!(
                "unexpected status code at create: {}, msg: {}",
                code,
                resp.text().await?
            ))),
        }
    }

    // delete one droplet
    pub async fn delete(&self, dropet_id: impl Display) -> DoResult<()> {
        println!("DropletsClient::delete");
        let resp = self
            .client
            .delete(&format!("/v2/droplets/{dropet_id}"))
            .send()
            .await?;
        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            code => Err(DoError::ApiRespError(format!(
                "unexpected status code at delete: {}, msg: {}",
                code,
                resp.text().await?
            ))),
        }
    }

    // list all droplets
    pub async fn list(&self) -> DoResult<ListAllDropletsResponse> {
        println!("DropletsClient::list");
        let resp = self.client.get("/v2/droplets").send().await?;
        match resp.status() {
            StatusCode::OK => Ok(resp.json().await?),
            code => Err(DoError::ApiRespError(format!(
                "unexpected status code at list: {}, msg: {}",
                code,
                resp.text().await?
            ))),
        }
    }
}

#[async_trait]
pub trait DropletHelper {
    async fn wait_creating(&self, droplet_id: usize) -> DoResult<Droplet>;
}

#[async_trait]
impl DropletHelper for DropletsClient {
    async fn wait_creating(&self, droplet_id: usize) -> DoResult<Droplet> {
        println!("DropletsClient::wait_creating");
        loop {
            let droplets = self.list().await?;
            if let Some(droplet) = droplets.droplets.iter().find(|d| d.id == droplet_id) {
                if droplet.status == "active" {
                    return Ok(droplet.clone());
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    }
}
