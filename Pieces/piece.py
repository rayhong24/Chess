from enums import Colour
from strings import *

class Piece:
    def __init__(self, colour, row, column):
        self.colour = colour
        self.row = row
        self.column = column
    
    def get_representation(self):
        raise NotImplementedError

    def get_moves(self, board):
        raise NotImplementedError
    
    def move(self, board, new_i, new_j):
        self.check_move_errors(board, self.get_representation())               
        
        self.row = new_i
        self.column = new_j
    
    def check_move_errors(self, board, representation):
        if board[self.row][self.column] is None:
            raise Exception(invalid_move_no_piece_message.format(\
            self.row,\
            self.column))
        elif board[self.row][self.column].get_representation() != representation:
            raise Exception(invalid_move_wrong_piece.format(\
            self.row,\
            self.column,\
            board[self.row][self.column].get_representation(), \
            representation))







    
