use constants::*;
use std::str;

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
    let mut val =  0;
    match void_board[i]{
      VoidBoardPieceStatus::EMPTY => val = 0,
      VoidBoardPieceStatus::OCCUPIED => val = 1,
      VoidBoardPieceStatus::INVALID => val = 2,
    }
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

pub fn loc_to_board(location: &str) -> Result<u64,String> {
  // Takes a two character location in rank-file notation, and convert it to a u64 board position
  let mut loc = location.trim().to_lowercase();  
  let mut output: u64  = 0;
  if loc.len() != 2{
    return Err(loc.to_owned()+ "is an invalid location");
  }
  let first_char = loc.chars().nth(0).unwrap();
  match  first_char{
    'a' => output |= RANKA,
    'b' => output |= RANKB,
    'c' => output |= RANKC,
    'd' => output |= RANKD,
    'e' => output |= RANKE,
    'f' => output |= RANKF,
    'g' => output |= RANKG,
    'h' => output |= RANKH,
    first_char => {
      return Err("Invalid Rank ".to_string()+ " for " + &first_char.to_string());
    }
  }
  let second_char = loc.chars().nth(1).unwrap();
  match  second_char{
    '1' => output &= FILE1,
    '2' => output &= FILE2,
    '3' => output &= FILE3,
    '4' => output &= FILE4,
    '5' => output &= FILE5,
    '6' => output &= FILE6,
    '7' => output &= FILE7,
    '8' => output &= FILE8,
    second_char => {
      return Err("Invalid File".to_string()+ " for " + &second_char.to_string());
    }
  }
  Ok(output)
}