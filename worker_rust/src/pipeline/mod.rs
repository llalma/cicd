use pyo3::prelude::*;
use pyo3::Python;

use chrono::{DateTime, Utc};

#[pyclass(get_all)]
pub struct Pipeline {
    name: String,
    start_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>,
}

#[pymethods]
impl Pipeline {
    #[new]
    fn new(name: String) -> Self {
        Pipeline {
            name: name,
            start_time: Utc::now(),
            end_time: None,
        }
    }

    fn __enter__<'p>(slf: PyRef<'p, Self>, _py: Python<'p>) -> PyResult<PyRef<'p, Self>> {
        println!("__enter__ pipeline");

        // Create a database entry for the pipeline

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
