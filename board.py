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
        self.board: [Piece] = [[None]*8 for _ in range(8)]

        # piece sets will probably be moved to player class later
        self.white_pieces = set()
        self.black_pieces = set()

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
                    piece_type, colour = get_piece_type(c)
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
            self.board[7][7].has_moved = False
            self.board[7][4].has_moved = False
        elif 'Q' in s:
            self.board[7][0].has_moved = False
            self.board[7][4].has_moved = False
        elif 'k' in s:
            self.board[0][7].has_moved = False
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

    # INPUT: move - string in algebraic chess notation
    # INPUT: player_to_move - colour used for error checking
    # TODO: Refactor
    def handle_move(self, move: str, player_to_move: Colour) -> bool:
        def get_new_piece(piece: str, colour: Colour) -> str:
            if piece == 'Q':
                return Queen(colour, end_i, end_j)
            elif piece == 'R':
                return Rook(colour, end_i, end_j)
            elif piece == 'B':
                return Bishop(colour, end_i, end_j)
            elif piece == 'N':
                return Knight(colour, end_i, end_j)
        # TODO add check to make sure it is a valid move
        print(f"{move=}")

        start_i = start_j = end_i = end_j = None
        piece = 'P'
        promotion = False

        if move == "O-O":
            start_i = 7 if player_to_move == Colour.WHITE else 0
            start_j = 4

            end_i = 7 if player_to_move == Colour.WHITE else 0
            end_j = 6

            piece = 'K' 
        elif move == "O-O-O":
            start_i = 0 if player_to_move == Colour.WHITE else 7
            start_j = 4

            end_i = 0 if player_to_move == Colour.WHITE else 7
            end_j = 2

            piece = 'K' 
        elif "-" in move:
            start, end = move.split('-')

            if len(start) == 3:
                piece = start[0]
                start_i, start_j = to_coords(start[1:])
            else:
                start_i, start_j = to_coords(start[:2])

            end_i, end_j = to_coords(end[:2])
            if '=' in end:
                promotion = True


        elif "x" in move:
            start, end = move.split('x')

            if len(start) == 3:
                piece = start[0]
                start_i, start_j = to_coords(start[1:])
            else:
                start_i, start_j = to_coords(start[:2])

            end_i, end_j = to_coords(end[:2])

            if '=' in end:
                promotion = True
        else:
            return False

        # error checking
        if not self.board[start_i][start_j] or \
        self.board[start_i][start_j].colour != player_to_move or \
        self.board[start_i][start_j].get_representation().upper() != piece or \
        move not in self.board[start_i][start_j].get_moves(self.board):
            return False

        self._move_piece(start_i, start_j, end_i, end_j)


        if move == "O-O":
            # Move Rook
            self._move_piece(start_i, 7, end_i, 5)
        elif move == "O-O-O":
            # Move Rook
            self._move_piece(start_i, 0, end_i, 3)

        if promotion: 
            _, piece_type = end.split('=')
            colour = self.board[end_i][end_j].colour
            self.remove_piece(self.board[end_i][end_j])
            self.board[end_i][end_j] = None
            new_piece = get_new_piece(piece_type, colour)
            self.board[end_i][end_j] = new_piece
            self.add_piece(new_piece)

        return True


    def _move_piece(self, orig_i: int, orig_j: int, new_i: int, new_j: int):
        if self.board[orig_i][orig_j] is not None:
            self.board[orig_i][orig_j].move(new_i, new_j)

        if self.board[new_i][new_j] != None:
            self.remove_piece(self.board[new_i][new_j])
        print(f"{new_i=}, {new_j=}")
        self.board[orig_i][orig_j], self.board[new_i][new_j] = None, self.board[orig_i][orig_j]