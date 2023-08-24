
use async_trait::async_trait;
use redis::{Client, Commands};
use serde_json::Value;
use crate::config::config::{Engine};
use crate::core::engine::TEngine;

pub struct Redis {
    connection: Client,
    engine: Engine
}

#[async_trait]
impl TEngine for Redis {
    async fn new(engine: Engine) -> Self {
        // TODO: Validation

        let connection = Client::open(engine.host.clone().unwrap()).unwrap();

        Self { connection, engine }
    }

    async fn get(&mut self) -> Value {
        let data = self.connection.get::<String, String>(self.engine.key.clone().unwrap()).unwrap();

        serde_json::from_str(data.as_str()).unwrap()
    }

    async fn set(&mut self, value: Value) -> bool {
        match self.connection.set::<String, String, String>(self.engine.key.clone().unwrap(), value.to_string()) {
            Ok(_) => true,
            Err(_) => false
        }
    }
}