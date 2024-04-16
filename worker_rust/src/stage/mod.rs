use pyo3::prelude::*;
use pyo3::{Python, ToPyObject};
use serde::Deserialize;

use chrono::prelude::*;
use chrono::{DateTime, Utc};
use lazy_static::__Deref;
use redis::Commands;
use std::fmt;

#[path = "../databaseEntry/mod.rs"]
mod databaseEntry;
// use crate::pipeline::databaseEntry::entry_pipeline;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[pyclass(get_all)]
pub struct Stage {
    pipeline_id: String,
    name: String,
    start_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>,
}

impl fmt::Display for Stage {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let _ = fmt.write_str(&format!("{}", &self.name));
        Ok(())
    }
}

#[pymethods]
impl Stage {
    #[new]
    fn new(name: String) -> Self {
        Stage {
            pipeline_id: crate::PIPELINE_ID.read().unwrap().to_string(),
            name: name,
            start_time: Utc::now(),
            end_time: None,
        }
    }

    fn __enter__<'p>(slf: PyRef<'p, Self>, _py: Python<'p>) -> PyResult<PyRef<'p, Self>> {
        // Add the stage to the DB
        add_stage(&slf.pipeline_id, slf.deref().clone());

        Ok(slf)
    }

    fn __exit__(
        &mut self,
        _exc_type: crate::PyObject,
        _exc_value: crate::PyObject,
        _traceback: crate::PyObject,
    ) {
        // Set the end time
        update_endtime(&self.pipeline_id);
    }
}

fn add_stage(id: &str, stage: Stage) {
    // Get the DB connection
    let mut conn = crate::REDIS.read().unwrap().get_connection().unwrap();

    // Fetch the data from redis
    let pipeline_json: String = conn.get(id).unwrap();
    let mut pipeline_data: databaseEntry::entry_pipeline =
        serde_json::from_str(&pipeline_json).unwrap();

    // Add the stage
    pipeline_data.stages.push(stage);

    // Push updated pipeline to DB
    let _: () = conn
        .set(id, serde_json::to_string(&pipeline_data).unwrap())
        .unwrap();
}

fn update_endtime(id: &str) {
    // Get the DB connection
    let mut conn = crate::REDIS.read().unwrap().get_connection().unwrap();

    // Fetch the data from redis
    let pipeline_json: String = conn.get(id).unwrap();
    let mut pipeline_data: databaseEntry::entry_pipeline =
        serde_json::from_str(&pipeline_json).unwrap();

    // Pop last value from stages
    let mut stage: Stage = pipeline_data.stages.pop().unwrap();

    // Update with the end time
    stage.end_time = Some(Utc::now());

    // Add the stage back to instance
    pipeline_data.stages.push(stage);

    // Push updated pipeline to DB
    let _: () = conn
        .set(id, serde_json::to_string(&pipeline_data).unwrap())
        .unwrap();
}
