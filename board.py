from strings import *
from enums import *
from coords import Coords

from Pieces.pieceFactory import PieceFactory

class Board:
    def __init__(self):
        self.piece_factory = PieceFactory()
        self.board = [[None]*8 for _ in range(8)]

    def get_square(self, coords: Coords):
        return self.board[8-coords.rank][coords.file.value]

    def set_square(self, value, coords: Coords):
        self.board[8-coords.rank][coords.file.value] = value

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
                    j += 1


    # def get_square_representation(self, val) -> str:
    #     if val is None:
    #         return ""
    #     else:
    #         return val.get_representation()

    def print_board(self):
        for i, row in enumerate(self.board):
            print(f"{8-i} {['{:^3}'.format(self.get_square_representation(val)) for val in row]}")
        print()
        print(f"  {['{:^3}'.format(File(i).name) for i in range(8)]}")

    def _is_inbounds(self, i: int, j: int) -> bool:
        return (0 <= i < 8) and (0 <= j < 8)

    def _remove_piece(self, i, j):
        self.board[i][j] = None