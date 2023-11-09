use pyo3::prelude::*;
use masks::Masks;
use magic::SlidingMoves;

#[pymodule]
fn Bindings(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
//    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
//    m.add_function(wrap_pyfunction!(vector, m)?)?;
//    m.add_function(wrap_pyfunction!(array, m)?)?;
//    m.add_class::<test>()?;
  m.add_class::<Masks>()?;
  m.add_class::<SlidingMoves>()?;
  Ok(())
}