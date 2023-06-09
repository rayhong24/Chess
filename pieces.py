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
        for di in range(1, moves_forward+1):
            i, j = self.row+(di*direction), self.column
            if 0<=i<=7 and 0<=j<=7 and board[i][j] == None: #TODO: Find better way for bounds error-checking
                valid_moves.append((i, j))
        
        # check captures
        i = self.row+direction
        j_left = self.column-1
        j_right = self.column+1
        if 0<=i<=7 and 0<=j_left<=7:
            square_to_check1 = board[i][j_left]
            if square_to_check1 is not None and square_to_check1.colour != self.colour:
                valid_moves.append((i, j_left))
        
        if 0<=i<=7 and 0<=j_right<=7:
            square_to_check2 = board[i][j_right]
            if  square_to_check2 is not None and square_to_check2.colour != self.colour:
                valid_moves.append((i, j_right))

        # TODO: En passant
        
        print(valid_moves)

        return valid_moves

    def move(self, board, new_i, new_j):
        super().check_move_errors(board, self.get_representation())               
        
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
    
