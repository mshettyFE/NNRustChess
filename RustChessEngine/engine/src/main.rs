use chessio::*;
use constants::*;
use masks::*;
use magic::*;

use std::collections::HashMap;

fn main() {
  let _KnightMoves: [u64; 64] = gen_knight_masks();
  let _KingMoves: [u64;64] = gen_king_masks();
  let _PawnMoves:  HashMap<Color,[u64;64]> =  gen_pawn_move_masks();
  let _PawnCaptures:  HashMap<Color,[u64;64]> =  gen_pawn_capture_masks();
  let _RookMasks: [u64;64] = gen_rook_masks();
  let _BishopMasks: [u64; 64] = gen_bishop_masks();
  let mask  = gen_all_sliding_moves(_RookMasks, _BishopMasks);
  let mut count = 0;
  /*  
  for item in mask.values(){
    println!("{}",count);
    count += 1;
    print_locations(*item);
  }
  */
}