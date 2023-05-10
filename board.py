from enums import Colour
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
            self.board[1][j] = Pawn(Colour.BLACK, 1, j)
            self.board[6][j] = Pawn(Colour.WHITE, 6, j)

        # Place rooks
        self.board[0][0] = Rook(Colour.BLACK, 0, 0)
        self.board[0][7] = Rook(Colour.BLACK, 0, 7)
        self.board[7][0] = Rook(Colour.WHITE, 7, 0)
        self.board[7][7] = Rook(Colour.WHITE, 7, 7)

        # Place knights
        self.board[0][1] = Knight(Colour.BLACK, 0, 1)
        self.board[0][6] = Knight(Colour.BLACK, 0, 6)
        self.board[7][1] = Knight(Colour.WHITE, 7, 1)
        self.board[7][6] = Knight(Colour.WHITE, 7, 6)

        # Place bishops
        self.board[0][2] = Bishop(Colour.BLACK, 0, 2)
        self.board[0][5] = Bishop(Colour.BLACK, 0, 5)
        self.board[7][2] = Bishop(Colour.WHITE, 7, 2)
        self.board[7][5] = Bishop(Colour.WHITE, 7, 5)

        # Place Queens
        self.board[0][3] = Queen(Colour.BLACK, 0, 3)
        self.board[7][3] = Queen(Colour.WHITE, 7, 3)

        # Place Kings
        self.board[0][4] = King(Colour.BLACK, 0, 4)
        self.board[7][4] = King(Colour.WHITE, 7, 4)
    
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

