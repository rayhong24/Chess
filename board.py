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
        # Maybe change to a dictionary
        self.white_pieces = []
        self.black_pieces = []

        self.__setup_board()

    def __add_piece(self, piece_type, colour, i, j):
        piece = piece_type(colour, i, j)
        self.board[i][j] = piece
        
        if colour == Colour.WHITE:
            self.white_pieces.append(piece)
        else:
            self.black_pieces.append(piece)


    def __setup_board(self):
        # Place pawns
        for j in range(8):
            self.__add_piece(Pawn, Colour.BLACK, 1, j)

            self.__add_piece(Pawn, Colour.WHITE, 6, j)

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
        for i, row in enumerate(self.board):
            print(f"{i} {['{:^3}'.format(self.__get_square_representation(val)) for val in row]}")
        print()
        print(f"  {['{:^3}'.format(str(i)) for i in range(8)]}")

        print("="*57)
    
    def is_inbounds(self, i, j):
        return (0 <= i < 8) and (0 <= j < 8)


    def move_piece(self, orig_i, orig_j, new_i, new_j):
        self.board[orig_i][orig_j].move(self.board, new_i, new_j)
        self.board[orig_i][orig_j], self.board[new_i][new_j] = None, self.board[orig_i][orig_j]

