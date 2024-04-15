use pyo3::prelude::*;
use pyo3::{Python, ToPyObject};

use chrono::prelude::*;
use chrono::{DateTime, Utc};
use redis::Commands;

#[derive(serde::Serialize, serde::Deserialize)]
#[pyclass(get_all)]
pub struct Stage {
    pipeline_id: String,
    name: String,
    start_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>,
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
        println!("__enter__");
        println!("{}", &slf.pipeline_id);

        // Update status in DB for stage as started
        // let mut conn = crate::REDIS.read().unwrap().get_connection().unwrap();
        // let _: () = conn.set("test", "val").unwrap();
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

        println!("{}", self.end_time.unwrap());
        // Update status in DB for stage as ended

        println!("__exit__");
    }
}
