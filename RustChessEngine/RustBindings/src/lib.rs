use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use masks::Masks;


/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn vector() ->PyResult<Vec<u64>> {
    Ok(Vec::from([1,2,3,4]))
}

#[pyfunction]
fn two_vector() ->PyResult<Vec<u64>> {
    Ok(Vec::from([1,2,3,4,5,6,7,8]))
}

#[pymodule]
fn Bindings(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(vector, m)?)?;
    m.add_function(wrap_pyfunction!(two_vector, m)?)?;
    Ok(())
}