use pyo3::prelude::*;
use pyo3::Python;

use chrono::{DateTime, Utc};
use redis::Commands;

#[path = "../databaseEntry/mod.rs"]
mod databaseEntry;
use crate::pipeline::databaseEntry::entry_pipeline;

#[pyclass(get_all)]
pub struct Pipeline {
    id: String,
    name: String,
    start_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>,
}

#[pymethods]
impl Pipeline {
    #[new]
    fn new(name: String) -> Self {
        Pipeline {
            id: crate::PIPELINE_ID.read().unwrap().to_string(),
            name,
            start_time: Utc::now(),
            end_time: None,
        }
    }

    fn __enter__<'p>(slf: PyRef<'p, Self>, _py: Python<'p>) -> PyResult<PyRef<'p, Self>> {
        println!(
            "{}",
            crate::SETTINGS
                .read()
                .unwrap()
                .get::<String>("REDIS_IP")
                .unwrap()
        );

        println!("__enter__ pipeline");
        println!("{}", &slf.id);
        // Create a database entry for the pipeline
        let mut conn = crate::REDIS.read().unwrap().get_connection().unwrap();

        // Create the DB JSON instance
        let x = entry_pipeline {
            id: slf.id.clone(),
            name: slf.name.clone(),
            stages: vec![],
        };

        let _: () = conn
            .set(&slf.id, serde_json::to_string(&x).unwrap())
            .unwrap();

        // return success
        Ok(slf)
    }

    fn __exit__(
        &mut self,
        _exc_type: crate::PyObject,
        _exc_value: crate::PyObject,
        _traceback: crate::PyObject,
    ) {
        // Set the end time
        self.end_time = Some(Utc::now());

        // Update status in DB for stage as ended

        println!("__exit__ pipeline");
    }
}
