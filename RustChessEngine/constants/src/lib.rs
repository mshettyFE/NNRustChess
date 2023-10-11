use crate::VoidBoardPieceStatus::EMPTY;
use crate::VoidBoardPieceStatus::INVALID;
use crate::VoidBoardPieceStatus::OCCUPIED;

// Ranks 
pub const RANKA: u64 = 255;
pub const RANKB: u64 = 255 << 8;
pub const RANKC: u64 = 255 << 16;
pub const RANKD: u64 = 255 << 24;
pub const RANKE: u64 = 255 << 32;
pub const RANKF: u64 = 255 << 40;
pub const RANKG: u64 = 255 << 48;
pub const RANKH: u64 = 255 << 56;

// Files
pub const FILE8: u64 = 1 + (1<<8) + (1<<16) + (1<<24) + (1<<32) + (1<<40) + (1<<48)  + (1<<56);
pub const FILE7: u64 = FILE8 << 1;
pub const FILE6: u64 = FILE8 << 2;
pub const FILE5: u64 = FILE8 << 3;
pub const FILE4: u64 = FILE8 << 4;
pub const FILE3: u64 = FILE8 << 5;
pub const FILE2: u64 = FILE8 << 6;
pub const FILE1: u64 = FILE8 << 7;


#[derive(Eq)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum VoidBoardPieceStatus{
    EMPTY, OCCUPIED, INVALID
}


#[derive(Eq)]
#[derive(Hash)]
#[derive(PartialEq)]
pub enum Color{
    WHITE,BLACK,
}

#[derive(Eq)]
#[derive(Hash)]
#[derive(PartialEq)]
pub enum SlidingPieceType{
    ROOK,BISHOP,
}

pub const VOID_BOARD: [VoidBoardPieceStatus;144] = 
[
    INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,
    INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,
    INVALID,INVALID,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,INVALID,INVALID,
    INVALID,INVALID,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,INVALID,INVALID,
    INVALID,INVALID,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,INVALID,INVALID,
    INVALID,INVALID,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,INVALID,INVALID,
    INVALID,INVALID,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,INVALID,INVALID,
    INVALID,INVALID,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,INVALID,INVALID,
    INVALID,INVALID,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,INVALID,INVALID,
    INVALID,INVALID,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,EMPTY,INVALID,INVALID,
    INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,
    INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,
];


#[derive(Debug)]
pub struct VoidBitConversion{
    _void: [usize; 64],

}

impl VoidBitConversion{
    pub fn void_to_bit(self: &Self, void_index: usize) -> Option<usize>{
        for index in 0..64 {
            if void_index == self._void[index] {
                return Some(index)
            }
        }
        return None
    }

    pub fn bit_to_void(self: &Self, bit_index: usize) -> Option<usize>{
        if bit_index > 63 {
            return None
        }
        return Some(self._void[bit_index])
    }

}

impl Default for VoidBitConversion { 
    fn default() -> Self {
        VoidBitConversion {
            _void: [
                26,  27,  28,  29,  30,  31,  32,  33,
                38,  39,  40,  41,  42,  43,  44,  45,
                50,  51,  52,  53,  54,  55,  56,  57,
                62,  63,  64,  65,  66,  67,  68,  69,
                74,  75,  76,  77,  78,  79,  80,  81,
                86,  87,  88,  89,  90,  91,  92,  93,
                98,  99,  100, 101, 102, 103, 104, 105,
                110, 111, 112, 113, 114, 115, 116, 117
            ],
      }
    }
}

pub struct MagicNumberData{
    pub _number: u64,
    pub _shift: u64,
}