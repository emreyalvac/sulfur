use async_trait::async_trait;
use serde_json::Value;
use sulfur_common::config::config::Engine;

#[async_trait]
pub trait TEngine {
    async fn new(engine: Engine) -> Self where Self: Sized;
    async fn get(&mut self) -> Value;
    async fn set(&mut self, value: Value) -> bool;
}