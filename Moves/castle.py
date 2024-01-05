from Moves.move import Move
from enums import *

class Castle(Move):
    def __init__(self, move_str, player_to_move) -> None:
        start_i = 7 if player_to_move == Colour.WHITE else 0
        start_j = 4

        end_i = 7 if player_to_move == Colour.WHITE else 0
        end_j = 6 if move_str == "O-O" else 2

        super().__init__(
            player_to_move,
            'K',
            (start_i, start_j),
            False,
            (end_i, end_j)
        )

        self.king_side = move_str == "O-O"