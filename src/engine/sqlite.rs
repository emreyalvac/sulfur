use core::panic;
use std::collections::HashMap;

use crate::config::config::Engine;
use crate::core::engine::TEngine;
use async_trait::async_trait;
use serde_json::{json, Value};
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
        // VALIDATION?
        // in the case of a db doesn't exists with the specified name
        // sqlite library will create a database with the specified name
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
        let query = format!(
            "SELECT * FROM {}",
            match self.engine.sqlite_table.clone() {
                Some(table) => table,
                None => {
                    panic!("trying use sqlite database without specifiying a table in the config");
                }
            }
        );
        let mut data: Vec<Value> = Vec::new();

        // without knowing the exact columns to get,
        // if we make end user specify the data columns,
        // performance improvements would be something to consider
        if let Err(err) = self.connection.iterate(query, |pairs| {
            let mut hash_map = HashMap::new();
            for &(name, value) in pairs.iter() {
                if let Some(value) = value {
                    hash_map.insert(name, value);
                };
            }
            data.push(json!(hash_map));
            true
        }) {
            println!("{}", err)
        }

        serde_json::to_value(data).unwrap()
    }

    async fn set(&mut self, value: Value) -> bool {
        let table = match self.engine.sqlite_table.clone() {
            Some(table) => table,
            None => {
                panic!("trying use sqlite database without specifiying a table in the config");
            }
        };

        let column = match self.engine.sqlite_column.clone() {
            Some(table) => table,
            None => {
                panic!("trying write a value to sqlite database without specifiying a column in the config");
            }
        };

        // SQLite supports json,
        // but the row should be specified
        // and the specified row's type should be TEXT
        let query = format!("INSERT INTO {table} ({column}) VALUES ('{value}');");
        match self.connection.execute(query) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
