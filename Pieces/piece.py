from __future__ import annotations

from enums import Colour
from strings import *

class Piece:
    def __init__(self, colour: Colour, row: int, column: int) -> None:
        self.colour = colour
        self.row = row
        self.column = column
        self.has_moved = False
    
    def get_representation(self) -> str:
        raise NotImplementedError

    def get_moves(self, board: Board) -> [tuple]:
        raise NotImplementedError
    
    def move(self, board: Board, new_i: int, new_j: int) -> None:
        self.check_move_errors(board, self.get_representation())               
        
        self.row = new_i
        self.column = new_j
        self.has_moved = True
    
    def check_move_errors(self, board: Board, representation) -> None:
        if board.board[self.row][self.column] is None:
            raise Exception(invalid_move_no_piece_message.format(\
            self.row,\
            self.column))
        elif board.board[self.row][self.column].get_representation() != representation:
            raise Exception(invalid_move_wrong_piece.format(\
            self.row,\
            self.column,\
            board[self.row][self.column].get_representation(), \
            representation))







    
