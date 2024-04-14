use pyo3::prelude::*;

#[pyclass]
pub struct Pipeline {
    stages: Vec<String>,
    stage_count: usize,
    upstream_triggers: Vec<String>,
    downstream_triggers: Vec<String>,
}

#[pymethods]
impl Pipeline {
    #[new]
    fn new(stages: Vec<String>) -> Self {
        Pipeline {
            stages: stages.clone(),
            stage_count: stages.len(),
            upstream_triggers: vec![],
            downstream_triggers: vec![],
        }
    }

    pub fn test_func(&self) -> PyResult<()> {
        println!("{:?}", &self.stages);
        Ok(())
    }
}
