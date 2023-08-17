use std::fmt::Debug;
use async_trait::async_trait;
use crate::core::Engine;
use redis::{Client, Commands};
use serde_json::Value;

pub struct Redis {
    connection: Client,
}

#[async_trait]
impl Engine for Redis {
    async fn new() -> Self {
        let mut client = Client::open("redis://127.0.0.1/").unwrap();

        Self { connection: client }
    }

    async fn get(&mut self) -> Value {
        let data = self.connection.get::<String, String>("test".to_string()).unwrap();

        return serde_json::from_str(data.as_str()).unwrap();
    }

    async fn set(&mut self, location: String, value: Value) -> bool {
        match self.connection.set::<String, String, String>(location, value.to_string()) {
            Ok(_) => true,
            Err(_) => false
        }
    }
}