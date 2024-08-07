use crate::constants::*;


#[derive(Clone)]
// Implement the state of  a given color. Gives location of all pieces, and then on a individual level
pub struct SideState{
  pub _occupied: u64,
  pub _king: u64,
  pub _queen: u64,
  pub _knight: u64,
  pub _bishop: u64,
  pub _rook: u64,
  pub _pawn: u64,
  pub _castling: u8,
}
  
impl Default for SideState { 
  fn default() -> Self {
    // 0x0F is is all castling rights enabled
    SideState {_occupied: 0, _king: 0, _queen: 0, _knight: 0, _bishop: 0, _pawn: 0 , _rook: 0 , _castling: 0}
  }
}
  
impl SideState{
// convert index to bitboard
  fn gen_location(location :usize) -> u64{
    if location >63 {
      panic!("Out of bounds in SideState");
    }
    return 1<< location;
  }
// update occupied and piece bitboard
  pub fn update_king(self: &mut Self, location: usize){
    let temp = Self::gen_location(location);
    self._king |= temp;
    self._occupied |= temp;
    self._castling &= CASTLING_ALL ^ CASTLING_KING;
    self._castling &= CASTLING_ALL ^ CASTLING_QUEEN;
  }
  pub fn update_queen(self: &mut Self, location: usize){
    let temp = Self::gen_location(location);
    self._queen |= temp;
    self._occupied |= temp;
  }
  pub fn update_knight(self: &mut Self, location: usize){
    let temp = Self::gen_location(location);
    self._knight |= temp;
    self._occupied |= temp;
  }
  pub fn update_bishop(self: &mut Self, location: usize){
    let temp = Self::gen_location(location);
    self._bishop |= temp;
    self._occupied |= temp;
  }
  pub fn update_rook(self: &mut Self, location: usize){
    let temp = Self::gen_location(location);
    if(temp & KING_CORNERS) == 0{
      self._castling &= CASTLING_ALL ^ CASTLING_KING;
    }
    if(temp & QUEEN_CORNERS) == 0{
      self._castling &= CASTLING_ALL ^ CASTLING_QUEEN;
    }
    self._rook |= temp;
    self._occupied |= temp;
  }
  pub fn update_pawn(self: &mut Self, location: usize){
    let temp = Self::gen_location(location);
    self._pawn |= temp;
    self._occupied |= temp;
  }

// special function to extract the given sides king. guaranteed to have only 1
  pub fn extract_king(self: &Self) -> Result<u64, String> {
    let output = (self._king as i128) & -(self._king as i128);
    match output {
      0 => Err("No king found".to_string()),
      _ => Ok(output as u64),
    }
  }
// extract piece type at location
  fn extract_location(self: & Self, location: usize) -> Result<char, String> {
    if location > 63 {
      return Err("Invalid location to access SideState".to_string());
    }
    let mut output: char = '-';
    let position: u64 = Self::gen_location(location);
    if(self._king & position) != 0 {
      match output{
        '-' => output = 'k',
        _ => return Err("Overlapping pieces at ".to_owned() + &location.to_string()),
      }
    }
    if(self._queen & position) != 0 {
      match output{
        '-' => output = 'q',
        _ => return Err("Overlapping pieces at ".to_owned() + &location.to_string()),
      }
    }
    if(self._pawn & position) != 0 {
      match output{
        '-' => output = 'p',
        _ => return Err("Overlapping pieces at ".to_owned() + &location.to_string()),
      }
    }
    if(self._knight & position) != 0 {
      match output{
        '-' => output = 'n',
        _ => return Err("Overlapping pieces at ".to_owned() + &location.to_string()),
      }
    }
    if(self._bishop & position) != 0 {
      match output{
        '-' => output = 'b',
        _ => return Err("Overlapping pieces at ".to_owned() + &location.to_string()),
      }
    }
    if(self._rook & position) != 0 {
      match output{
        '-' => output = 'r',
        _ => return Err("Overlapping pieces at ".to_owned() + &location.to_string()),
      }
    }
    return Ok(output);
  }

  // convert Side State to character array
  pub fn to_char_board(self: &Self) -> Result<[char; 64],String>{
    let mut output: [char; 64] = ['-'; 64];
    for number in 0..64{
      match self.extract_location(number){
        Ok(chr) => output[number] = chr,
        Err(msg) => return Err(msg),
      }
    }
    Ok(output)
  }
}

