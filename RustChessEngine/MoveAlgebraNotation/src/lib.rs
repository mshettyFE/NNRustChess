use constants::*;
use GameState::GameState;
use magic::SlidingMoves;
use masks::Masks;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use chessio::*;

#[pyclass]
pub struct MoveAN{
    pub _start: u64,
    pub _end: u64,
    pub _color: Color,
    pub _type: PieceType,
    pub _promotion: PieceType,
    pub _capture: bool,
    pub _enpassant: bool,
    pub _castling: Castling,
    pub _index: u16,
}

impl Default for MoveAN{
    fn default() -> Self {
        MoveAN{_start: 0, _end: 0, _color : Color::WHITE,  _type: PieceType::NONE, _promotion: PieceType::NONE, _capture: false, _enpassant: false, _castling: Castling::None, _index: 0}
    }
}

#[pymethods]
impl MoveAN{
    #[new]
    pub fn new_py_move() -> Self {
        MoveAN::default()
    }  

    pub fn parse_move_py(&mut self, msg: &str, board: &GameState, slide: &SlidingMoves, masks: &Masks ) -> PyResult<String> {
        let result = self.ParseSANMove(msg, board, slide, masks);
       match result{
          Ok(t) => Ok((t)),
          Err(msg) => Err(PyTypeError::new_err(msg))
        }
      }

    pub fn get_index(&self) -> PyResult<u16>{Ok(self._index)}


}

impl MoveAN{
    pub fn new(start_pos: u64, end_pos: u64, color: Color, typ: PieceType,  prm: PieceType, cap: bool, enpass: bool, cast: Castling, index:u16) -> Self{
        MoveAN{_start: start_pos, _end: end_pos, _color : color,  _type: typ, _promotion: prm, 
            _capture: cap, _enpassant: enpass, _castling: cast, _index: index}
    }
 
    pub fn print(&self){
        print_locations(self._start);
        print_locations(self._end);
        println!("{:?}", self._color);
        println!("{:?}", self._type);
        println!("{:?}", self._promotion);
        println!("{}", self._capture);
        println!("{}", self._enpassant);
        println!("{:?}", self._castling);
        println!("{}", self._index);
        
    }
 
    pub fn get_start_u64(&self) -> String{"".to_string()}
 
    pub fn get_end_u64(&self) -> String{"".to_string()}

    pub fn ParseSANMove(&mut self, msg: &str, board: &GameState, slide: &SlidingMoves, masks: &Masks ) -> Result<String,String>{
        self._index = 10;
        Ok(("e4".to_string()))
    }

    pub fn ParseUCIMove(&mut self, msg: &str, board: &GameState, slide: &SlidingMoves, masks: &Masks ) -> Result<String,String>{
        self._index = 10;
        Ok(("e2e4".to_string()))
    }
}

#[pyfunction]
pub fn gen_index_py(uci_move: String) -> PyResult<u16>{Ok(gen_index(uci_move))}


// Takes in UCI string and converts that to the index of the output array
pub fn gen_index(uci_move: String) -> u16 {0}