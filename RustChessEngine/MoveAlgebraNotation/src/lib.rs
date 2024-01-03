use constants::*;
use chessio::*;

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

    pub fn emit_UCI(&self)-> String{
        return "".to_string();
    }
}