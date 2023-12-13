use super::Droplet;

use serde::{Deserialize, Serialize};

// response of GET /v2/droplets
#[derive(Debug, Deserialize)]
pub struct ListAllDropletsResponse {
    pub droplets: Vec<Droplet>,
}

#[derive(Debug, Serialize)]
pub struct CreateOneDropletReq {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    pub size: String,
    pub image: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ssh_keys: Vec<usize>,
    // pub backups: bool,
    // pub ipv6: bool,
    // pub monitoring: bool,
    // pub tags: Vec<String>,
    // pub user_data: String,
    // // pub private_networking: bool,
    // pub volumes: Vec<String>,
    // pub vpc_uuid: String,
}

// response of POST /v2/droplets
#[derive(Debug, Deserialize)]
pub struct CreateOneDropletResponse {
    pub droplet: Droplet,
    // pub link: ...
}

// response of GET /v2/account/keys
#[derive(Debug, Deserialize)]
pub struct ListAllSSHKeyResponse {
    pub ssh_keys: Vec<SSHKey>,
    // links: Links, // todo marker and pagination
}

#[derive(Debug, Deserialize)]
pub struct SSHKey {
    pub id: usize,
    pub fingerprint: String,
    pub public_key: String,
    pub name: String,
}
