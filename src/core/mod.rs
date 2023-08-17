use std::fmt::Debug;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait Engine {
    async fn new() -> Self;
    async fn get(&mut self) -> Value;
    async fn set(&mut self, location: String, value: Value) -> bool;
}