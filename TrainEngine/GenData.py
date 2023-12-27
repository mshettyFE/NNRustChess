import sys
import re
import bindings
import numpy as np
import chess
import chess.engine

def InitState():
	a = bindings.Masks()
	b = bindings.SlidingMoves()
	b.initialize(a)
	c = bindings.GameState()
	c.parse_fen_py("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
	d = bindings.MoveAN()
	return (a,b,c,d)

def pgn_parser():
	inpt=  sys.stdin
	games = []
	line = sys.stdin.readline()
	while line:
		line = sys.stdin.readline()
		if(line=="\n"):
			line = sys.stdin.readline()
# Replace Numbering with asterisks
			line = re.sub("[0-9]+\.","*",line)
# Append * to end for more convenient parsing
			line = line[:-1] +  "*"
# Extract contents between asterisks
			turns = re.findall("(?<=\*)[#0-9A-Za-z +-]+(?=\*)",line)
# Remove white space around all contents
			turns  = [turn.strip() for turn in turns]
			moves = []
			for turn in turns:
# Split each turn into moves via splitting on white space
				mvs = turn.split(" ")
				for m in mvs:
					moves.append(m)
# remove last item in moves since that is just the game result
			moves = moves[:-1]
			if (any( [len(m) > 6 for m in moves] ) ):
				print(moves)
			line = inpt.readline()
			games.append(moves)
			break
	return games

def gen_data_entry(board, engine):
	# Output representation of moves. The numbers are:
	#   64: number of squares on a chess board
	#	8: number of ray directions
	#	7: maximum array length
	#	+8: number of knight moves
	#	3*4: Possible types of Pawn Promotions
	policy_vector_output = np.zeros(64*(8*7+8+3*4))
	# 1000 arbitrary. Set to large number to generate all possible legal moves
	for move in board.legal_moves:
		index = bindings.gen_index_py(move.uci())
		info = engine.analyse(board, chess.engine.Limit(depth=20)).get("score").white()
		policy_vector_output[index] = info.score(mate_score=100000)
	return policy_vector_output

def run_stock(engine, game):
	board = chess.Board()
	output_data = []
	for move in game:
		print(move)
		input_fen = board.fen()
		output_vec = gen_data_entry(board, engine)
		output_data.append((input_fen, output_vec) )
		chess_move = board.parse_san(move)
		board.push(chess_move)
	return output_data

if __name__ == "__main__":
	Masks, SlidingMoves, Board, Move = InitState()
	engine = chess.engine.SimpleEngine.popen_uci("/usr/games/stockfish")
	games = pgn_parser()
	data = []
	for game in games:
		print(game)
		run_stock(engine,game)
	engine.close()
