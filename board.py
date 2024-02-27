from strings import *
from enums import *
from coords import Coords

from Pieces.pieceFactory import PieceFactory

class Board:
    def __init__(self):
        self.piece_factory = PieceFactory()
        self.board = [[None]*8 for _ in range(8)]

        # piece sets will probably be moved to player class later
        self.white_pieces = set()
        self.black_pieces = set()


    def _add_piece(self, piece_str, coords: Coords):
        piece = self.piece_factory.init_piece(piece_str, coords)
        self.set_square(piece, coords)
        
        self.get_player_pieces(piece.colour).add(piece)

    def get_square(self, coords: Coords):
        return self.board[coords.rank][coords.file.value]

    def set_square(self, value, coords: Coords):
        self.board[coords.rank][coords.file.value] = value

    def get_player_pieces(self, colour):
        return self.white_pieces if colour == Colour.WHITE else self.black_pieces

    # Input: string from a fenstring (i.e. rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR)
    def add_fenstr_pieces(self, s: str) -> None:
        self.white_pieces = set()
        self.black_pieces = set()
        for i, row in enumerate(s.split('/')):
            j = 0
            for c in row:
                if c.isnumeric():
                    for _ in range(int(c)):
                        self.board[i][j] = None
                        j += 1
                else:
                    self._add_piece(c, Coords(i, File(j)))
                    j += 1

    # Input: string from a fenstring (ei. KQkq or -)
    #TODO: Refactor (see comments in function)
    def set_castling_rights(self, s: str) -> None:
        # if s == '-':
        #     for piece in self.white_pieces:
        #         if type(piece) == Rook:
        #             piece.has_moved = True
        #     for piece in self.black_pieces:
        #         if type(piece) == Rook:
        #             piece.has_moved = True

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

    def get_square_representation(self, val) -> str:
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

    def remove_piece_from_sets(self, piece):
        self.get_player_pieces(piece.colour).remove(piece)

    def add_piece_to_sets(self, piece):
        self.get_player_pieces(piece.colour).add(piece)

    def remove_piece(self, i, j):
        self.remove_piece_from_sets(self.board[i][j])
        self.board[i][j] = None

    def move_piece(self, start_coords: Coords, end_coords: Coords):
        start_square = self.get_square(start_coords)
        end_square = self.get_square(end_coords)
        if start_square is not None:
            start_square.move(end_coords)

        if end_square != None:
            self.remove_piece_from_sets(end_square)
        
        self.set_square(None, start_coords)
        self.set_square(start_square, end_coords)

    # assumes promotion is valid
    def promote_piece(self, i, j, piece_str):
        colour = self.board[i][j].colour

        piece_str = piece_str.upper() if colour == Colour.WHITE else piece_str.lower()

        new_piece = self.piece_factory.init_piece(piece_str, i, j)

        self.board[i][j] = new_piece

