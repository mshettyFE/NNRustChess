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
  let temp = bitboard_to_voidboard(BishopMoves[0]);
  print_locations(BishopMoves[0]);
  print_void_board(&temp);
  let temp2 = voidboard_to_bitboard(&temp);
  print_locations(temp2);
}