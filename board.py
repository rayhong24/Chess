from strings import *
from enums import Colour
from enums import File
from utils import *
from Pieces.piece import Piece
from Pieces.pawn import Pawn
from Pieces.rook import Rook
from Pieces.knight import Knight
from Pieces.bishop import Bishop
from Pieces.queen import Queen
from Pieces.king import King

class Board:
    def __init__(self):
        self.board = [[None]*8 for _ in range(8)]
        # Maybe change to a dictionary
        self.white_pieces = set()
        self.black_pieces = set()

        self.__setup_board()

    def __add_piece(self, piece_type: Piece, colour: Colour, i: int, j: int):
        piece = piece_type(colour, i, j)
        self.board[i][j] = piece
        
        if colour == Colour.WHITE:
            self.white_pieces.add(piece)
        else:
            self.black_pieces.add(piece)


    def __setup_board(self):
        # Place pawns
        for j in range(8):
            self.__add_piece(Pawn, Colour.BLACK, 1, j)

            self.__add_piece(Pawn, Colour.WHITE, 6, j)

        # Place rooks
        self.__add_piece(Rook, Colour.BLACK, 0, 0)
        self.__add_piece(Rook, Colour.BLACK, 0, 7)
        self.__add_piece(Rook, Colour.WHITE, 7, 0)
        self.__add_piece(Rook, Colour.WHITE, 7, 7)

        # Place knights
        self.__add_piece(Knight, Colour.BLACK, 0, 1)
        self.__add_piece(Knight, Colour.BLACK, 0, 6)
        self.__add_piece(Knight, Colour.BLACK, 7, 1)
        self.__add_piece(Knight, Colour.BLACK, 7, 6)

        # Place bishops
        self.__add_piece(Bishop, Colour.BLACK, 0, 2)
        self.__add_piece(Bishop, Colour.BLACK, 0, 5)
        self.__add_piece(Bishop, Colour.BLACK, 7, 2)
        self.__add_piece(Bishop, Colour.BLACK, 7, 5)

        # Place Queens
        self.__add_piece(Queen, Colour.BLACK, 0, 3)
        self.__add_piece(Queen, Colour.WHITE, 7, 3)

        # Place Kings
        self.__add_piece(King, Colour.BLACK, 0, 4)
        self.__add_piece(King, Colour.BLACK, 7, 4)
    
    def __get_square_representation(self, val: Piece) -> str:
        if val is None:
            return ""
        else:
            return val.get_representation()

    def print_board(self):
        for i, row in enumerate(self.board):
            print(f"{i} {['{:^3}'.format(self.__get_square_representation(val)) for val in row]}")
        print()
        print(f"  {['{:^3}'.format(File(i).name) for i in range(8)]}")

        print("="*57)
    
    def is_inbounds(self, i: int, j: int) -> bool:
        return (0 <= i < 8) and (0 <= j < 8)

    def remove_piece(self, piece: Piece):
        if piece.colour == Colour.BLACK:
            self.black_pieces.remove(piece)
        else:
            self.white_pieces.remove(piece)
    def add_piece(self, piece: Piece):
        if piece.colour == Colour.BLACK:
            self.black_pieces.add(piece)
        else:
            self.white_pieces.add(piece)

    def handle_move(self, move: str) -> bool:
        def get_new_piece(piece: str, colour: Colour) -> str:
            if piece == 'q':
                return Queen(colour, new_i, new_j)
            elif piece == 'r':
                return Rook(colour, new_i, new_j)
            elif piece == 'b':
                return Bishop(colour, new_i, new_j)
            elif piece == 'n':
                return Knight(colour, new_i, new_j)
        # TODO add check to make sure it is a valid move

        start_i, start_j, new_i, new_j = get_coords(move)
        if len(move) == 4:
            self.move_piece(start_i, start_j, new_i, new_j)
            # check king side castling (assuming valid)
            if type(self.board[start_i][start_j]) == King\
            and start_j == 4 and new_j == 6:
                # move rook (assumes rook is in the right place)
                self.move_piece(start_i, 7, start_i, 5)

            # check queen side castling (assuming valid)
            if type(self.board[start_i][start_j]) == King\
            and start_j == 4 and new_j == 2:
                # move rook (assumes rook is in the right place)
                self.move_piece(start_i, 0, start_i, 3)

        elif len(move) == 5:
            colour = self.board[start_i][start_j].colour
            self.remove_piece(self.board[start_i][start_j])
            self.board[start_i][start_j] = None
            new_piece = get_new_piece(move[4], colour)
            self.board[new_i][new_j] = new_piece
            self.add_piece(new_piece)


    def move_piece(self, orig_i: int, orig_j: int, new_i: int, new_j: int):
        self.board[orig_i][orig_j].move(new_i, new_j)
        if self.board[new_i][new_j] != None:
            self.remove_piece(self.board[new_i][new_j])
        self.board[orig_i][orig_j], self.board[new_i][new_j] = None, self.board[orig_i][orig_j]