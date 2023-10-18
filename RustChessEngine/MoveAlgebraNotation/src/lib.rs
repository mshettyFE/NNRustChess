use constants::*;

struct MoveAN{
    pub _start: u64,
    pub _end: u64,
    pub _type: PieceType,
    pub _promotion: PieceType
}

impl Default for MoveAN{
    fn default() -> Self {
        MoveAN{_start: 0, _end: 0, _type: PieceType::NONE, _promotion: PieceType::NONE}
    }
}