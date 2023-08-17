use std::fmt::Debug;
use async_trait::async_trait;
use futures::{StreamExt};
use mongodb::bson::{doc, Document};
use mongodb::{bson, Client};
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use serde_json::Value;
use crate::core::Engine;

pub struct Mongo {
    connection: Client,
}

#[async_trait]
impl Engine for Mongo {
    async fn new() -> Self {
        let uri = "mongodb://127.0.0.1:27017";
        let mut client_options =
            ClientOptions::parse(uri)
                .await.unwrap();
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
        let client = Client::with_options(client_options).unwrap();

        Self { connection: client }
    }

    async fn get(&mut self) -> Value {
        let db = self.connection.database("new_db");
        let mut find = db.collection::<Document>("new_collection").find(None, None).await.unwrap();
        let mut data: Vec<Value> = Vec::new();

        while let Some(Ok(doc)) = find.next().await {
            data.push(serde_json::to_value(&doc).unwrap());
        }

        return serde_json::to_value(data).unwrap();
    }

    async fn set(&mut self, location: String, value: Value) -> bool {
        let db = self.connection.database("yeni_db");
        let mut collection = db.collection::<Document>("yeni_collection");

        if value.is_array() {
            let ar = value.as_array().unwrap();
            for val in ar {
                match collection.insert_one(bson::to_document(&val).unwrap(), None).await {
                    Ok(_) => {}
                    Err(_) => {}
                };
            }

            return true;
        }

        return match collection.insert_one(bson::to_document(&value).unwrap(), None).await {
            Ok(_) => true,
            Err(_) => false
        };
    }
}