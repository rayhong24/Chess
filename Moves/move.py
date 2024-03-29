import copy

import Pieces

from enums import Colour
from coords import Coords

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
        repr_str = "{}{}".format(
            self.player_to_move,
            self.__str__()
        )

        return repr_str

    def __str__(self):
        piece_str = "" if self.piece_str == 'P' else self.piece_str
        capture_str = "x" if self.capture else "-"
        s = "{}{}{}{}".format(
            piece_str,
            self.start_coords,
            capture_str,
            self.end_coords
        )

        return s


    def check_valid(self, game) -> bool:
        piece_on_square = game.board.get_square(self.start_coords)

        return game.player_turn == self.player_to_move\
        and piece_on_square is not None\
        and piece_on_square.colour == self.player_to_move\
        and piece_on_square.get_representation().upper() == self.piece_str\
        and self in piece_on_square.get_moves(game)

    def set_new_board(self, board) -> bool:
        board.move_piece(
            self.start_coords,
            self.end_coords
        )

    def make_move(self, game) -> bool:
        if not self.check_valid(game):
            return False

        old_board = copy.deepcopy(game.board)
        self.set_new_board(game.board)

        if game.is_king_in_check(game.player_turn):
            game.board = old_board
            return False


        # enpassant
        if self.piece_str == 'P' and abs(self.start_coords.rank-self.end_coords.rank) == 2:
            direction = 1 if self.player_to_move == Colour.WHITE else -1
            game.enpassant_coords = self.start_coords.get_neighbour(direction, 0)
        else:
            game.enpassant_coords = None
        game.switch_player_turn()

        return True

