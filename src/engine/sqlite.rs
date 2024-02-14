use crate::config::config::Engine;
use crate::core::engine::TEngine;
use async_trait::async_trait;
use serde_json::Value;
use sqlite::Connection;

/// Sqlite engine for Sulfur
///
/// Will create a new SQLite database if specified database in the config doesn't exist
///
pub struct Sqlite {
    connection: Connection,
    engine: Engine,
}

#[async_trait]
impl TEngine for Sqlite {
    async fn new(engine: Engine) -> Self {
        let clone_engine = engine.clone();
        match clone_engine.sqlite_database {
            Some(database) => {
                let connection = sqlite::open(database).unwrap();
                Sqlite { engine, connection }
            }
            None => {
                panic!("trying use sqlite database without specifiying a database in the config");
            }
        }
    }

    async fn get(&mut self) -> Value {
        //TODO: Add table to config
        //TODO: Get all the values in the table specified in the config
        serde_json::to_value("test").unwrap()
    }

    async fn set(&mut self, value: Value) -> bool {
        //TODO: add the value to the table specified in the config
        //SUGGESTION? maybe add validation to here? if the value
        true
    }
}
