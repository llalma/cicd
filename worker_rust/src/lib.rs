mod pipeline;
use pyo3::prelude::{wrap_pyfunction, *};

#[pymodule]
fn CICD_TBD(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_class::<pipeline::Pipeline>()?;
    m.add_class::<pipeline::withfunc>()?;
    m.add_function(wrap_pyfunction!(pipeline::stage_wrapper, m)?)?;
    Ok(())
}
