use constants::*;
use std::collections::HashMap;
use std::vec::Vec;

fn test_move_candidates(void_candidates: Vec<usize>, void_bit_mapping: &VoidBitConversion) -> u64{
    let mut output_moves: u64 = 0;
    for potential_move in void_candidates.iter(){
        match void_bit_mapping.void_to_bit(*potential_move){  
            None => (),
            Some(index) => output_moves |= 1 << index,
        }            
    }
  output_moves
}

pub fn gen_knight_masks() -> [u64; 64] {
    let void_bit_mapping: VoidBitConversion = VoidBitConversion::default();
    let mut arr:[u64; 64] = [0; 64];
    for bit_index in 0..64{
// Convert bitboard index to void index
        let void_index: usize = void_bit_mapping.bit_to_void(bit_index).unwrap();
// Create array of possible knight moves from current position on voidboard
        let void_candidates =  Vec::from([void_index+25,void_index+23,void_index+14, void_index+10,
                                            void_index-25,void_index-23,void_index-14, void_index-10]);
        let output_moves: u64 = test_move_candidates(void_candidates, &void_bit_mapping);
        arr[bit_index] = output_moves;    
    }
    arr
}

pub fn gen_king_masks() -> [u64; 64] {
    let void_bit_mapping: VoidBitConversion = VoidBitConversion::default();
    let mut arr:[u64; 64] = [0; 64];
    for bit_index in 0..64{
// Convert bitboard index to void index
        let void_index: usize = void_bit_mapping.bit_to_void(bit_index).unwrap();
// Create array of possible king moves from current position on voidboard
        let void_candidates = Vec::from([void_index+12,void_index+13,void_index+11, void_index+1, void_index-12,void_index-13,void_index-11, void_index-1]);
        let output_moves: u64 = test_move_candidates(void_candidates, &void_bit_mapping);
        arr[bit_index] = output_moves;    
    }
    arr
}

pub fn gen_pawn_move_masks() -> HashMap<Color,[u64;64]> {
    let void_bit_mapping: VoidBitConversion = VoidBitConversion::default();
// Create an empty hashmap for the output
    let mut output = HashMap::<Color,[u64;64]>::new();
  // we will store the possible moves for both white and black seperately
    let mut white_moves: [u64; 64] = [0;64];
    let mut black_moves: [u64; 64] = [0;64];
    for bit_index in 0..64{
        let current = 1 << bit_index;
    // white move generation
        let mut white_output_move: u64 = 0;
        let void_index: usize = void_bit_mapping.bit_to_void(bit_index).unwrap();
        // white pawns can't be on RANKA
        if (current& RANKA) != 0 {
            white_moves[bit_index] = 0;
        }
    // can do double advancements in RANKB for white
        else if (current& RANKB) !=0 {
            let white_void_candidates = Vec::from([void_index+12, void_index+24]);    
            white_output_move = test_move_candidates(white_void_candidates, &void_bit_mapping);
            white_moves[bit_index] = white_output_move;
        }
    // Normal pawn moves otherwise
        else{
            let white_void_candidates = Vec::from([void_index+12]);    
            white_output_move = test_move_candidates(white_void_candidates, &void_bit_mapping);
            white_moves[bit_index] = white_output_move;
        }
    // black move generation
        let mut black_output_move: u64 = 0;
        if (current& RANKH) != 0 {
            black_moves[bit_index] = 0;
        }
        else if (current& RANKG) !=0 {
            let black_void_candidates = Vec::from([void_index-12, void_index-24]);    
            black_output_move = test_move_candidates(black_void_candidates, &void_bit_mapping);
            black_moves[bit_index] = black_output_move;
        }
        else {
            let black_void_candidates = Vec::from([void_index-12]);    
            black_output_move = test_move_candidates(black_void_candidates, &void_bit_mapping);
            black_moves[bit_index] = black_output_move;
        }
    }
    output.insert(Color::WHITE,white_moves);
    output.insert(Color::BLACK,black_moves);
    output
}

pub fn gen_pawn_capture_masks() -> HashMap<Color,[u64;64]> {
    let void_bit_mapping: VoidBitConversion = VoidBitConversion::default();
// Create an empty hashmap for the output
    let mut output = HashMap::<Color,[u64;64]>::new();
  // we will store the possible moves for both white and black seperately
    let mut white_moves: [u64; 64] = [0;64];
    let mut black_moves: [u64; 64] = [0;64];
    for bit_index in 0..64{
        let current = 1 << bit_index;
    // white move generation
        let mut white_output_move: u64 = 0;
        let void_index: usize = void_bit_mapping.bit_to_void(bit_index).unwrap();
        // white pawns can't be on RANKA
        if (current& RANKA) != 0 {
            white_moves[bit_index] = 0;
        }
    // Normal pawn moves otherwise
        else{
            let white_void_candidates = Vec::from([void_index+11,void_index+13]);    
            white_output_move = test_move_candidates(white_void_candidates, &void_bit_mapping);
            white_moves[bit_index] = white_output_move;
        }
    // black move generation
        let mut black_output_move: u64 = 0;
        if (current& RANKH) != 0 {
            black_moves[bit_index] = 0;
        }
        else {
            let black_void_candidates = Vec::from([void_index-11,void_index-13]);    
            black_output_move = test_move_candidates(black_void_candidates, &void_bit_mapping);
            black_moves[bit_index] = black_output_move;
        }
    }
    output.insert(Color::WHITE,white_moves);
    output.insert(Color::BLACK,black_moves);
    output
}