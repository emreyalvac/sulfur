use redis::{RedisWrite, ToRedisArgs};
use serde::{Deserialize, Serialize};
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
