
extern crate clap;
use clap::Parser;

use engine::magic::SlidingMoves;
use engine::masks::Masks;
use engine::game_state::GameState;
use engine::chessio::*;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name="NULL")]
#[command(version="0.1")]
#[command(about="Chess Engine in Rust")]
struct Args {
  #[arg(short, long)]
  train: bool,  
}

fn main() {
  let masks: Masks = Masks::default();
  let mut board: GameState = GameState::default();
  let mut sliding_moves: SlidingMoves = SlidingMoves::default();
  sliding_moves.initialize(&masks);
  let args = Args::parse();
  if args.train==true {
    let (mut stock_stdin, mut stock_stout) = spawn_stock();
    match set_position("r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 b KQ g6 0 21", &mut stock_stdin, &mut stock_stout) {
        Ok(()) => (),
        Err(err_msg) => panic!("{}", err_msg),
    }
//    println!("eval:{}", eval_pos(&mut stock_stdin, &mut stock_stout));
      print_stock(&mut stock_stdin, &mut  stock_stout);
  }
  else{
    match board.parse_fen("r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 w KQ g6 0 21"){
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
  }
}
