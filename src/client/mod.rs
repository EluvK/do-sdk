use std::sync::Arc;

use reqwest::{Client, Method, RequestBuilder};

pub mod droplets;
pub mod model;
pub mod ssh_keys;

pub struct DoClient {
    client: Arc<DoBaseClient>,
}

impl DoClient {
    pub fn new(token: String) -> Self {
        Self {
            client: Arc::new(DoBaseClient::new(token)),
        }
    }

    pub fn droplets(&self) -> droplets::DropletsClient {
        droplets::DropletsClient::new(self.client.clone())
    }

    pub fn ssh_keys(&self) -> ssh_keys::SshKeysClient {
        ssh_keys::SshKeysClient::new(self.client.clone())
    }
}

pub struct DoBaseClient {
    token: String,
    client: Client,
    base_url: String,
}

impl DoBaseClient {
    pub fn new(token: String) -> Self {
        Self {
            token,
            client: Client::new(),
            base_url: "https://api.digitalocean.com".into(),
        }
    }

    pub fn request(&self, method: Method, url: &str) -> RequestBuilder {
        self.client
            .request(method, format!("{}{}", self.base_url, url))
            .bearer_auth(&self.token)
    }

    /// Get method
    pub fn get(&self, url: &str) -> RequestBuilder {
        self.request(Method::GET, url)
    }

    /// Post method
    pub fn post(&self, url: &str) -> RequestBuilder {
        self.request(Method::POST, url)
    }

    /// Put method
    pub fn put(&self, url: &str) -> RequestBuilder {
        self.request(Method::PUT, url)
    }

    /// Delete method
    pub fn delete(&self, url: &str) -> RequestBuilder {
        self.request(Method::DELETE, url)
    }
}
