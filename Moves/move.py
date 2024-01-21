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

    def __eq__(self, other) -> bool:
        if not isinstance(other, Move):
            return False
        return self.player_to_move == other.player_to_move\
        and self.piece_str == other.piece_str\
        and self.start_coords == other.start_coords\
        and self.capture == other.capture\
        and self.end_coords == other.end_coords

    def __repr__(self):
        repr_str = "{}{}{}{}{}{}{}".format(
            self.__class__.__name__,
            self.player_to_move,
            self.piece_str,
            self.start_coords,
            self.capture,
            self.end_coords,
            self.__str__()
        )

        return repr_str

    def __str__(self):
        piece_str = "" if self.piece_str == 'P' else self.piece_str
        capture_str = "x" if self.capture else "-"
        s = "{}{}{}{}".format(
            piece_str,
            coords_to_square(self.start_coords[0], self.start_coords[1]),
            capture_str,
            coords_to_square(self.end_coords[0], self.end_coords[1])
        )

        return s


    def check_valid(self, game) -> bool:
        piece_on_square = game.board.board[self.start_coords[0]][self.start_coords[1]] 

        # Check piece at starting coords
        if not (game.player_turn == self.player_to_move\
        and piece_on_square is not None \
        and piece_on_square.colour == self.player_to_move \
        and piece_on_square.get_representation().upper() == self.piece_str):
            return False
        
        # en passant

        return self in piece_on_square.get_moves(game.board.board)

    def make_move(self, game) -> bool:
        if not self.check_valid(game):
            return False

        game.board.move_piece(
            self.start_coords[0],
            self.start_coords[1],
            self.end_coords[0],
            self.end_coords[1]
        )
        game.switch_player_turn()
        return True

