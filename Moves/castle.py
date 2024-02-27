from copy import deepcopy
from Moves.move import Move
from enums import *
from coords import Coords

class Castle(Move):
    def __init__(self, move_str, player_to_move) -> None:
        start_i = 7 if player_to_move == Colour.WHITE else 0
        start_j = 4

        end_i = 7 if player_to_move == Colour.WHITE else 0
        end_j = start_j + 2 if move_str == "O-O" else start_j - 2

        super().__init__(
            player_to_move,
            'K',
            Coords(start_i, File(start_j)),
            False,
            Coords(end_i, File(end_j))
        )

        self.king_side = move_str == "O-O"
    
    def __hash__(self) -> int:
        return hash(repr(self))

    def __eq__(self, other):
        if not isinstance(other, Castle):
            return False

        return self.king_side == other.king_side and self.player_to_move == other.player_to_move

    def __repr__(self):
        repr_str = "{}{}{}".format(
            self.__class__.__name__,
            self.player_to_move,
            self.king_side   
        )
        return repr_str

    def __str__(self):
        return "O-O" if self.king_side else "O-O-O"

    def check_valid(self, game) -> bool:
        if not super().check_valid(game):
            return False

        dj = 1 if self.king_side else -1

        for coords in self.start_coords.get_line(0, dj):
            square = game.board.get_square(coords)
            if square is not None:
                return square.get_representation().upper() == 'R'

        return False

    def set_new_board(self, board) -> bool:
        dj = 1 if self.king_side else -1

        rook_coords = None

        # assumed to be valid because of check_valid() call
        for coords in self.start_coords.get_line(0, dj):
            if board.get_square(coords) is not None:
                rook_coords = coords
                break

        # Move king
        board.move_piece(
            self.start_coords,
            self.end_coords
        )

        # Move rook
        board.move_piece(
            rook_coords,
            self.end_coords.get_neighbour(0, -dj)
        )

    def make_move(self, game) -> bool:
        if not self.check_valid(game):
            return False 
        
        dj = 1 if self.king_side else -1

        old_board = deepcopy(game.board)
        self.set_new_board(game.board)

        # if castling through check
        # should try to move logic into check_valid()
        if game.is_square_in_check(self.start_coords, game.player_turn):
            game.board = old_board
            return False
        for coords in self.start_coords.get_line(0, dj):
            if game.is_square_in_check(coords, game.player_turn):
                game.board = old_board
                return False

            if coords == self.end_coords:
                break

        game.enpassant_coords = None

        game.switch_player_turn()

        return True