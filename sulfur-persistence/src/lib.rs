use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::{Surreal};
use surrealdb::opt::auth::Root;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub struct SulfurPersistence {
    db: Surreal<Client>,
}

impl SulfurPersistence {
    pub async fn new() -> Self {
        let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();

        db.signin(Root {
            username: "root",
            password: "root",
        })
            .await.unwrap();

        db.use_ns("sulfur_ns").use_db("sulfur_db").await.unwrap();

        Self {
            db
        }
    }

    pub async fn set<T>(&self, data: T) -> bool where T: DeserializeOwned + Serialize {
        match self.db.
            create::<Vec<T>>("flow")
            .content(data).await {
            Ok(_) => true,
            Err(_) => panic!()
        }
    }
}