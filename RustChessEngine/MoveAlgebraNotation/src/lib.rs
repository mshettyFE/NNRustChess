use constants::*;
use GameState::GameState;
use magic::SlidingMoves;
use masks::Masks;

pub struct MoveAN{
    pub _start: String,
    pub _end: String,
    pub _color: Color,
    pub _type: PieceType,
    pub _promotion: PieceType,
    pub _capture: bool,
    pub _enpassant: bool,
    pub _castling: Castling,
}

impl Default for MoveAN{
    fn default() -> Self {
        MoveAN{_start: "".to_string(), _end: "".to_string(), _color : Color::WHITE,  _type: PieceType::NONE, _promotion: PieceType::NONE, _capture: false, _enpassant: false, _castling: Castling::None}
    }
}

impl MoveAN{
    pub fn new(strt: &str, end: &str, color: Color, typ: PieceType,  prm: PieceType, cap: bool, enpass: bool, cast: Castling) -> Self{
        MoveAN{_start: strt.to_string(), _end: end.to_string(), _color : color,  _type: typ, _promotion: prm, 
            _capture: cap, _enpassant: enpass, _castling: cast}
    }
 
    pub fn print(){

    }
 
    fn ValidateMove() -> bool{
        false
    }
 
    pub fn get_start_u64() -> u64{0}
 
    pub fn get_end_u64() -> u64{0}

}

pub fn ParseMove(msg: &str, board: &GameState, slide: &SlidingMoves, masks: &Masks ) -> Result<MoveAN,String>{
    Ok(MoveAN::default())
}