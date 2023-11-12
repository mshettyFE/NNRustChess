import sys
import re
from stockfish import Stockfish
import Bindings
import numpy as np
import chess

def InitState():
	a = Bindings.Masks()
	b = Bindings.SlidingMoves()
	b.initialize(a)
	c = Bindings.GameState()
	c.parse_fen_py("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
	d = Bindings.MoveAN()
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

def gen_data_entry(stockfish,board_position, MoveData, Board, Slide, Masks):
	stockfish.set_fen_position(board_position)
	# Output representation of moves. The numbers are:
	#   64: number of squares on a chess board
	#	8: number of ray directions
	#	7: maximum array length
	#	+8: number of knight moves
	#	3*4: Possible types of Pawn Promotions
	policy_vector_output = np.zeros(64*(8*7+8+3*4))
	# 1000 arbitrary. Set to large number to generate all possible legal moves
	for move in stockfish.get_top_moves(1000):
		print(move)
		index = Bindings.gen_index_py(move["Move"])
		if move["Centipawn"]:
			policy_vector_output[index] = move["Centipawn"]
		else:
		# Checkmate condition
			policy_vector_output[index] = 525600
	return (board_position, policy_vector_output)

def run_stock(stockfish, game, MoveData, Board, Slide, Masks):
	stockfish.set_fen_position("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
	starting_input = stockfish.get_fen_position()
	output_data = []
	for move in game:
		output_data.append(gen_data_entry(stockfish, starting_input, MoveData, Board, Slide, Masks))
		stockfish_move = MoveData.parse_move_py(move,Board,Slide, Masks)
		stockfish.make_moves_from_current_position([stockfish_move])
		starting_input = stockfish.get_fen_position()
		break
	return output_data

if __name__ == "__main__":
	Masks, SlidingMoves, Board, Move = InitState()
	stockfish = Stockfish(path= "/usr/games/stockfish")
	games = pgn_parser()
	data = []
	for game in games:
		print(game)
		run_stock(stockfish,game, Move, Board, SlidingMoves, Masks)
		break
