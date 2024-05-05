use pyo3::prelude::*;
use pyo3::wrap_pymodule;
use pyo3::PyResult;

mod segment;

#[pymodule]
fn funrustlib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(segment::segment))?;

    Ok(())
}
