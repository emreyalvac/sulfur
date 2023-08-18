use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub sulfur: Vec<Sulfur>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sulfur {
    pub name: String,
    pub cron: Option<String>,
    pub source: Engine,
    pub destination: Engine,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Engine {
    pub r#type: Option<String>,
    pub host: Option<String>,
    pub port: Option<String>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub credentials: Option<String>,

    // Database
    pub database: Option<String>,
    pub collection: Option<String>,
    pub query: Option<String>,

    // Redis
    pub key: Option<String>,

    // BQ
    pub project_id: Option<String>,
    pub dataset_id: Option<String>,
    pub table_id: Option<String>,
}