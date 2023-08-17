use serde::{Deserialize, Serialize};

pub struct ConfigReader {}

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

#[derive(Debug, Serialize, Deserialize)]
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
    
    // Redis
    pub key: Option<String>,

    // BQ
    pub project_id: Option<String>,
    pub dataset_id: Option<String>,
    pub table_id: Option<String>,
}

impl ConfigReader {
    pub(crate) fn new() -> Config {
        let f = std::fs::File::open("./config.yml").expect("Could not open file.");
        let scrape_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");

        scrape_config
    }
}