from copy import deepcopy
from Moves.move import Move

class EnPassant(Move):
    def __init__(self, player_to_move, start_coords, end_coords) -> None:
        super().__init__(player_to_move, 'P', start_coords, True, end_coords)

    def __eq__(self, other):
        if not isinstance(other, EnPassant):
            return False
        return super().__eq__(other)

    def __hash__(self) -> int:
        return super().__hash__()

    def __repr__(self):
        return "enp{}{}".format(
            self.player_to_move,
            super().__str__()
        )

    def check_valid(self, game) -> bool:
        return super().check_valid(game)

    def set_new_board(self, board) -> bool:
        super().set_new_board(board)

        board.remove_piece(
            8-self.start_coords[0],
            self.end_coords[1],
        )

    def make_move(self, game) -> bool:
        super().make_move(game)




        
