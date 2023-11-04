use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use masks::Masks;
use pyo3::types::PySlice;
use pyo3::types::PyTuple;

#[pymodule]
fn Bindings(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
//    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
//    m.add_function(wrap_pyfunction!(vector, m)?)?;
//    m.add_function(wrap_pyfunction!(array, m)?)?;
//    m.add_class::<test>()?;
    m.add_class::<Masks>()?;
    Ok(())
}