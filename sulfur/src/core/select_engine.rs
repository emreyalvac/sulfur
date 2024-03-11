use sulfur_base::engine::engine::TEngine;
use sulfur_base::flow::flow::Engine;
use sulfur_engine::big_query::BigQuery;
use sulfur_engine::elasticsearch::ElasticSearch;
use sulfur_engine::mongo::Mongo;
use sulfur_engine::redis::Redis;

pub async fn select_engine(r#type: String, config: Engine) -> Box<dyn TEngine> {
    // TODO: Move to Compile time

    match r#type.as_str() {
        "Mongo" => Box::new(Mongo::new(config).await),
        "Redis" => Box::new(Redis::new(config).await),
        "BigQuery" => Box::new(BigQuery::new(config).await),
        "ElasticSearch" => Box::new(ElasticSearch::new(config).await),
        _ => panic!("")
    }
}