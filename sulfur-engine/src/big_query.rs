use async_trait::async_trait;
use gcp_bigquery_client::Client;
use gcp_bigquery_client::model::query_request::QueryRequest;
use gcp_bigquery_client::model::table_data_insert_all_request::TableDataInsertAllRequest;
use serde_json::Value;
use sulfur_base::engine::engine::TEngine;
use sulfur_base::flow::flow::Engine;

pub struct BigQuery {
    connection: Client,
    engine: Engine,
}

#[async_trait]
impl TEngine for BigQuery {
    async fn new(engine: Engine) -> Self {
        // Validation

        let connection = Client::from_service_account_key_file(engine.credentials.clone().unwrap().as_str()).await.unwrap();

        Self { connection, engine }
    }

    async fn get(&mut self) -> Value {
        // TODO: Query will be dynamic

        let mut rs = self.connection
            .job()
            .query(
                self.engine.project_id.clone().unwrap().as_str(),
                QueryRequest::new(format!(
                    "SELECT * FROM `{}.{}.{}`",
                    self.engine.project_id.clone().unwrap(), self.engine.dataset_id.clone().unwrap(), self.engine.table_id.clone().unwrap()
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

        serde_json::to_value(values).unwrap()
    }

    async fn set(&mut self, value: Value) -> bool {
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
            .insert_all(self.engine.project_id.clone().unwrap().as_str(), self.engine.dataset_id.clone().unwrap().as_str(), self.engine.table_id.clone().unwrap().as_str(), insert_request)
            .await {
            Ok(_e) => {}
            Err(_e) => {}
        }

        return true;
    }
}