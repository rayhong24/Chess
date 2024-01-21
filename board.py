from strings import *
from enums import *
from utils import *

from Moves.move import Move
from Pieces.piece import Piece
from Pieces.pawn import Pawn
from Pieces.rook import Rook
from Pieces.knight import Knight
from Pieces.bishop import Bishop
from Pieces.queen import Queen
from Pieces.king import King

class Board:
    def __init__(self):
        self.board: [Piece] = [[None]*8 for _ in range(8)]

        # piece sets will probably be moved to player class later
        self.white_pieces = set()
        self.black_pieces = set()

    def get_piece_type(self, s: str) -> Piece:
        piece_dict = {'p': Pawn, 'r': Rook, 'n': Knight, 'b': Bishop, 'q':Queen, 'k':King}

        piece_type = piece_dict[s.lower()]
        piece_colour = Colour.WHITE if s.isupper() else Colour.BLACK

        return piece_type, piece_colour

    def __add_piece(self, piece_type: Piece, colour: Colour, i: int, j: int):
        piece = piece_type(colour, i, j)
        self.board[i][j] = piece
        
        if colour == Colour.WHITE:
            self.white_pieces.add(piece)
        else:
            self.black_pieces.add(piece)


    # Input: string from a fenstring (i.e. rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR)
    def add_fenstr_pieces(self, s: str) -> None:
        for i, row in enumerate(s.split('/')):
            j = 0
            for c in row:
                if c.isnumeric():
                    for _ in range(int(c)):
                        self.board[i][j] = None
                        j += 1
                else:
                    piece_type, colour = self.get_piece_type(c)
                    self.__add_piece(piece_type, colour, i, j)
                    j += 1

    # Input: string from a fenstring (ei. KQkq or -)
    #TODO: Refactor (see comments in function)
    def set_castling_rights(self, s: str) -> None:
        if s == '-':
            for piece in self.white_pieces:
                if type(piece) == Rook:
                    piece.has_moved = True
            for piece in self.black_pieces:
                if type(piece) == Rook:
                    piece.has_moved = True

        # TODO: Change implementation to not use hardcoded indices
        # TODO: Error Checking (e.i. make sure rook is actually on board[7][7])
        if 'K' in s:
            if self.board[7][7]:
                self.board[7][7].has_moved = False
            if self.board[7][4]:
                self.board[7][4].has_moved = False
        elif 'Q' in s:
            if self.board[7][0]:
                self.board[7][0].has_moved = False
            if self.board[7][4]:
                self.board[7][4].has_moved = False
        elif 'k' in s:
            if self.board[0][7]:
                self.board[0][7].has_moved = False
            if self.board[0][4]:
                self.board[0][4].has_moved = False
        elif 'q' in s:
            self.board[0][0].has_moved = False
            self.board[0][4].has_moved = False

    def get_square_representation(self, val: Piece) -> str:
        if val is None:
            return ""
        else:
            return val.get_representation()

    def print_board(self):
        for i, row in enumerate(self.board):
            print(f"{8-i} {['{:^3}'.format(self.get_square_representation(val)) for val in row]}")
        print()
        print(f"  {['{:^3}'.format(File(i).name) for i in range(8)]}")

    def is_inbounds(self, i: int, j: int) -> bool:
        return (0 <= i < 8) and (0 <= j < 8)

    def remove_piece_from_sets(self, piece: Piece):
        if piece.colour == Colour.BLACK:
            self.black_pieces.remove(piece)
        else:
            self.white_pieces.remove(piece)
    def add_piece_to_sets(self, piece: Piece):
        if piece.colour == Colour.BLACK:
            self.black_pieces.add(piece)
        else:
            self.white_pieces.add(piece)

    def move_piece(self, orig_i: int, orig_j: int, new_i: int, new_j: int):
        if self.board[orig_i][orig_j] is not None:
            self.board[orig_i][orig_j].move(new_i, new_j)

        if self.board[new_i][new_j] != None:
            self.remove_piece_from_sets(self.board[new_i][new_j])
        self.board[orig_i][orig_j], self.board[new_i][new_j] = None, self.board[orig_i][orig_j]

    # assumes promotion is valid
    def promote_piece(self, i, j, piece_str):
        colour = self.board[i][j].colour

        piece_type = self.get_piece_type(piece_str)[0]
        self.board[i][j] = piece_type(colour, i, j)

