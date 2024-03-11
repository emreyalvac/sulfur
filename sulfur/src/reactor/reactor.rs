use log::{info, LevelFilter, trace};
use sulfur_base::engine::engine::TEngine;
use sulfur_base::flow::flow::Flow;
use sulfur_persistence::SulfurPersistence;

pub struct SulfurReactor {
    sulfur_persistence: SulfurPersistence,
}

impl SulfurReactor {
    pub async fn new() -> Self {
        pretty_env_logger::init();

        let sulfur_persistence = SulfurPersistence::new().await;
        info!("Reactor initialized.");

        Self { sulfur_persistence }
    }

    pub async fn add_new_flow(&self, flow: Flow) {
        self.sulfur_persistence.set(flow).await;
    }

    pub fn status(&self) {}

    pub fn execute(&self) {}
}

// pub async fn _reactor() {
//     let mut engines: Vec<EngineFlow> = Vec::new();
//
//     let args = Args::parse();
//     let config = ConfigReader::new(args.config.as_str());
//
//     for sulfur in config.sulfur {
//         let source = select_engine(sulfur.source.r#type.clone().unwrap(), (&sulfur).source.clone()).await;
//         let destination = select_engine(sulfur.destination.r#type.clone().unwrap(), (&sulfur).destination.clone()).await;
//
//         engines.push(EngineFlow { source, destination, transform: sulfur.transform });
//     }
//
//     // TODO: Implement multi job at the same time
//     for mut flow in engines {
//         let transformed_data = transform(flow.source.get().await, flow.transform);
//
//         flow.destination.set(transformed_data).await;
//     }
// }