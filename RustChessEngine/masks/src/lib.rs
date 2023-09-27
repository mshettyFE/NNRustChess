use constants::*;

pub fn BitboardIndexToVoidIndex(index: usize){
  
}

pub fn GenKnightMoves() -> [u64; 64] {
    let VoidBitMapping: VoidBitConversion = VoidBitConversion::default();
    let mut arr:[u64; 64] = [0; 64];
    for bit_index in 0..64{
// Convert bitboard index to void index
        let void_index: usize = VoidBitMapping.bit_to_void(bit_index).unwrap();
// Create array of possible knight moves from current position on voidboard
        let void_candidates: [usize; 8] = [void_index+25,void_index+23,void_index+14, void_index+10,
                                            void_index-25,void_index-23,void_index-14, void_index-10];
        let mut output_moves: u64 = 0;
        for potential_move in void_candidates.iter(){
            match VoidBitMapping.void_to_bit(*potential_move){  
                None => (),
                Some(index) => output_moves |= (1 << index),
            }
            
        }
        arr[bit_index] = output_moves;    
    }
    arr
}
