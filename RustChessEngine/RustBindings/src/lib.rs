use pyo3::prelude::*;
use masks::Masks;
use magic::SlidingMoves;
use GameState::GameState;
use MoveAlgebraNotation::MoveAN;

#[pymodule]
fn Bindings(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
  m.add_class::<Masks>()?;
  m.add_class::<SlidingMoves>()?;
  m.add_class::<GameState>()?;
  m.add_class::<MoveAN>()?;
  m.add_function(wrap_pyfunction!(MoveAlgebraNotation::gen_index_py, m)?)?;
  Ok(())
}