use pyo3::prelude::*;
use masks::Masks;
use magic::SlidingMoves;
use GameState::GameState;

#[pymodule]
fn Bindings(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
  m.add_class::<Masks>()?;
  m.add_class::<SlidingMoves>()?;
  m.add_class::<GameState>()?;
  Ok(())
}