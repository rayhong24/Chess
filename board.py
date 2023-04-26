from enums import Colour
from pieces import Piece
from pieces import Pawn
from pieces import Rook
from pieces import Knight
from pieces import Bishop
from pieces import Queen
from pieces import King

class Board:
    def __init__(self):
        self.board = [[None]*8 for _ in range(8)]

        # Place pawns
        for j in range(8):
            self.board[1][j] = Pawn(Colour.BLACK)
            self.board[6][j] = Pawn(Colour.WHITE)

        # Place rooks
        self.board[0][0] = Rook(Colour.BLACK)
        self.board[0][-1] = Rook(Colour.BLACK)
        self.board[-1][0] = Rook(Colour.WHITE)
        self.board[-1][-1] = Rook(Colour.WHITE)

        # Place knights
        self.board[0][1] = Knight(Colour.BLACK)
        self.board[0][6] = Knight(Colour.BLACK)
        self.board[-1][1] = Knight(Colour.WHITE)
        self.board[-1][6] = Knight(Colour.WHITE)

        # Place bishops
        self.board[0][2] = Bishop(Colour.BLACK)
        self.board[0][5] = Bishop(Colour.BLACK)
        self.board[-1][2] = Bishop(Colour.WHITE)
        self.board[-1][5] = Bishop(Colour.WHITE)

        # Place Queens
        self.board[0][3] = Queen(Colour.BLACK)
        self.board[-1][3] = Queen(Colour.WHITE)

        # Place Kings
        self.board[0][4] = King(Colour.BLACK)
        self.board[-1][4] = King(Colour.WHITE)
    
    def __get_square_representation(self, val):
        if val is None:
            return ""
        else:
            return val.get_representation()

    def print_board(self):
        for row in self.board:
            print(["{:^3}".format(self.__get_square_representation(val)) for val in row])

        print("="*57)

    def move_piece(self, orig_i, orig_j, new_i, new_j):
        self.board[orig_i][orig_j], self.board[new_i][new_j] = None, self.board[orig_i][orig_j]

