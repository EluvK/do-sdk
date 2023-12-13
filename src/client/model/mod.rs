// request and response
mod re;
pub use re::*;

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Droplet {
    pub id: usize,
    pub name: String,
    pub memory: usize,
    pub vcpus: usize,
    pub disk: usize,
    pub locked: bool,
    pub status: String,
    // pub kernel: Option<Kernel>,
    pub created_at: DateTime<Utc>,
    // pub features: Vec<String>,
    // pub backup_ids: Vec<usize>,
    // pub snapshot_ids: Vec<usize>,
    // pub volume_ids: Vec<usize>,
    // pub size: Size,
    // pub size_slug: String,
    pub networks: Networks,
    pub region: Region,
    // pub tags: Vec<String>,
    // pub next_backup_window: Option<NextBackupWindow>,
    // pub urn: String,
    pub image: Image,
    // pub volume_ids: Vec<usize>,
    // pub vpc_uuid: Option<String>,
    // pub database_cluster_ids: Vec<usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Networks {
    pub v4: Vec<Network>,
    pub v6: Vec<Network>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Network {
    pub ip_address: String,
    pub netmask: String,
    pub gateway: String,
    #[serde(rename = "type")]
    pub r#type: String,
    // pub cidr: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Region {
    pub name: String,
    pub slug: String,
    // pub sizes: Vec<String>,
    // pub features: Vec<String>,
    pub available: bool,
    // pub sizes_info: Vec<SizeInfo>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Image {
    pub id: usize,
    pub name: String,
    pub distribution: String,
    // pub slug: Option<String>,
    // pub public: bool,
    // pub regions: Vec<String>,
    pub created_at: DateTime<Utc>,
    // pub min_disk_size: usize,
    // pub type_: String,
    // pub size_gigabytes: f64,
    // pub description: String,
    // pub tags: Vec<String>,
    // pub status: String,
    // pub error_message: Option<String>,
    // pub snapshot_ids: Vec<usize>,
    // pub droplet_ids: Vec<usize>,
    // pub volume_ids: Vec<usize>,
    // pub min_memory_size: usize,
}
