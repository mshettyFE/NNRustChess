Have engine toggle between
1. Generating Training Data 
2. Running in UCI mode

Plan:
	Uses bitboards: DONE
	Can read in FEN notation: DONE
	Implement SAN parser/emitter
	Implement PGN parser: DONE
		As part of SAN and PGN parser, calculate index of move in output array
	Implement UCI parser/emitter
	Implements UCI protocol
Parser:
	Have Engine open up stockfish in uci mode in one thread, and zstdcat a lichess database in another thread
		Use PGN and SAN parser to break games from zstdcat into individual moves
		Iterating through moves, have stockfish evaluate all legal moves from a board position, and shove into numpy array
		Save input and output as two safetensor files
Training:
	Have PyTorch each safetensor files from disk
	Use NN to learn stuff
