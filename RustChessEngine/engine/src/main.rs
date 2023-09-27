use chessio::*;
use constants::*;
use masks::*;

use std::collections::HashMap;

fn main() {
  let KnightMoves: [u64; 64] = gen_knight_masks();
  let KingMoves: [u64;64] = gen_king_masks();
  let PawnMoves:  HashMap<Color,[u64;64]> =  gen_pawn_move_masks();
  let PawnCaptures:  HashMap<Color,[u64;64]> =  gen_pawn_capture_masks();
  print_locations(PawnMoves.get(&Color::BLACK).unwrap()[52]);
  print_locations(PawnCaptures.get(&Color::BLACK).unwrap()[52]);
}