use chessio::*;

pub fn permutate_board(board: u64) -> Vec<u64>{
// Give a board state, returns a vector if size 2^M where M is the number of on bits
// The vector is filled with all possible permutations of on and off of the set bits of board
// Used in sliding move generation

// Find which bits are set in the board
  let mut temp: u64 = board;

  let mut set_bits: Vec<u64> = Vec::new();
  temp = board;
// LSB = Least significant bit 
  let mut LSB: u64 = 0;
  while(temp!=0){
// Extract LSB 
    LSB  = !(temp&(temp-1))&temp;
    set_bits.push(LSB);   
// Use XOR to eliminate LSB from board state
    temp ^= LSB;
  }

// Get total number of set bits
  let mut count: usize  = set_bits.len();

  let mut output: Vec<u64> = Vec::new();
  // If no bits are set, then return immediately, since output should have length of 1
  if(count==0){
    output.push(0);
    return output
  }


// Calculate maximum permutations
  let max_permutations:usize = 1<<count;

// Fill the output with 2^count values
  let mut current: usize;
  let mut output_value: u64;
  for i in 0..max_permutations {
    output_value = 0;
    current = i;
    for index in 0..count{
// Check if current[index]==1
      if ((current >> index) & 1) == 1 {
// If there is a 1 present, include that index 
        output_value |= set_bits[index];  
      }
    }
    output.push(output_value);
  }

  output
}