mod pipeline;
mod stage;
use pyo3::prelude::{wrap_pyfunction, *};

use config::Config;
use lazy_static::lazy_static;
use redis::{Client, Commands};
use std::sync::RwLock;

// Global Configs
lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new(
        Config::builder()
            .add_source(config::Environment::with_prefix("CICD"))
            .add_source(config::File::with_name("Config.toml").required(false))
            .build()
            .unwrap()
    );
}

lazy_static! {
    static ref REDIS: RwLock<Client> = RwLock::new(
        redis::Client::open(format!(
            "redis://{}:{}",
            SETTINGS.read().unwrap().get::<String>("REDIS_IP").unwrap(),
            SETTINGS
                .read()
                .unwrap()
                .get::<String>("REDIS_PORT")
                .unwrap(),
        ))
        .unwrap()
    );
}

#[pymodule]
fn CICD_TBD(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_class::<pipeline::Pipeline>()?;
    m.add_class::<pipeline::Pipeline>()?;
    m.add_class::<stage::Stage>()?;
    // m.add_function(wrap_pyfunction!(pipeline::stage_wrapper, m)?)?;
    Ok(())
}
