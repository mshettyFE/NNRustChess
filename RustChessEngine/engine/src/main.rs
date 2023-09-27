use chessio::*;
use constants::*;
use masks::*;

fn main() {
/*
  print_locations(FILE1);
  print_locations(FILE2);
  print_locations(FILE3);
  print_locations(FILE4);
  print_locations(FILE5);
  print_locations(FILE6);
  print_locations(FILE7);
  print_locations(FILE8);
*/

/*
print_locations(TOPLEFTCORNER);
print_locations(TOPRIGHTCORNER);
print_locations(BOTLEFTCORNER);
print_locations(BOTRIGHTCORNER);
*/
  let KnightMoves: [u64; 64] = GenKnightMoves();
  for index in 0..64{
    println!("{}",index);
    print_locations(KnightMoves[index]);
    println!(" ");
  }
}
