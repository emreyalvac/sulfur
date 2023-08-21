use crate::config::config::{Engine, Transform};
use crate::core::engine::TEngine;
use crate::engine::big_query::BigQuery;
use crate::engine::mongo::Mongo;
use crate::engine::redis::Redis;

pub async fn select_engine(r#type: String, config: Engine, transform: Option<Transform>) -> Box<dyn TEngine> {
    if r#type == "Mongo" {
        return Box::new(Mongo::new(config, transform).await);
    } else if r#type == "Redis" {
        return Box::new(Redis::new(config, transform).await);
    } else if r#type == "BigQuery" {
        return Box::new(BigQuery::new(config, transform).await);
    }

    return Box::new(Redis::new(config, transform).await);
}