use chessio::*;
use constants::*;
use masks::*;
use magic::*;

use std::collections::HashMap;

fn main() {
  let KnightMoves: [u64; 64] = gen_knight_masks();
  let KingMoves: [u64;64] = gen_king_masks();
  let PawnMoves:  HashMap<Color,[u64;64]> =  gen_pawn_move_masks();
  let PawnCaptures:  HashMap<Color,[u64;64]> =  gen_pawn_capture_masks();
  let RookMoves: [u64;64] = gen_rook_masks();
  let BishopMoves: [u64; 64] = gen_bishop_masks();
  let mut t: Vec<u64> = vec![0];
  for i in 0..64{
    permutate_board(RookMoves[i]);
  }
  for i in 0..64{
    t = permutate_board(BishopMoves[i]);
  }
}