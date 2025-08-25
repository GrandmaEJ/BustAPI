use pyo3::prelude::*;
mod run;

#[pyclass]
pub struct BustAPI;

#[pymethods]
impl BustAPI {
    #[new]
    pub fn new() -> Self {
        BustAPI
    }

    pub fn run(&self, host: String, port: u16) -> PyResult<()> {
        run::start_server(host, port)
    }
}

/// Python module entry
#[pymodule]
fn bustapi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<BustAPI>()?;
    Ok(())
}
