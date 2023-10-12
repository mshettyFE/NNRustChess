use chessio::*;
use constants::*;
use masks::*;
use magic::*;

fn main() {
/*
  let _KnightMoves: [u64; 64] = gen_knight_masks();
  let _KingMoves: [u64;64] = gen_king_masks();
  let _PawnMoves:  HashMap<Color,[u64;64]> =  gen_pawn_move_masks();
  let _PawnCaptures:  HashMap<Color,[u64;64]> =  gen_pawn_capture_masks();
  let _RookMasks: [u64;64] = gen_rook_masks();
  let _BishopMasks: [u64; 64] = gen_bishop_masks();
  let mask  = gen_all_sliding_moves(_RookMasks, _BishopMasks);
*/
  let mut board: GameState = GameState::default();
  match board.ParseFEN("r3k2r/ p1pppppp/ n7/ 8/ 8/ 8/ P2PPPpP/ R3K2R w KQkq - 0 5"){
    Ok(()) => (),
    Err(e) => return println!("{}",e),
  };
  println!("test");
  print_locations(board._white._rook);
  println!("test");
  board.print_board();
}