use constants::*;
use std::str;
use masks::*;
use magic::*;
use chessio::*; 


#[derive(Clone)]
pub struct SideState{
  pub _occupied: u64,
  pub _king: u64,
  pub _queen: u64,
  pub _knight: u64,
  pub _bishop: u64,
  pub _rook: u64,
  pub _pawn: u64,
}
  
impl Default for SideState { 
  fn default() -> Self {
    SideState {_occupied: 0, _king: 0, _queen: 0, _knight: 0, _bishop: 0, _pawn: 0 , _rook: 0 }
  }
}
  
impl SideState{
  fn gen_location(location :usize) -> u64{
    if(location >63){
      panic!("Out of bounds in SideState");
    }
    return 1<< location;
  }
  pub fn update_king(self: &mut Self, location: usize){
    let temp = Self::gen_location(location);
    self._king |= temp;
    self._occupied |= temp;
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
    self._rook |= temp;
    self._occupied |= temp;
  }
  pub fn update_pawn(self: &mut Self, location: usize){
    let temp = Self::gen_location(location);
    self._pawn |= temp;
    self._occupied |= temp;
  }
  pub fn extract_king(self: &Self) -> Result<u64, String> {
    let output = (self._king as i128) & -(self._king as i128);
    match output {
      0 => Err("No king found".to_string()),
      _ => Ok(output as u64),
    }
  }
  fn extract_location(self: & Self, location: usize) -> Result<char, String> {
    if(location > 63){
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

pub struct GameState{
  pub _white: SideState,
  pub _black: SideState,
  pub _current_move: Color,
  pub _castling: i8,
  pub _enpassant: u64, 
  pub _halfmove: u64,
  pub _fullmove: u64,
}

impl Default for GameState {
  fn default() -> Self {
    GameState{ _white: SideState::default(),_black: SideState::default(), _current_move: Color::WHITE,  _castling: 0xF, _enpassant: 0, _halfmove:0, _fullmove: 0}
  }
}

impl GameState{

  fn get_attacks(self: &Self, masks: &Masks, sliding: &SlidingMoves) -> Result<u64, String> {
    let current_side: &SideState = match self._current_move{
      Color::WHITE => &self._white,
      Color::BLACK => &self._black,
    };
    let occupied: u64 = self.find_occupied().unwrap();
    let mut temp_current_occupied: u64 =  current_side._occupied;
    let mut output: u64 = 0;
    while (temp_current_occupied != 0){
      let mut index: usize = temp_current_occupied.trailing_zeros() as usize;
      let mut lowest: u64 = temp_current_occupied & (1 << index);
      if (lowest & current_side._king) != 0{
        // find overlaps with your own pieces, and remove this overlap from attacks
        let mut attack: u64 = masks._king_mask[index] ^ (masks._king_mask[index] & current_side._occupied);
        output |= attack;
      }
      else if (lowest & current_side._knight) != 0{
        let mut attack: u64 = masks._knight_mask[index] ^ (masks._knight_mask[index] & current_side._occupied);
        output |= attack;
      }
      else if (lowest & current_side._bishop) != 0{
        output |=  sliding.get_bishop_move(current_side._occupied, index, masks);
      }
      else if (lowest & current_side._rook) != 0{
        output |=  sliding.get_rook_move(current_side._occupied, index, masks);
      }
      else if (lowest & current_side._queen) != 0{
        output |=  sliding.get_bishop_move(current_side._occupied, index, masks);
        output |=  sliding.get_rook_move(current_side._occupied, index, masks);
      }
      else if (lowest & current_side._pawn) != 0{
        let mut pawn_attack_mask = masks._pawn_capture_mask[&self._current_move][index];
        let mut pawn_attack = pawn_attack_mask ^ (pawn_attack_mask & (current_side._occupied) );
        if (pawn_attack_mask & self._enpassant) != 0{
          pawn_attack |= self._enpassant;
        }
        output |= pawn_attack;
      }
      else{
        return Err(index.to_string()+" is occupied, but no piece bitboard contains it");
      }
      temp_current_occupied ^= lowest;
    }
    Ok(output)
  }

  fn find_occupied(self: &Self) ->Result<u64, String> {
    let white_oc = self._white._occupied;
    let black_oc = self._black._occupied;
    if (white_oc & black_oc) != 0{
      return Err("Overlapping white and black pieces".to_string());
    }
    Ok(white_oc | black_oc)
  }

  pub fn ParseFEN(self: &mut Self, FEN: &str ) -> Result<(), String> {
    // break up by / to seperate ranks
    let mut parts = FEN.split("/");
    if (parts.clone().count() != 8){
      return Err("Invalid number of Ranks".to_owned());
    }
    let mut  sections: Vec<&str> = parts.collect();
    // break up last part and seperate by space
    let mut end_part = sections[7].trim().split(" ");
    if (end_part.clone().count() != 6){
      return Err("Invalid number of partitions at end of FEN notation".to_owned());
    }
    let mut  end_section: Vec<&str> = end_part.collect();
    // transfer first element of end_section to last element of section
    sections.remove(7);
    sections.push(end_section[0]);
    end_section.remove(0);
    // run through each rank in sections, and update state accordingly
    let mut cur_index:usize = 63;
    'outer: for temp_rank in sections.iter(){
      let rank = temp_rank.trim();
      let mut occupied_spaces = 0;
      for c in rank.chars(){
        let mut subtraction = 0;
        match c{
          'r' => {
            self._black.update_rook(cur_index);
            subtraction = 1;
            occupied_spaces += 1;
          },
          'n' => {
            self._black.update_knight(cur_index);
            subtraction = 1;
            occupied_spaces += 1;
          },
          'b' => {
            self._black.update_bishop(cur_index);
            subtraction = 1;
            occupied_spaces += 1;
          },
          'q' => {
            self._black.update_queen(cur_index);
            subtraction = 1;
            occupied_spaces += 1;
          },
          'k' => {
            self._black.update_king(cur_index);
            subtraction = 1;
            occupied_spaces += 1;
          },
          'p' => {
            self._black.update_pawn(cur_index);
            subtraction = 1;
            occupied_spaces += 1;
          },
          'R' => {
            self._white.update_rook(cur_index);
            subtraction = 1;
            occupied_spaces += 1;
          },
          'N' => {
            self._white.update_knight(cur_index);
            subtraction = 1;
            occupied_spaces += 1;
          },
          'B' => {
            self._white.update_bishop(cur_index);
            subtraction = 1;
            occupied_spaces += 1;
          },
          'Q' => {
            self._white.update_queen(cur_index);
            subtraction = 1;
            occupied_spaces += 1;
          },
          'K' => {
            self._white.update_king(cur_index);
            subtraction = 1;
            occupied_spaces += 1;
          },
          'P' => {
            self._white.update_pawn(cur_index);
            subtraction = 1;
            occupied_spaces += 1;
          },
          '8' => {
            occupied_spaces += 8;
            subtraction = 8;
          },
          '7' => {
            occupied_spaces += 7;
            subtraction = 7;
          },
          '6' => {
            occupied_spaces += 6;
            subtraction = 6;
          },
          '5' => {
            occupied_spaces += 5;
            subtraction = 5;
          },
          '4' => {
            occupied_spaces += 4;
            subtraction = 4;
          },
          '3' => {
            occupied_spaces += 3;
            subtraction = 3;
          },
          '2' => {
            occupied_spaces += 2;
            subtraction = 2;
          },
          '1' => {
            occupied_spaces += 1;
            subtraction = 1;
          },
          _ => {
            let msg: String = "Problem here: ".to_owned()+rank.clone() + "Invalid character";
            return Err(msg);
          }
        }
        if occupied_spaces >8 {
          let msg: String = "Problem here: ".to_owned()+rank.clone();
          return Err(msg);
        }
        if(cur_index == 0){
          break 'outer;
        }
        else if(cur_index as isize - subtraction)  >= 0 {
          cur_index -= subtraction as usize;
        }
        else{
          let msg: String = "Problem here: ".to_owned()+rank.clone() + " Out of bounds";
          return Err(msg);
        }
      }
      occupied_spaces = 0;
    }
    // Who is to move currently
    let binding: String = end_section[0].trim().to_lowercase();
    let current_player: &str = binding.as_str();
    match current_player{
      "w" => {
        self._current_move = Color::WHITE;
      }
      "b" => {
        self._current_move = Color::BLACK;
      }
      current_player => {
        let msg: String = "Problem here: ".to_owned() + current_player+ " Must be w or b";
        return Err(msg);
      }
    }
    // Castling rights
    let castle: &str =  end_section[1].trim();
    let mut  final_castle: i8 = 0;
    let castle_len = castle.len();
    if (castle_len == 0){
      let msg: String = "Problem here: ".to_owned() + castle+ " Must be non-empty";
      return Err(msg);
    }
    else if(castle_len == 1){
      if(castle == "-"){}
      match (castle) {
        "k" => final_castle |= Castling::WHITE_KING as i8,
        "K" => final_castle |= Castling::BLACK_KING as i8,
        "q" => final_castle |= Castling::WHITE_QUEEN as i8,
        "Q" => final_castle |= Castling::BLACK_QUEEN as i8,
        castle => {
          let msg: String = "Problem here: ".to_owned() + &castle.to_string() + " Characters must be either k, K, q, or Q";
          return Err(msg);    
        }    
      }
    }
    else if(castle_len > 4){
      let msg: String = "Problem here: ".to_owned() + castle+ " Must be at most 4 characters of either k, K, q, or Q";
      return Err(msg);
    }
    else{
      for castle_char in castle.chars() {
        match (castle_char) {
          'k' => final_castle |= Castling::WHITE_KING as i8,
          'K' => final_castle |= Castling::BLACK_KING as i8,
          'q' => final_castle |= Castling::WHITE_QUEEN as i8,
          'Q' => final_castle |= Castling::BLACK_QUEEN as i8,
          castle_char => {
            let msg: String = "Problem here: ".to_owned() + &castle_char.to_string() + " Characters must be either k, K, q, or Q";
            return Err(msg);    
          }    
        }
      }  
    }
    self._castling = final_castle;
    // enpassant

    let enpassant: &str =  end_section[2].trim();
    let mut final_enpass: u64  = 0;
    let enpassant_len = enpassant.len();
    if enpassant.len() == 1{
      if(enpassant == "-"){}
      else{
        let msg: String = "Problem here: ".to_owned() + enpassant+ " Must be either - or a board location in rank-file notation";
        return Err(msg);
      }  
    }
    else if enpassant.len() == 2{
      match loc_to_board(enpassant) {
        Ok(board) => final_enpass = board,
        Err(msg) => return Err(msg),
      }
    }
    else{
      let msg: String = "Problem here for enpassant: ".to_owned() + enpassant+ " Must be either - or a board location in rank-file notation";
      return Err(msg);
    }
    self._enpassant = final_enpass;
    // half moves
    let half_move_str: &str =  end_section[3].trim();
    match half_move_str.parse::<u64>(){
      Ok(half_moves) => self._halfmove = half_moves,
      Err(msg) => return Err("Can't convert half move value to unsigned integer".to_string()),
    }
    // full moves
    let full_move_str: &str =  end_section[4].trim();
    match full_move_str.parse::<u64>(){
      Ok(full_moves) => self._fullmove = full_moves,
      Err(msg) => return Err("Can't convert full move value to unsigned integer".to_string()),
    }
    Ok(())
  }

  pub fn print_board(self: &Self){
    let mut white_output: [char; 64];
    let mut black_output: [char; 64];
    let mut final_output: [char; 64] = ['-'; 64];
    match self._black.to_char_board() {
      Ok(black_otpt) => black_output = black_otpt,
      Err(msg) => panic!("Invalid board State for black"),
    }
    match self._white.to_char_board() {
      Ok(white_otpt) => white_output = white_otpt,
      Err(msg) => panic!("Invalid board State for white"),
    }
    for i in 0..64{
      if(white_output[i] == '-' && black_output[i] == '-'){
        final_output[i] = '-';
      }
      else if(white_output[i] != '-' && black_output[i] == '-'){
        final_output[i] = white_output[i].to_ascii_uppercase();
      }
      else if(white_output[i] == '-' && black_output[i] != '-'){
        final_output[i] = black_output[i];
      }
      else{
        panic!("Invalid board state. Overlapping pieces");
      }
    }
    print_char_board(&final_output);
  }

  pub fn check_board_legality(self: &Self, masks: &Masks, sliding: &SlidingMoves) -> Result<(bool), String>{
    let opposing_king_location = match self._current_move {
      Color::WHITE => self._black.extract_king().unwrap(),
      Color::BLACK => self._white.extract_king().unwrap(),
    };
    let attacks:u64 = match self.get_attacks(&masks, &sliding) {
      Ok(atk) => atk,
      Err(msg) => return Err(msg),
    };
    if(attacks & opposing_king_location) != 0 {
      return Ok(false)
    }
    Ok((true))
  }

  pub fn check_move_legality(self: &mut Self,) -> Result<(),String>{
    Ok(())
  }

}
  
