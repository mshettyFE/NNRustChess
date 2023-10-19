use GameState::GameState;
use masks::*;
use magic::*;
use tch::Tensor;

fn main() {
  let masks: Masks = Masks::default();
  let mut board: GameState = GameState::default();
  let mut sliding_moves: SlidingMoves = SlidingMoves::default();
  sliding_moves.initialize(&masks);
  match board.parse_fen("r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 b KQ g6 0 21"){
    Ok(()) => (),
    Err(e) => return println!("{}",e),
  };
  board.print_board();
  match board.check_board_legality(&masks, &sliding_moves) {
    Ok(bol) => match bol{
      true => println!("Yes"),
      false => println!("No"),
    },
    Err(msg) => println!("{}",msg),
  }
    let t = Tensor::from_slice(&[3, 1, 4, 1, 5]);
    let t = t * 2;
    t.print();
}
