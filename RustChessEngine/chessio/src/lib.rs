use regex::{Regex, bytes};
use std::thread::sleep;
use std::{io::*};
use std::{thread,time};
use constants::*;
use safetensors::*;
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::process::*;

pub fn spawn_stock()->(ChildStdin, ChildStdout){
//  let mut child = Command::new("/home/lief/NNChessEngine/RustChessEngine/temp.sh")
  let mut child = Command::new("stockfish")
  .stdin(Stdio::piped())
  .stdout(Stdio::piped())
  .spawn()
  .unwrap();
  let mut stdin = child.stdin.take().unwrap();
  let mut stdout = child.stdout.take().unwrap();
  let mut stdout_buf = BufReader::new(stdout.by_ref());
  let mut log = String::new();
  // discard welcome message from stockfish
  stdout_buf.read_line(&mut log);
  return (stdin, stdout);
}

pub fn run_command(msg: &str, stock_stdin: &mut ChildStdin,  stock_stdout: &mut ChildStdout)->String{
// takes in msg, and return the first complete line of output from stockfish
  let mut log = String::new();
  let mut msg_cpy = msg.to_string();
  msg_cpy.push('\n');
  let writer = Arc::new(Mutex::new(stock_stdin));
  let mut writer_clone = writer.clone();
  thread::scope(|s| {
    s.spawn(move ||{
      let mut writer_lock = writer_clone.lock().unwrap();
      writer_lock.write_all(msg_cpy.as_bytes()).expect("Couldn't write to buffer");
      writer_lock.flush();
    });
  });
  let mut reader = BufReader::new(stock_stdout);
  reader.read_line(&mut log);
  return log;
}

pub fn eval_pos(stock_stdin: &mut ChildStdin,  stock_stdout: &mut ChildStdout)->f32{
// Evaluate a board position via stockfish.
//Still need to pass in FEN string to initialize board to evaluation.
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
    if str.contains("Final evaluation"){
      let caps = get_eval.captures(&str).unwrap();
      log = caps.get(1).unwrap().as_str().to_string();
      break;
    }
  }
  let out: f32 = log.parse().unwrap();
  return out;
}


pub fn pgn_parser() ->Vec<Vec<String>>{
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