use crate::config::config::{Engine, Transform};
use crate::core::engine::TEngine;
use crate::engine::big_query::BigQuery;
use crate::engine::elasticsearch::ElasticSearch;
use crate::engine::mongo::Mongo;
use crate::engine::redis::Redis;

pub async fn select_engine(r#type: String, config: Engine, transform: Option<Transform>) -> Box<dyn TEngine> {
    // TODO: Move to Compile time

    if r#type == "Mongo" {
        return Box::new(Mongo::new(config).await);
    } else if r#type == "Redis" {
        return Box::new(Redis::new(config).await);
    } else if r#type == "BigQuery" {
        return Box::new(BigQuery::new(config).await);
    } else if r#type == "ElasticSearch" {
        return Box::new(ElasticSearch::new(config).await);
    }

    return Box::new(Redis::new(config).await);
}