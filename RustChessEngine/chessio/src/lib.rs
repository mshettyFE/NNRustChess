use constants::*;

pub fn print_locations(board: u64){
  // for loop with numbers
  let mut mask;
  for number in (0..64).rev(){
    mask = 1 << number;
    if (mask & board)!=0 {
      print!("1");
    }
    else{
      print!("0");
    }
    if(number % 8) == 0 {
      print!("\n");
    }
  }
  print!("\n");
}

pub fn print_binary(board: u64){
  // for loop with numbers
  let mut mask;
  for number in (0..64).rev(){
    mask = 1 << number;
    if (mask & board)!=0 {
      print!("1");
    }
    else{
      print!("0");
    }
  }
  print!("\n");
}

pub fn print_void_board(void_board: &[VoidBoardPieceStatus; 144]){
  for i in (0..144).rev(){
    let val = match void_board[i]{
      VoidBoardPieceStatus::EMPTY => 0,
      VoidBoardPieceStatus::OCCUPIED =>  1,
      VoidBoardPieceStatus::INVALID =>  2,
    };
    print!("{} ",val);
    if (i%12)==0{
      print!("\n");
    }
  }
  print!("\n");
}

pub fn print_char_board(char_board: &[char; 64]){
  for number in (0..64).rev(){
    print!("{} ",char_board[number]);
    if (number%8)==0{
      print!("\n");
    }
  }
  print!("\n");
}