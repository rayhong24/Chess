from __future__ import annotations

from Pieces.piece import Piece
from enums import Colour

class Pawn(Piece):
    def __init__(self, colour: Colour, row: int, column: int) -> None:
        super().__init__(colour, row, column)
    
    def get_representation(self) -> str:
        return 'p' if self.colour == Colour.BLACK else 'P'

    def get_moves(self, board: [[Piece]]) -> [tuple]:
        # list of tuples of new coordinates the piece can go
        valid_moves = []

        direction = -1 if self.colour == Colour.WHITE else 1

        # checking forward moves
        moves_forward = 1 if self.has_moved else 2
        for di in range(1, moves_forward+1):
            i, j = self.row+(di*direction), self.column
            if self.is_inbounds(i, j) and board[i][j] == None:
                valid_moves.append((i, j))
        
        # check captures
        i = self.row+direction
        j_left = self.column-1
        j_right = self.column+1
        if self.is_inbounds(i, j_left):
            square_to_check1 = board[i][j_left]
            if square_to_check1 is not None and square_to_check1.colour != self.colour:
                valid_moves.append((i, j_left))
        
        if self.is_inbounds(i, j_right):
            square_to_check2 = board[i][j_right]
            if  square_to_check2 is not None and square_to_check2.colour != self.colour:
                valid_moves.append((i, j_right))

        # TODO: En passant
        
        return valid_moves