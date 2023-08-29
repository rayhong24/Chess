from enums import File
from enums import Colour
from Pieces.piece import Piece
from Pieces.piece import Piece
from Pieces.pawn import Pawn
from Pieces.rook import Rook
from Pieces.knight import Knight
from Pieces.bishop import Bishop
from Pieces.queen import Queen
from Pieces.king import King

def get_coords(move: str) -> tuple[int]:
    start_i, start_j = int(move[1]), File[move[0]].value
    new_i, new_j = int(move[3]), File[move[2]].value

    return (start_i, start_j, new_i, new_j)

def get_piece_type(s: str) -> Piece:
    piece_dict = {'p': Pawn, 'r': Rook, 'n': Knight, 'b': Bishop, 'q':Queen, 'k':King}

    piece_type = piece_dict[s.lower()]
    piece_colour = Colour.WHITE if s.isupper() else Colour.BLACK

    return piece_type, piece_colour
    