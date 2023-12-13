use reqwest::StatusCode;
use std::sync::Arc;

use crate::error::{DoError, DoResult};

use super::{model::ListAllSSHKeyResponse, DoBaseClient};

pub struct SshKeysClient {
    client: Arc<DoBaseClient>,
}

impl SshKeysClient {
    pub fn new(client: Arc<DoBaseClient>) -> Self {
        Self { client }
    }

    // todo marker and pagination
    pub async fn list_all_ssh_keys(&self) -> DoResult<ListAllSSHKeyResponse> {
        println!("SshKeysClient::list_all_ssh_keys");
        let resp = self.client.get("/v2/account/keys").send().await?;
        match resp.status() {
            StatusCode::OK => Ok(resp.json().await?),
            code => Err(DoError::ApiRespError(format!(
                "unexpected status code at list_all_ssh_keys: {}, msg: {}",
                code,
                resp.text().await?
            ))),
        }
    }

    // todo: add more methods
    // pub async fn create_a_new_ssh_key(&self) -> DoResult<()> {
    //     println!("SshKeysClient::create_a_new_ssh_key");
    //     Ok(())
    // }
    // pub async fn retrieve_an_existing_ssh_key(&self) -> DoResult<()> {
    //     println!("SshKeysClient::retrieve_an_existing_ssh_key");
    //     Ok(())
    // }
    // pub async fn update_an_existing_ssh_key(&self) -> DoResult<()> {
    //     println!("SshKeysClient::update_an_existing_ssh_key");
    //     Ok(())
    // }
    // pub async fn delete_an_existing_ssh_key(&self) -> DoResult<()> {
    //     println!("SshKeysClient::delete_an_existing_ssh_key");
    //     Ok(())
    // }
}
