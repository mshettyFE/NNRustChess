import sys
import re
from stockfish import Stockfish
import Bindings

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


def run_stock(stockfish, game):
	stockfish.set_fen_position("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
	stockfish.make_moves_from_current_position(["e2e4"])
	print(stockfish.get_board_visual())
#	for move in game:
#		stockfish.make_moves_from_current_position([move])
#		stockfish.get_board_visual()

if __name__ == "__main__":
	Masks, SlidingMoves, Board, Move = InitState()
	stockfish = Stockfish(path= "/usr/games/stockfish")
	games = pgn_parser()
	data = []
	for game in games:
		print(game)
		print(Move.parse_move_py(game[0],Board,SlidingMoves, Masks))
#		run_stock(stockfish,game)
		break
