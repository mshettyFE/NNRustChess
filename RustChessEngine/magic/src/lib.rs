use chessio::*;
use constants::*;
use masks::*;
use std::collections::HashMap;
use rand::Rng;

pub struct SlidingMoves{
  pub _rook_hash_map: HashMap<u64,u64>,
  pub _bishop_hash_map: HashMap<u64,u64>,
}

impl Default for SlidingMoves{
  fn default() -> Self {
    SlidingMoves {_rook_hash_map: HashMap::<u64,u64>::default(),  _bishop_hash_map: HashMap::<u64,u64>::default()  }
  }
}

impl SlidingMoves{
  pub fn initialize(self: &mut Self, mask: &Masks){
    self._rook_hash_map =  gen_sliding_rook_moves(&mask._rook_mask);
    self._bishop_hash_map =  gen_sliding_bishop_moves(&mask._bishop_mask)
  }
  pub fn iterate_moves(self: &Self){
    println!("Test");
    for (key, value) in self._rook_hash_map.clone().into_iter() {
      println!("Rook {} {}", key, value);
      print_locations(key);
      print_locations(value);
    }
    for (key, value) in self._bishop_hash_map.clone().into_iter() {
      println!("Bishop {} {}", key, value);
      print_locations(key);
      print_locations(value);
    }
  }
  pub fn get_rook_move(self: &Self, occupied: u64, index: usize, masks: &Masks) -> u64{
    let key: u64 =  occupied & masks._rook_mask[index];
    let  rook_slide_move: u64 = self._rook_hash_map[&key];
    rook_slide_move ^ (rook_slide_move & occupied)
  }
  
  pub fn get_bishop_move(self: &Self, occupied: u64, index: usize, masks: &Masks) -> u64{
    let key: u64 = occupied & masks._bishop_mask[index];
    let  bishop_slide_move: u64 = self._bishop_hash_map[&key];
    bishop_slide_move ^ (bishop_slide_move & occupied)
  }
}

fn permutate_board(board: u64) -> Vec<u64>{
  // Give a board state, returns a vector if size 2^M where M is the number of on bits
  // The vector is filled with all possible permutations of on and off of the set bits of board
  // Used in sliding move generation

  // Find which bits are set in the board
  let mut temp: u64 = board;

  let mut set_bits: Vec<u64> = Vec::new();
  temp = board;
  // LSB = Least significant bit 
  let mut LSB: u64 = 0;
  while(temp!=0){
  // Extract LSB 
    LSB  = !(temp&(temp-1))&temp;
    set_bits.push(LSB);   
  // Use XOR to eliminate LSB from board state
    temp ^= LSB;
  }

  // Get total number of set bits
    let mut count: usize  = set_bits.len();

    let mut output: Vec<u64> = Vec::new();
    // If no bits are set, then return immediately, since output should have length of 1
    if(count==0){
      output.push(0);
      return output
    }


  // Calculate maximum permutations
    let max_permutations:usize = 1<<count;

  // Fill the output with 2^count values
    let mut current: usize;
    let mut output_value: u64;
    for i in 0..max_permutations {
      output_value = 0;
      current = i;
      for index in 0..count{
  // Check if current[index]==1
        if ((current >> index) & 1) == 1 {
  // If there is a 1 present, include that index 
          output_value |= set_bits[index];  
        }
      }
      output.push(output_value);
    }

    output
}

fn test_slide_move_candidates_filled_board(current_void_board_square: usize, void_board_rep: &[VoidBoardPieceStatus; 144], sliding_directions: &Vec<isize>) -> u64{
    let void_bit_mapping: VoidBitConversion = VoidBitConversion::default();
    let mut output_moves: u64 = 0;
    let mut current: isize = current_void_board_square as isize;
  // probe each provided direction in void space
    for direction in sliding_directions.iter(){
      // reset starting square
      current = current_void_board_square as isize;
      let mut inside: bool = true;
      while inside{
  // project out the vector, and see if positive number. If not, then exit loop
          current += direction;
          if(current < 0){
              break;
          }
          match(void_board_rep[current as usize]){
            VoidBoardPieceStatus::EMPTY =>{
              match void_bit_mapping.void_to_bit(current as usize){  
                None => (inside = false),
                Some(index) => output_moves |= 1 << index,
            }
          },
            VoidBoardPieceStatus::OCCUPIED =>{
                match void_bit_mapping.void_to_bit(current as usize){  
                  None => (inside = false),
                  Some(index) => output_moves |= 1 << index,
              }
              break;
            },
            VoidBoardPieceStatus::INVALID =>{
                break;
            },
          }
      }
    }
    output_moves
}

fn gen_rook_moves(board: u64, location: usize) -> u64{
  let void_bit_mapping: VoidBitConversion = VoidBitConversion::default();
  if(location > 63){
      panic!("index out of range");
  }
  let void_position = void_bit_mapping.bit_to_void(location).unwrap();
  let piece_position = 1<<location;
  // XOR removes piece from board
  let other_pieces = board ^ piece_position;
  // Transform to void space
  let mut board_in_void  = void_bit_mapping.bitboard_to_voidboard(other_pieces);
  let sliding_directions: Vec<isize> = Vec::from([12,-12,1,-1]);
  test_slide_move_candidates_filled_board(void_position, &board_in_void, &sliding_directions)
}

fn gen_bishop_moves(board: u64, location: usize) -> u64{
  let void_bit_mapping: VoidBitConversion = VoidBitConversion::default();
  if(location > 63){
    panic!("index out of range");
  }
  let void_position = void_bit_mapping.bit_to_void(location).unwrap();
  let piece_position = 1<<location;
  // XOR removes piece from board
  let other_pieces = board ^ piece_position;
  // Transform to void space
  let mut board_in_void  = void_bit_mapping.bitboard_to_voidboard(other_pieces);
  let sliding_directions: Vec<isize> = Vec::from([13,11,-13,-11]);
  test_slide_move_candidates_filled_board(void_position, &board_in_void, &sliding_directions)
}

/*
fn calc_magic_number(initial_state: &u64, final_state: u64, shift: u64, hash_map: &HashMap<u64, u64> ) -> u64{
  if(final_state ==0){
    panic!("Final state can't be empty, since captures are allowed!");
  }
  let mut rng = rand::thread_rng();
  let mut  roll: u64 = 0;
  let output: u64  = 0;
  let mut index: u64 = 0;
  while(1==1){
    roll = rng.gen();
    index = (roll.wrapping_mul(*initial_state))>>shift;
    if(hash_map.contains_key(&index)){
      let stored_final_state = hash_map.get(&index).unwrap();
      if(*stored_final_state==final_state){
        break;
      }
    }
    else{
      break;
    }
  }
  roll
}
*/

fn gen_sliding_rook_moves(RookMasks: &[u64; 64]) -> HashMap<u64, u64> {
  let mut output: HashMap<u64, u64> = Default::default();
  for location in 0..64{
    let mut rookmask = RookMasks[location];
    if(rookmask.leading_zeros()==64){
      panic!("Mask is zero!");
    }
    let permutations = permutate_board(rookmask);
    for initial_state in permutations.iter(){
      let mut final_state:u64 = 0;
      final_state = gen_rook_moves(*initial_state, location);
      output.insert(*initial_state,final_state);
    }  
  }
  output
}

fn gen_sliding_bishop_moves(BishopMasks: &[u64;64]) -> HashMap<u64, u64 > {
  let mut output: HashMap<u64, u64> = Default::default();
  for location in 0..64{
    let bishopmask = BishopMasks[location];
    if(bishopmask.trailing_zeros()==64){
      panic!("Mask is zero!");
    }
    let permutations = permutate_board(bishopmask);
    for initial_state in permutations.iter(){
       let mut final_state:u64 = 0;
      final_state = gen_rook_moves(*initial_state, location);
      output.insert(*initial_state,final_state);
    }  
  }
  output
}
