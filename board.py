from Pieces.piece import Piece

from strings import *
from enums import *
from coords import Coords

from Pieces.pieceFactory import PieceFactory

class Board:
    def __init__(self):
        self._piece_factory = PieceFactory()
        self._board = [[None]*8 for _ in range(8)]

    def get_square(self, coords: Coords) -> Piece:
        return self._board[8-coords.rank][coords.file.value]

    def set_square(self, value, coords: Coords):
        self._board[8-coords.rank][coords.file.value] = value

    # Input: string from a fenstring (i.e. rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR)
    def set_fenstr(self, s: str) -> None:
        for i, row in enumerate(s.split('/')):
            j = 0
            for c in row:
                if c.isnumeric():
                    for _ in range(int(c)):
                        self._board[i][j] = None
                        j += 1
                else:
                    self._board[i][j] = self._piece_factory.init_piece(c, Coords(8-i, File(j)))
                    j += 1


    def get_square_representation(self, val) -> str:
        if val is None:
            return ""
        else:
            return val.get_representation()

    def print_board(self):
        for i, row in enumerate(self._board):
            print(f"{8-i} {['{:^3}'.format(self.get_square_representation(val)) for val in row]}")
        print()
        print(f"  {['{:^3}'.format(File(i).name) for i in range(8)]}")

    def get_moves(self, player: Colour, enpassant_coords: Coords):
        moves = []

        for coords in self._all_squares_iterator():
            piece = self.get_square(coords)

            if piece and piece.colour == player:
                moves.extend(piece.get_candidate_moves())

        return moves 



    def _all_squares_iterator(self) -> list[Coords]:
        coords = []
        for i in range(1,len(self._board)+1):
            for j in range(len(self._board[0])):
                coords.append(Coords(i, File(j)))

        return coords

    def _remove_piece(self, i, j):
        self.board[i][j] = None