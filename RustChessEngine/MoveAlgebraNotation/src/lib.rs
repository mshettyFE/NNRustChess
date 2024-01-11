use constants::*;
use chessio::*;

// encode Move in a manner that can be understood by engine
pub struct MoveAN{
    pub _start: u64, // starting position of piece
    pub _end: u64, // ending position of piece
    pub _color: Color, // color of piece
    pub _type: PieceType, // what piece is being moved
    pub _promotion: PieceType, // for pawns, what do you promote to?
    pub _capture: bool, // is this a capture?
    pub _enpassant: bool, // is this an enpassant?
    pub _castling: Castling, // is this a castle (all other members are then ignored)
    pub _index: u16, // the index in the output vector (TBD)
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