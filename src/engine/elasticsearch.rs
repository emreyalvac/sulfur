use async_trait::async_trait;
use elastic::AsyncClient;
use serde_json::{json, Value};
use crate::config::config::{Engine, Transform};
use crate::core::engine::TEngine;
use elastic::prelude::*;
use crate::transform::python::transform;

pub struct ElasticSearch {
    connection: elastic::SyncClient,
    engine: Engine,
    transform: Option<Transform>,
}

#[async_trait]
impl TEngine for ElasticSearch {
    async fn new(engine: Engine, transform: Option<Transform>) -> Self where Self: Sized {
        let connection = SyncClient::builder().static_node(engine.host.clone().unwrap().as_str()).build().unwrap();


        ElasticSearch {
            connection,
            engine,
            transform,
        }
    }

    async fn get(&mut self) -> Value {
        let res = self.connection.search::<Value>()
            .index(self.engine.index.clone().unwrap())
            .body(json!({
                    "size": 10000,
                    "query": {
                        "match_all": {
                        }
                    }
                }))
            .send().unwrap();

        transform(Value::Null, self.transform.clone())
    }

    async fn set(&mut self, value: Value) -> bool {
        true
    }
}