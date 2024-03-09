use async_trait::async_trait;
use elasticsearch::{Elasticsearch, IndexParts, SearchParts};
use elasticsearch::http::transport::Transport;
use serde_json::{json, Value};
use sulfur_common::config::config::Engine;
use crate::core::engine::TEngine;

pub struct ElasticSearch {
    connection: Elasticsearch,
    engine: Engine,
}

#[async_trait]
impl TEngine for ElasticSearch {
    async fn new(engine: Engine) -> Self where Self: Sized {
        let transport = Transport::single_node(engine.host.clone().unwrap().as_str()).unwrap();
        let connection = Elasticsearch::new(transport);

        ElasticSearch {
            connection,
            engine,
        }
    }

    async fn get(&mut self) -> Value {
        let search_response = self.connection
            .search(SearchParts::Index(&[self.engine.index.clone().unwrap().as_str()]))
            .body(json!({
                "size": self.engine.size.clone().unwrap(),
                "query": {
                    "match_all": {}
                }
        }))
            .allow_no_indices(true)
            .send()
            .await.unwrap();

        let response_body = search_response.json::<Value>().await.unwrap();

        response_body
    }

    async fn set(&mut self, value: Value) -> bool {
        let response = self.connection
            .index(IndexParts::Index(self.engine.index.clone().unwrap().as_str()))
            .body(value)
            .send()
            .await;

        match response {
            Ok(_) => true,
            Err(_) => false
        }
    }
}