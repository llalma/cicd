#![feature(specialization)]
use pyo3::prelude::*;
use pyo3::types::{PyCFunction, PyDict, PyTuple, PyType};
use pyo3::PyContextProtocol;

// #[pyclass]
// pub struct Pipeline {
//     stages: Vec<String>,
//     stage_count: usize,
//     upstream_triggers: Vec<String>,
//     downstream_triggers: Vec<String>,
// }
//
// #[pymethods]
// impl Pipeline {
//     #[new]
//     fn new(stages: Vec<String>) -> Self {
//         Pipeline {
//             stages: stages.clone(),
//             stage_count: stages.len(),
//             upstream_triggers: vec![],
//             downstream_triggers: vec![],
//         }
//     }
//
//     pub fn test_func(&self) -> PyResult<()> {
//         println!("{:?}", &self.stages);
//         Ok(())
//     }
// }
//
#[pyclass]
pub struct withfunc {
    name: String,
}

#[pymethods]
impl withfunc {
    #[new]
    fn new(name: String) -> Self {
        withfunc { name: name }
    }

    fn __enter__<'p>(slf: PyRef<'p, Self>, _py: Python<'p>) -> PyResult<PyRef<'p, Self>> {
        Ok(slf)
    }

    fn __exit__<'p>(slf: PyRef<'p, Self>, _py: Python<'p>) -> PyResult<PyRef<'p, Self>> {
        Ok(slf)
    }
    // fn __enter__(&self) {
    //     println!("enter_block")
    // }
    //
    // fn __exit__(&self) {
    //     println!("exit_block")
    // }
}

#[pyproto]
impl<'p> PyContextProtocol<'p> for withfunc {
    fn __enter__(&'p mut self) -> PyResult<()> {
        println!("enter_block");
        Ok(())
    }

    fn __exit__(
        &mut self,
        ty: Option<&'p PyType>,
        _value: Option<&'p PyAny>,
        _traceback: Option<&'p PyAny>,
    ) -> PyResult<bool> {
        println!("exit_block");
        Ok(())
    }
}

#[pyfunction]
pub fn stage_wrapper(py: Python, wraps: PyObject) -> PyResult<&PyCFunction> {
    PyCFunction::new_closure(
        py,
        None,
        None,
        move |args: &PyTuple, kwargs: Option<&PyDict>| -> PyResult<PyObject> {
            Python::with_gil(|py| {
                // Any code to run before function
                println!("before func");

                // Call the given func
                let ret = wraps.call(py, args, kwargs);

                // Any code to run after function
                println!("after func");

                // Return the executed func
                ret
            })
        },
    )
}
