use crate::config::config::Config;

pub struct ConfigReader {}

impl ConfigReader {
    pub(crate) fn new(path: &str) -> Config {
        let f = std::fs::File::open(path).expect("Could not open file.");
        let config: Config = serde_yaml::from_reader(f).expect("Could not read values.");

        config
    }
}