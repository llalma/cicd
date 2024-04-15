use pyo3::prelude::*;
use pyo3::{Python, ToPyObject};

use chrono::prelude::*;
use chrono::{DateTime, Utc};
use redis::Commands;

#[pyclass(get_all)]
pub struct Stage {
    name: String,
    start_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>,
}

#[pymethods]
impl Stage {
    #[new]
    fn new(name: String) -> Self {
        Stage {
            name: name,
            start_time: Utc::now(),
            end_time: None,
        }
    }

    fn __enter__<'p>(slf: PyRef<'p, Self>, _py: Python<'p>) -> PyResult<PyRef<'p, Self>> {
        println!("__enter__");

        // Update status in DB for stage as started
        let mut conn = crate::REDIS.read().unwrap().get_connection().unwrap();
        let _: () = conn.set("test", "val").unwrap();

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

        println!("{}", self.end_time.unwrap());
        // Update status in DB for stage as ended

        println!("__exit__");
    }
}
