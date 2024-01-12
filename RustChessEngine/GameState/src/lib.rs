use constants::*;
use masks::*;
use magic::*;
use chessio::*; 
use SideState::SideState;
use MoveAlgebraNotation::MoveAN;

pub struct GameState{
  pub _white: SideState,
  pub _black: SideState,
  pub _current_move: Color,
  pub _enpassant: u64, // is enpassant allowed (and where is it?)
  pub _halfmove: u64, // half moves are when 1 side moves
  pub _fullmove: u64, // full mvoes are when both sides take a turn
}

impl Default for GameState {
  fn default() -> Self {
    GameState{ _white: SideState::default(),_black: SideState::default(), _current_move: Color::WHITE, _enpassant: 0, _halfmove:0, _fullmove: 0}
  }
}

impl GameState{

  pub fn parse_fen(&mut self, fen: &str ) -> Result<(), String> {
    // parse a FEN string into the game state
    // break up by / to seperate ranks
    let parts = fen.split("/");
    if parts.clone().count() != 8{
      return Err("Invalid number of Ranks".to_owned());
    }
    let mut  sections: Vec<&str> = parts.collect();
    // break up last part and seperate by space
    let end_part = sections[7].trim().split(" ");
    if end_part.clone().count() != 6 {
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
        let subtraction: isize;
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
        if cur_index == 0{
          break 'outer;
        }
        else if (cur_index as isize - subtraction)  >= 0 {
          cur_index -= subtraction as usize;
        }
        else{
          let msg: String = "Problem here: ".to_owned()+rank.clone() + " Out of bounds";
          return Err(msg);
        }
      }
    }
      // Castling rights
      let castle: &str =  end_section[1].trim();
      let castle_len = castle.len();
      let mut white_castle: u8 = 0;
      let mut black_castle: u8 = 0;
      if castle_len == 0{
        let msg: String = "Problem here: ".to_owned() + castle+ " Must be non-empty";
        return Err(msg);
      }
      else if castle_len == 1{
        if castle == "-" {}
        match castle {
          "k" => black_castle |= CASTLING_KING,
          "K" => white_castle |= CASTLING_KING,
          "q" => black_castle |= CASTLING_QUEEN,
          "Q" => white_castle |= CASTLING_QUEEN,
          castle => {
            let msg: String = "Problem here: ".to_owned() + &castle.to_string() + " Characters must be either k, K, q, or Q";
            return Err(msg);    
          }    
        }
      }
      else if castle_len > 4 {
        let msg: String = "Problem here: ".to_owned() + castle+ " Must be at most 4 characters of either k, K, q, or Q";
        return Err(msg);
      }
      else{
        for castle_char in castle.chars() {
          match castle_char {
            'k' => black_castle |= CASTLING_KING,
            'K' => white_castle |= CASTLING_KING,
            'q' => black_castle |= CASTLING_QUEEN,
            'Q' => white_castle |= CASTLING_QUEEN,
              castle_char => {
              let msg: String = "Problem here: ".to_owned() + &castle_char.to_string() + " Characters must be either k, K, q, or Q";
              return Err(msg);    
            }    
          }
        }  
      }
      self._black._castling = black_castle;
      self._white._castling = white_castle;
  
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
    // enpassant

    let enpassant: &str =  end_section[2].trim();
    let mut final_enpass: u64  = 0;
    let _enpassant_len = enpassant.len();
    if enpassant.len() == 1{
      if enpassant == "-" {}
      else{
        let msg: String = "Problem here: ".to_owned() + enpassant+ " Must be either - or a board location in rank-file notation";
        return Err(msg);
      }  
    }
    else if _enpassant_len == 2{
      match chessio::loc_to_board(enpassant) {
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
      Err(_msg) => return Err("Unable convert half move value to unsigned integer".to_string()),
    }
    // full moves
    let full_move_str: &str =  end_section[4].trim();
    match full_move_str.parse::<u64>(){
      Ok(full_moves) => self._fullmove = full_moves,
      Err(_msg) => return Err("Unable convert full move value to unsigned integer".to_string()),
    }
    Ok(())
  }

  fn get_attacks(&self, masks: &Masks, sliding: &SlidingMoves) -> Result<u64, String> {
    // get all the attack locations of the current side
    let current_side: &SideState = match self._current_move{
      Color::WHITE => &self._white,
      Color::BLACK => &self._black,
    };
    let mut temp_current_occupied: u64 =  current_side._occupied;
    let mut output: u64 = 0;
    while temp_current_occupied != 0 {
      let index: usize = temp_current_occupied.trailing_zeros() as usize;
      let lowest: u64 = temp_current_occupied & (1 << index);
      if (lowest & current_side._king) != 0{
        // find overlaps with your own pieces, and remove this overlap from attacks
        let attack: u64 = masks._king_mask[index] ^ (masks._king_mask[index] & current_side._occupied);
        output |= attack;
      }
      else if (lowest & current_side._knight) != 0{
        let attack: u64 = masks._knight_mask[index] ^ (masks._knight_mask[index] & current_side._occupied);
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
        let pawn_attack_mask = masks._pawn_capture_mask[&self._current_move][index];
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

  fn find_occupied(&self) ->Result<u64, String> {
// caluclate bitboard for denoting all occupied positions
    let white_oc = self._white._occupied;
    let black_oc = self._black._occupied;
    if (white_oc & black_oc) != 0{
      return Err("Overlapping white and black pieces".to_string());
    }
    Ok(white_oc | black_oc)
  }

  pub fn print_board( &self){
    // print board in human readable format
    let  white_output: [char; 64];
    let  black_output: [char; 64];
    let mut final_output: [char; 64] = ['-'; 64];
    match self._black.to_char_board() {
      Ok(black_otpt) => black_output = black_otpt,
      Err(_msg) => panic!("Invalid board State for black"),
    }
    match self._white.to_char_board() {
      Ok(white_otpt) => white_output = white_otpt,
      Err(_msg) => panic!("Invalid board State for white"),
    }
    for i in 0..64{
      if white_output[i] == '-' && black_output[i] == '-'{
        final_output[i] = '-';
      }
      else if white_output[i] != '-' && black_output[i] == '-'{
        final_output[i] = white_output[i].to_ascii_uppercase();
      }
      else if white_output[i] == '-' && black_output[i] != '-'{
        final_output[i] = black_output[i];
      }
      else{
        panic!("Invalid board state. Overlapping pieces");
      }
    }
    print_char_board(&final_output);
  }

  pub fn check_board_legality(&self, masks: &Masks, sliding: &SlidingMoves) -> Result<bool, String>{
// check if it is your turn and you can currently attack the opposing king. If you can, then we are in an invalid board position
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
    Ok(true)
  }

  pub fn validate_move_an(&mut self, cand_move: &MoveAN, masks: &Masks, sliding: &SlidingMoves) -> Result<(),String>{
    // validates if cand_move is compatible with current board state
    let mut output: u64 = EMPTY_BOARD;
    if(cand_move._color != self._current_move){
      return Err("Move color doesn't match side that moves next".to_string());
    }
    let cur_side = match(self._current_move){
      Color::BLACK => self._white.clone(),
      Color::WHITE => self._black.clone(),
    };
    // where all the pieces are on the board
    let occupied: u64 = self.find_occupied().unwrap();
    // check for castling
    if(cand_move._castling != CASTLING_NONE){
    // check lower 4 bits of current side to see if any bits match. If not
      if (cand_move._castling & cur_side._castling & 0x0F) == 0{
        // Move wants to castle, but board state doesn't allow it
        return Err("Unable to castle".to_string());
      }
      else{
        // Some bits are the same between the board state and the move. check if you can do this castle
        // get the other side's attacks
        let mut other_side_attacks: u64 = 0;
        // also get what squares to check for opposing attacks (can't castle over check. Can't castle out of check)
        let mut king_castle_square_check: u64 = 0;
        let mut queen_castle_square_check: u64 = 0;
        match self._current_move {
            Color::WHITE => {
              self._current_move = Color::BLACK;
              other_side_attacks = self.get_attacks(masks, sliding).unwrap();
              king_castle_square_check = WHITE_KING_SIDE_CASTLE_CHECK;
              queen_castle_square_check = WHITE_QUEEN_SIDE_CASTLE_CHECK;
              self._current_move = Color::WHITE;
            },
            Color::BLACK => {
              self._current_move = Color::WHITE;
              other_side_attacks =self.get_attacks(masks, sliding).unwrap();
              king_castle_square_check = BLACK_KING_SIDE_CASTLE_CHECK;
              queen_castle_square_check = BLACK_QUEEN_SIDE_CASTLE_CHECK;
              self._current_move = Color::BLACK;
            },
        }
        // some non-NONE castling is found. See if you can castle
        match cur_side._castling {
          CASTLING_KING => {
            // see if you are castling over check
            if(other_side_attacks&king_castle_square_check) !=0{
              return Err("Can't king-side castle in current board state due to checking rules".to_string());
            }
            else{
              return Ok(());
            }
          },
          CASTLING_QUEEN => {
            // see if you are castling over check
            if(other_side_attacks&queen_castle_square_check) !=0{
              return Err("Can't queen-side castle in current board state due to checking rules".to_string());
            }
            else{
              return Ok(());
            }
          }
          _ => return Err("Invalid Actionable Castling state (NONE and ALL don't count)".to_string()),            
        }
      }  
    }

    // generate attack output for a piece type from cand_move
    match cand_move._type {
      PieceType::NONE => return Err("NONE is not a real piece type".to_string()),
      PieceType::PAWN => {
          if(cand_move._capture == true){
            output |= masks._pawn_capture_mask[&cand_move._color][cand_move._start.trailing_zeros() as usize];
          }
          else{
            output |= masks._pawn_moves_mask[&cand_move._color][cand_move._start.trailing_zeros() as usize];
          }
        },
      PieceType::BISHOP => {
          output |= sliding.get_bishop_move(occupied, cand_move._start as usize, masks);
        },
      PieceType::ROOK => {
          output |= sliding.get_rook_move(occupied, cand_move._start as usize, masks);
        },
      PieceType::QUEEN => {
          output |= sliding.get_bishop_move(occupied, cand_move._start as usize, masks);
          output |= sliding.get_rook_move(occupied, cand_move._start as usize, masks);
        },
      PieceType::KING => {
          output |= masks._king_mask[cand_move._start.trailing_zeros() as usize];
        },
      PieceType::KNIGHT => {
          output |= masks._knight_mask[cand_move._start.trailing_zeros() as usize];
        },
    }
    if(cand_move._capture){
      // Check for enpassant
      if( (cand_move._type == PieceType::PAWN) && cand_move._enpassant){
          // check if enpassant square overlap with pawn capture
          if(self._enpassant & output) == 0{
            return Err("Enpassant move not valid".to_string());
          }
        }
      // If capture, remove same color pieces from consideration
      let mut friendly = match cand_move._color{
        Color::WHITE => self._white._occupied,
        Color::BLACK =>self._black._occupied,
      };
      output ^= (output & friendly);
    }
    else{
  // otherwise, remove all other pieces from the output number
      output ^= (output & occupied);
    }
    // check if start position is consistent/ correct piece type is selected
    match cand_move._type {
      PieceType::NONE => return Err("NONE is not a real piece type".to_string()),
      PieceType::PAWN =>
        if (cur_side._pawn & cand_move._start) == 0{
          return Err("Pawn not found".to_string());
        }
      PieceType::ROOK =>
          if (cur_side._rook & cand_move._start) == 0{
            return Err("Rook not found".to_string());
        },
      PieceType::BISHOP =>
          if (cur_side._bishop & cand_move._start) == 0{
            return Err("Bishop not found".to_string());
        },
      PieceType::KNIGHT =>
          if (cur_side._knight & cand_move._start) == 0{
            return Err("Knight not found".to_string());
        },
      PieceType::KING =>
          if (cur_side._king & cand_move._start) == 0{
            return Err("King not found".to_string());
        },
      PieceType::QUEEN =>
          if (cur_side._queen & cand_move._start) == 0{
            return Err("Queen not found".to_string());
        },
    }
    // check if end position is consistent
    if(cand_move._end & output) ==0{
      return Err("end position not consistent with attack pattern".to_string());
    }

    // check promotion type if needed
    if(PieceType::PAWN  == cand_move._type){
      let penultimate_rank = match self._current_move {
        Color::WHITE => RANKB,
        Color::BLACK => RANKG,
      };
    // check if pawn in on penultimate rank to see if we need to promote
    if(penultimate_rank & cand_move._start) != 0 {
    // Invalid piece types for promotion
      if( (cand_move._promotion == PieceType::NONE) || (cand_move._promotion == PieceType::KING) || (cand_move._promotion == PieceType::PAWN)){
        return Err("Invalid promotion type".to_string());
      }
    }
  }
  Ok(())
  }

  pub fn make_move(&mut self, cand_move: &MoveAN,  masks: &Masks, sliding: &SlidingMoves) ->Result<(), String>{
    return Ok(());
  }

  pub fn check_move_legality(&mut self, cand_move: &MoveAN) -> bool{
    return true;
  }

  // given board state, generate all current legal moves
  pub fn gen_legal_moves(&self,masks: &Masks, sliding: &SlidingMoves)-> Vec<MoveAN> {
    let output = Vec::new();
    return output;
  }

  // formatted like https://en.wikipedia.org/wiki/Algebraic_notation_(chess)#Notation_for_moves
  pub fn gen_move_SAN(&self, candidate_move: String,masks: &Masks, sliding: &SlidingMoves) -> Result<(MoveAN),String>{
    let output_move = MoveAN::default();
    Ok((output_move))
  }

  pub fn gen_training_pair(&self, masks: &Masks, sliding: &SlidingMoves) -> ((u64, u64), Vec<u64>){
	// Output representation of moves. The numbers are:
	//   64: number of squares on a chess board
	//	8: number of ray directions
	//	7: maximum array length
	//	+8: number of knight moves
	//	3*4: Possible types of Pawn Promotions
  // for the indices generated in gen_legal_moves(), fill that index with stockfish evaluation of board position after that move
//	policy_vector_output = np.zeros(64*(8*7+8+3*4))
    let output: ((u64,u64), Vec<u64>) = ((0,0),vec![1,2]);
    return output;
  }

}