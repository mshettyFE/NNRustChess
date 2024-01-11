use constants::*;
use std::collections::HashMap;
use std::vec::Vec;

// Move Masks for all the piece types.
// This means that on an empty board, each bitboard represents all valid moves of a given piece
pub struct Masks{
    pub _knight_mask: [u64; 64],
    pub _rook_mask: [u64; 64],
    pub _bishop_mask: [u64; 64],
    pub _king_mask: [u64; 64],
    pub _pawn_moves_mask: HashMap<Color,[u64;64]>,
    pub _pawn_capture_mask: HashMap<Color,[u64;64]>,
}

impl Default for Masks{
// generate masks for all piece types
  fn default() -> Self {
    Masks{ _knight_mask: gen_knight_masks(),
        _rook_mask: gen_rook_masks(),
        _bishop_mask: gen_bishop_masks(),
        _king_mask: gen_king_masks(),
        _pawn_moves_mask: gen_pawn_move_masks(),
        _pawn_capture_mask: gen_pawn_capture_masks(),
    }
  }
}

  fn gen_knight_masks( ) -> [u64; 64] {
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

    fn gen_rook_masks( ) -> [u64; 64]{
    let void_bit_mapping: VoidBitConversion = VoidBitConversion::default();
    let mut output: [u64; 64] = [0;64];
    let sliding_directions: Vec<isize> = Vec::from([12,-12,1,-1]);
    for bit_index in 0..64 {
        let void_index: usize = void_bit_mapping.bit_to_void(bit_index).unwrap();
        output[bit_index] = test_slide_move_candidates(void_index, &sliding_directions, &void_bit_mapping);
    }
    output
  }

  fn gen_bishop_masks( ) -> [u64; 64]{
      let void_bit_mapping: VoidBitConversion = VoidBitConversion::default();
      let mut output: [u64; 64] = [0;64];
      let sliding_directions: Vec<isize> = Vec::from([13,11,-13,-11]);
      for bit_index in 0..64 {
          let void_index: usize = void_bit_mapping.bit_to_void(bit_index).unwrap();
          output[bit_index] = test_slide_move_candidates(void_index, &sliding_directions, &void_bit_mapping);
      }
      output
  }

  fn gen_king_masks( ) -> [u64; 64] {
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

  fn gen_pawn_move_masks( ) -> HashMap<Color,[u64;64]> {
    // Normal pawn moves. Takes into account double moves on first advancement
      let void_bit_mapping: VoidBitConversion = VoidBitConversion::default();
      // Create an empty hashmap for the output
      let mut output = HashMap::<Color,[u64;64]>::new();
      // we will store the possible moves for both white and black seperately
      let mut white_moves: [u64; 64] = [0;64];
      let mut black_moves: [u64; 64] = [0;64];
      for bit_index in 0..64{
          let current = 1 << bit_index;
      // white move generation
          let white_output_move: u64;
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
          let black_output_move: u64;
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

  fn gen_pawn_capture_masks( ) -> HashMap<Color,[u64;64]> {
    // generate capture moves for pawns
      let void_bit_mapping: VoidBitConversion = VoidBitConversion::default();
      // Create an empty hashmap for the output
      let mut output = HashMap::<Color,[u64;64]>::new();
      // we will store the possible moves for both white and black seperately
      let mut white_moves: [u64; 64] = [0;64];
      let mut black_moves: [u64; 64] = [0;64];
      for bit_index in 0..64{
          let current = 1 << bit_index;
      // white move generation
          let white_output_move: u64;
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
          let black_output_move: u64;
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

  fn test_move_candidates( void_candidates: Vec<usize>, void_bit_mapping: &VoidBitConversion) -> u64{
    // helper function to make make all void board positions to a single bitboard. Used for non-sliding pieces (Pawn, Knight, King)
    let mut output_moves: u64 = 0;
        for potential_move in void_candidates.iter(){
        // Check if you can convert the void index to a valid bit index. If you can. Then OR in that position to output
            match void_bit_mapping.void_to_bit(*potential_move){  
                None => (),
                Some(index) => output_moves |= 1 << index,
            }            
        }
        output_moves
    }

    fn test_slide_move_candidates( current_void_square: usize, slide_directions: &Vec<isize>, void_bit_mapping: &VoidBitConversion) -> u64{
        // helper function to make make all void board positions to a single bitboard. For sliding pieces (Rook, Bishop, Queen)
        let mut output_moves: u64 = 0;
        // probe each provided direction in void space
        for direction in slide_directions.iter(){
            // reset starting square
            let mut current = current_void_square as isize;
            let mut inside: bool = true;
            while inside{
        // project out the vector, and see if positive number. If not, then exit loop
                current += direction;
                if current < 0 {
                    break;
                }
        // see if you can match the void index to a bitboard index. If you can, then OR with output, otherwise, set loop condition to false
        match void_bit_mapping.void_to_bit(current as usize){  
            None => inside = false,
            Some(index) => output_moves |= 1 << index,
            }
        }
        }
        output_moves
    }

