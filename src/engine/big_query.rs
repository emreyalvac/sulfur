use std::fmt::Debug;
use async_trait::async_trait;
use gcp_bigquery_client::Client;
use gcp_bigquery_client::model::query_request::QueryRequest;
use gcp_bigquery_client::model::table_data_insert_all_request::TableDataInsertAllRequest;
use serde_json::Value;
use crate::core::Engine;

pub struct BigQuery {
    pub connection: Client,
}

#[async_trait]
impl Engine for BigQuery {
    async fn new() -> Self {
        let client = Client::from_service_account_key_file("./service_key.json").await.unwrap();


        Self { connection: client }
    }

    async fn get(&mut self) -> Value {
        let mut rs = self.connection
            .job()
            .query(
                "hb-kubernetes-operator",
                QueryRequest::new(format!(
                    "SELECT * FROM `{}.{}.{}`",
                    "hb-kubernetes-operator", "test", "test"
                )),
            )
            .await.unwrap();

        let mut values: Vec<Value> = Vec::new();

        while rs.next_row() {
            let mut value = serde_json::json!({});
            for column_name in rs.column_names() {
                value[column_name] = rs.get_serde_by_name::<Value>(column_name.as_str()).unwrap().unwrap();
            }

            values.push(value);
        };

        return serde_json::to_value(values).unwrap();
    }

    async fn set(&mut self, location: String, value: Value) -> bool {
        let mut insert_request = TableDataInsertAllRequest::new();
        insert_request.ignore_unknown_values();

        if value.is_array() {
            for val in value.as_array().unwrap() {
                let x: Value = val.clone();
                insert_request.add_row::<Value>(None, x).expect("TODO: panic message");
            }
        } else {
            insert_request.add_row::<Value>(None, value).expect("TODO: panic message");
        }

        match self.connection
            .tabledata()
            .insert_all("hb-kubernetes-operator", "test", "test", insert_request)
            .await {
            Ok(e) => {}
            Err(e) => {}
        }

        return true;
    }
}