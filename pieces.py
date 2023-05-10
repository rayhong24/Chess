from enums import Colour
from strings import *

class Piece:
    def __init__(self, colour, row, column):
        self.colour = colour
        self.row = row
        self.column = column
    
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

class Pawn(Piece):
    def __init__(self, colour, row, column):
        super().__init__(colour, row, column)
        self.has_moved = False
        
    
    def get_representation(self):
        return 'p' if self.colour == Colour.BLACK else 'P'

    def get_moves(self, board):
        super().check_move_errors(board, self.get_representation())
        # list of tuples of new coordinates the piece can go
        valid_moves = []

        direction = -1 if self.colour == Colour.WHITE else 1

        # checking forward moves
        moves_forward = 1 if self.has_moved else 2
        for i in range(1, moves_forward+1):
            valid_moves.append((self.row+(i*direction), self.column))
        
        # check captures
        square_to_check1 = board[self.row+(i*direction)][self.column-1]
        if self.column > 0 and square_to_check1 is not None and square_to_check1.colour != self.colour:
            valid_moves.append((self.row+(i*direction), self.column-1))

        square_to_check2 = board[self.row+(i*direction)][self.column+1]
        if self.column < 7 and square_to_check2 is not None and square_to_check2.colour != self.colour:
            valid_moves.append((self.row+(i*direction), self.column+1))

        # TODO: En passent
        
        print(valid_moves)

        return valid_moves

    def move(self, board, orig_i, orig_j, new_i, new_j):
        super.check_move_errors(board, self.get_representation())               
        
        self.row = new_i
        self.column = new_j

        self.has_moved = True
        


class Rook(Piece):
    def __init__(self, colour, row, column):
        super().__init__(colour, row, column)

    def get_representation(self):
        return 'r' if self.colour == Colour.BLACK else 'R'

class Knight(Piece):
    def __init__(self, colour, row, column):
        super().__init__(colour, row, column)
    def get_representation(self):
        return 'n' if self.colour == Colour.BLACK else 'N'

class Bishop(Piece):
    def __init__(self, colour, row, column):
        super().__init__(colour, row, column)
    def get_representation(self):
        return 'b' if self.colour == Colour.BLACK else 'B'

class Queen(Piece):
    def __init__(self, colour, row, column):
        super().__init__(colour, row, column)
    def get_representation(self):
        return 'q' if self.colour == Colour.BLACK else 'Q'

class King(Piece):
    def __init__(self, colour, row, column):
        super().__init__(colour, row, column)
    def get_representation(self):
        return 'k' if self.colour == Colour.BLACK else 'K'
    
