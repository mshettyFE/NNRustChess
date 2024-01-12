use regex::{Regex};
use std::{thread};
use constants::*;
use safetensors::*;
use std::io::{BufRead, BufReader, Write,Read};
use std::sync::{Arc, Mutex};
use std::process::*;

pub fn spawn_stock()->(ChildStdin, ChildStdout){
// spawn a stockfish instance a background process. Returns Stdin and Stdout to said process so that calls can be made to stockfish
// needed to generate board position evaluations for training data generation purposes
  let mut child = Command::new("stockfish")
  .stdin(Stdio::piped())
  .stdout(Stdio::piped())
  .spawn()
  .unwrap();
  let mut stdin = child.stdin.take().unwrap();
  let mut stdout = child.stdout.take().unwrap();
  // discard welcome message from stockfish so that stdout is clean
  let mut stdout_buf = BufReader::new(stdout.by_ref());
  let mut log = String::new();
  stdout_buf.read_line(&mut log);
  return (stdin, stdout);
}

// send certain commands to stockfish. seperate functions since stdout must be handled differently in each case

pub fn set_position(fen: &str, stock_stdin: &mut ChildStdin,  stock_stdout: &mut ChildStdout) ->Result<(), String>{
// given a FEN string, set the internal state of stockfish to this board
  match validate_fen(fen) {
    Ok(()) => (),
    Err(msg) => return Err(msg)
  }
// stockfish command is: position fen <fen_string>\n
  let mut log = String::new();
  let mut msg_cpy = "position fen ".to_string()+fen;
  msg_cpy.push('\n');
// send command to stockfish in another thread, then wait for it to finish. Using threads incase I want to read from stdout later on
  let writer = Arc::new(Mutex::new(stock_stdin));
  let mut writer_clone = writer.clone();
  thread::scope(|s| {
    let h = s.spawn(move ||{
      let mut writer_lock = writer_clone.lock().unwrap();
      writer_lock.write_all(msg_cpy.as_bytes()).expect("Couldn't write to buffer");
      writer_lock.flush();
    });
  // thread needs to finish
    h.join();
  });
  Ok(())
}

pub fn print_stock(stock_stdin: &mut ChildStdin,  stock_stdout: &mut ChildStdout){
  let mut log = String::new();
  let mut msg_cpy = "d\n".to_string();
  let writer = Arc::new(Mutex::new(stock_stdin));
  let mut writer_clone = writer.clone();
  thread::scope(|s| {
    s.spawn(move ||{
      let mut writer_lock = writer_clone.lock().unwrap();
      writer_lock.write_all(msg_cpy.as_bytes()).expect("Couldn't write to buffer");
      writer_lock.flush();
    });
  });
// terminating condition is a line containing Checkers
  let mut buffer = BufReader::new(stock_stdout);
  for line in buffer.lines(){
    let mut l = line.unwrap();
    if(l.contains("Checkers")){
      println!("{}",l);
      return;
    }
    println!("{}",l);
  }
}

pub fn eval_pos(stock_stdin: &mut ChildStdin,  stock_stdout: &mut ChildStdout)->f32{
// Evaluate a board position via stockfish.
//Still need to pass in FEN string to initialize board prior to evaluation.
  let get_eval = Regex:: new(r".+([\+\-][0-9\.]+)").unwrap();
  let mut log = String::new();
  let writer = Arc::new(Mutex::new(stock_stdin));
  let mut writer_clone = writer.clone();
  thread::scope(|s| {
    s.spawn(move ||{
      let mut writer_lock = writer_clone.lock().unwrap();
      writer_lock.write_all("eval\n".as_bytes()).expect("Couldn't write to buffer");
      writer_lock.flush();
    });
  });
  let mut reader = BufReader::new(stock_stdout);
  for line in reader.lines(){
    let str = line.unwrap();
// terminating condition is Final evaluation. This is were the evaluation should be
// TODO: Might need to check if position in checkmate
    if str.contains("Final evaluation"){
      let caps = get_eval.captures(&str).unwrap();
      log = caps.get(1).unwrap().as_str().to_string();
      break;
    }
  }
  let out: f32 = log.parse().unwrap();
  return out;
}

pub fn loc_to_board(location: &str) -> Result<u64,String> {
  // Takes a two character location in rank-file notation, and convert it to a u64 board position
  let loc = location.trim().to_lowercase();  
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

pub fn validate_fen(fen: &str ) -> Result<(), String> {
  // Given a string, check if it is a valid FEN position
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
          subtraction = 1;
          occupied_spaces += 1;
        },
        'n' => {
          subtraction = 1;
          occupied_spaces += 1;
        },
        'b' => {
          subtraction = 1;
          occupied_spaces += 1;
        },
        'q' => {
          subtraction = 1;
          occupied_spaces += 1;
        },
        'k' => {
          subtraction = 1;
          occupied_spaces += 1;
        },
        'p' => {
          subtraction = 1;
          occupied_spaces += 1;
        },
        'R' => {
          subtraction = 1;
          occupied_spaces += 1;
        },
        'N' => {
          subtraction = 1;
          occupied_spaces += 1;
        },
        'B' => {
          subtraction = 1;
          occupied_spaces += 1;
        },
        'Q' => {
          subtraction = 1;
          occupied_spaces += 1;
        },
        'K' => {
          subtraction = 1;
          occupied_spaces += 1;
        },
        'P' => {
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
  // Who is to move currently
  let binding: String = end_section[0].trim().to_lowercase();
  let current_player: &str = binding.as_str();
  match current_player{
    "w" => {
    }
    "b" => {
    }
    current_player => {
      let msg: String = "Problem here: ".to_owned() + current_player+ " Must be w or b";
      return Err(msg);
    }
  }
  // Castling rights
  let castle: &str =  end_section[1].trim();
  let castle_len = castle.len();
  if castle_len == 0{
    let msg: String = "Problem here: ".to_owned() + castle+ " Must be non-empty";
    return Err(msg);
  }
  else if castle_len == 1{
    if castle == "-" {}
    match castle {
      "k" => (),
      "K" => (),
      "q" => (),
      "Q" => (),
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
        'k' => (),
        'K' => (),
        'q' => (),
        castle_char => {
          let msg: String = "Problem here: ".to_owned() + &castle_char.to_string() + " Characters must be either k, K, q, or Q";
          return Err(msg);    
        }    
      }
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
    match loc_to_board(enpassant) {
      Ok(board) => final_enpass = board,
      Err(msg) => return Err(msg),
    }
  }
  else{
    let msg: String = "Problem here for enpassant: ".to_owned() + enpassant+ " Must be either - or a board location in rank-file notation";
    return Err(msg);
  }
  // half moves
  let half_move_str: &str =  end_section[3].trim();
  match half_move_str.parse::<u64>(){
    Ok(half_moves) => (),
    Err(_msg) => return Err("Unable convert half move value to unsigned integer".to_string()),
  }
  // full moves
  let full_move_str: &str =  end_section[4].trim();
  match full_move_str.parse::<u64>(){
    Ok(full_moves) => (),
    Err(_msg) => return Err("Unable convert full move value to unsigned integer".to_string()),
  }
  Ok(())
}

pub fn pgn_parser() ->Vec<Vec<String>>{
// given a pgn file from the lichess database, grab all the moves for all the games present
  let remove_numbering = Regex:: new(r"[0-9]+\.").unwrap();
  let move_re = Regex::new(r"\* ([#0-9A-Za-z+-]+) ([#0-9A-Za-z+-]+) ([ #0-9A-Za-z+-\/]+)?\*").unwrap();
  let mut buffer  = String::new();
  let mut games  = Vec::new();
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  let mut counter: i32 = 0;
  while(true){
      let mut bytes_read = handle.read_line(&mut buffer).unwrap();
      // EOF
      if(bytes_read == 0){
          break;
      }
      // New line encountered. This only happens if starting a new game, or the next line is the moves
      if(bytes_read == 1){
          buffer.clear();
          bytes_read = handle.read_line(&mut buffer).unwrap();
          // Skip new game headers
          if(buffer.contains("Event")){
              continue;
          }
          // replace full move numbers with "**" for easier parsing
          let mut stripped = remove_numbering.replace_all(&buffer[..],"**");
          // replace newline with " *" for easier parsing
          let terminated = stripped.replace('\n', " **");
          // Get all
          let mut full_moves: Vec<_> = move_re.captures_iter(&terminated).flat_map(
              |caps|{[caps.get(1).unwrap().as_str().to_string(), caps.get(2).unwrap().as_str().to_string()]}
          ).collect();
          // if the games are eval() games, ignore them (while be running eval on our own later)
          if(full_moves.len()==0){
              continue;
          }
          // remove game result  from end
          let last_index = full_moves.len()-1;
          if(full_moves[last_index].contains("-")){
              full_moves.remove(last_index);
          }
          games.push(full_moves.clone());
      }
  }
// list of list of SAN moves
  return games;
}

pub fn print_locations(board: u64){
  // print 1 if piece is present at square, otherwise, print 0
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
  // print the binary string representation of the occupied squares
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
// Print the occupancy void board (void board used for boundary checking)
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
// print the piece type of each square
  for number in (0..64).rev(){
    print!("{} ",char_board[number]);
    if (number%8)==0{
      print!("\n");
    }
  }
  print!("\n");
}