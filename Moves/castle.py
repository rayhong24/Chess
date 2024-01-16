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

        i, j = self.start_coords
        j += dj

        board = game.board

        while board.is_inbounds(i, j):
            if board.board[i][j] is not None:
                return board.board[i][j].get_representation().upper() == 'R'
            j += dj

        return False

    def make_move(self, game) -> bool:
        if not self.check_valid(game):
            return False 
        
        dj = 1 if self.king_side else -1

        rook_i, rook_j = self.start_coords
        rook_j += dj

        board = game.board

        # assumed to be valid because of check_valid() call
        while board.board[rook_i][rook_j] is None or\
        board.board[rook_i][rook_j].get_representation().upper() != 'R':
            rook_j += dj


        # Move king
        game.board.move_piece(
            self.start_coords[0],
            self.start_coords[1],
            self.end_coords[0],
            self.end_coords[1]
        )

        # Move rook
        game.board.move_piece(
            rook_i,
            rook_j,
            self.end_coords[0],
            self.end_coords[1]-dj
        )


        game.switch_player_turn()