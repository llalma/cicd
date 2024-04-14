mod pipeline;
use pyo3::prelude::*;

#[pymodule]
fn CICD_TBD(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<pipeline::Pipeline>()?;
    Ok(())
}
