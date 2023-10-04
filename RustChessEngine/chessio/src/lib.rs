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