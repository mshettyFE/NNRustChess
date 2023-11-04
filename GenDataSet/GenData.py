import sys
import re
from stockfish import Stockfish
import Bindings

def GMasks():
	print(Bindings.vector())
	print(Bindings.array())
	print(Bindings.sum_as_string(10,20))

## (?<=\*)[#0-9A-Za-z +-]+(?=\*)

WHITE = True
BLACK = False

def parser():
	inpt=  sys.stdin
	games = []
	line = sys.stdin.readline()
	while line:
		line = sys.stdin.readline()
		if(line=="\n"):
			line = sys.stdin.readline()
# Replace Numbering with asteriks
			line = re.sub("[0-9]+\.","*",line)
# Append * to end for more convinient parsing
			line = line[:-1] +  "*"
# Extract contents between asteriks
			turns = re.findall("(?<=\*)[#0-9A-Za-z +-]+(?=\*)",line)
# Remove white space around all contents
			turns  = [turn.strip() for turn in turns]
			moves = []
			player = WHITE
			for turn in turns:
# Split each turn into moves via splitting on white space
				mvs = turn.split(" ")
				for m in mvs:
					moves.append(clean_move(m), player)
					player = not player
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
	GMasks()
	sys.exit()
	stockfish = Stockfish(path= "/usr/games/stockfish")
	games = parser()
	data = []
	for game in games:
		print(game)
		run_stock(stockfish,game)
		break
