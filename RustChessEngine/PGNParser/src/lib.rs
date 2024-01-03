use regex::{Regex, bytes};
use std::io::*;


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
    return games;
}
