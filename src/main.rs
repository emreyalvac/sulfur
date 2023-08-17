use redis::{RedisWrite, ToRedisArgs, Value};
use serde::{Deserialize, Serialize};
use crate::config::config_reader::ConfigReader;
use crate::core::Engine;
use crate::engine::big_query::BigQuery;
use crate::engine::mongo::Mongo;
use crate::engine::redis::Redis;
use crate::reactor::reactor::_reactor;

mod engine;
mod config;
mod core;
mod utils;
mod reactor;


#[tokio::main]
async fn main() {
    _reactor().await;
}
