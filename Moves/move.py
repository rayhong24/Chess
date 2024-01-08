import Pieces

from enums import Colour
from utils import *

class Move:
    def __init__(self, player_to_move, piece_str, start_coords, capture, end_coords) -> None:
        self.player_to_move = player_to_move

        self.piece_str = piece_str
        self.start_coords = start_coords
        self.capture = capture
        self.end_coords = end_coords
    
    def __hash__(self) -> int:
        return hash(repr(self))

    def check_valid(self, game) -> bool:
        piece_on_square = game.board.board[self.start_coords[0]][self.start_coords[1]] 

        return game.player_turn == self.player_to_move\
        and piece_on_square is not None \
        and piece_on_square.colour == self.player_to_move \
        and piece_on_square.get_representation().upper() == self.piece_str \
        # and self in piece_on_square.get_moves(game.board.board)

    def make_move(self, game) -> bool:
        if not self.check_valid(game):
            return False

        game.board.move_piece(
            self.start_coords[0],\
            self.start_coords[1],\
            self.end_coords[0],\
            self.end_coords[1]
        )
        return True

