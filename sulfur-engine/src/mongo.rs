use async_trait::async_trait;
use futures::{StreamExt};
use mongodb::bson::{Document};
use mongodb::{bson, Client};
use mongodb::options::{ClientOptions, ServerAddress};
use serde_json::Value;
use sulfur_base::engine::engine::TEngine;
use sulfur_base::flow::flow::Engine;

pub struct Mongo {
    connection: Client,
    engine: Engine,
}

#[async_trait]
impl TEngine for Mongo {
    async fn new(engine: Engine) -> Self {
        // TODO: Validation

        let client_options =
            ClientOptions::builder()
                .hosts(vec![ServerAddress::parse(format!("{}:{}", engine.host.clone().unwrap(), engine.port.clone().unwrap())).unwrap()]);

        let client = Client::with_options(client_options.build()).unwrap();

        Self { connection: client, engine }
    }

    async fn get(&mut self) -> Value {
        let db = self.connection.database(self.engine.database.clone().unwrap().as_str());
        let mut find = db.collection::<Document>(self.engine.collection.clone().unwrap().as_str()).find(None, None).await.unwrap();
        let mut data: Vec<Value> = Vec::new();

        while let Some(Ok(doc)) = find.next().await {
            data.push(serde_json::to_value(&doc).unwrap());
        }

        serde_json::to_value(data).unwrap()
    }

    async fn set(&mut self, value: Value) -> bool {
        let db = self.connection.database(self.engine.database.clone().unwrap().as_str());
        let collection = db.collection::<Document>(self.engine.collection.clone().unwrap().as_str());

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