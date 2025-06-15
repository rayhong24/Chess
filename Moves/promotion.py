from Moves.move import Move
from GameClasses.board import Board

from Pieces.pawn import Pawn
from Pieces.queen import Queen
from Pieces.rook import Rook
from Pieces.bishop import Bishop
from Pieces.knight import Knight

class Promotion(Move):
    def __init__(self, player_to_move, start_coords, capture, end_coords, promotion_str):
        super().__init__(player_to_move, start_coords, capture, end_coords)

        self.promotion_str = promotion_str
        

    def __str__(self):
        s = "{}{}{}".format(
            self.start_coords,
            self.end_coords,
            self.promotion_str
        )

        return s
    
    def apply(self, board: Board):
        super().apply(board)

        board.set_square(None, self.start_coords)

        new_piece = None

        if self.promotion_str == "q":
            new_piece = Queen(self.player_to_move)
        elif self.promotion_str == "r":
            new_piece = Rook(self.player_to_move)
        elif self.promotion_str == "b":
            new_piece = Bishop(self.player_to_move)
        elif self.promotion_str == "n":
            new_piece = Knight(self.player_to_move)

        board.set_square(new_piece, self.end_coords)
        return

    def undo(self, board: Board):
        board.set_square(Pawn(self.player_to_move), self.start_coords)
        board.set_square(self.end_piece, self.end_coords)

