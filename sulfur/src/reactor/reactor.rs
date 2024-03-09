use clap::Parser;
use sulfur_common::config::config::Transform;
use sulfur_config::config_reader::ConfigReader;
use sulfur_transform::python::transform;
use crate::core::engine::{TEngine};
use crate::core::select_engine::select_engine;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: String,
}

struct EngineFlow {
    source: Box<dyn TEngine>,
    destination: Box<dyn TEngine>,
    transform: Option<Transform>,
}

pub async fn _reactor() {
    let mut engines: Vec<EngineFlow> = Vec::new();

    let args = Args::parse();
    let config = ConfigReader::new(args.config.as_str());

    for sulfur in config.sulfur {
        let source = select_engine(sulfur.source.r#type.clone().unwrap(), (&sulfur).source.clone()).await;
        let destination = select_engine(sulfur.destination.r#type.clone().unwrap(), (&sulfur).destination.clone()).await;

        engines.push(EngineFlow { source, destination, transform: sulfur.transform });
    }

    // TODO: Implement multi job at the same time
    for mut flow in engines {
        let transformed_data = transform(flow.source.get().await, flow.transform);

        flow.destination.set(transformed_data).await;
    }
}