from enums import Colour
from GameClasses.board import Board
# from Pieces.piece import Piece

class Move:
    def __init__(self, player_to_move, start_coords, capture, end_coords) -> None:
        self.player_to_move: Colour = player_to_move

        self.capture = capture

        # self.piece_type: Piece = piece

        self.start_coords = start_coords
        self.capture = capture
        self.end_coords = end_coords

        self.end_piece = None
        self.has_piece_moved_before = False
    
    def __hash__(self) -> int:
        return hash(repr(self))

    def __eq__(self, other) -> bool:
        if not isinstance(other, Move):
            return False
        return self.player_to_move == other.player_to_move\
        and self.start_coords == other.start_coords\
        and self.capture == other.capture\
        and self.end_coords == other.end_coords
        # and self.piece_type == other.piece_type

    def __repr__(self):
        repr_str = "{}".format(
            self.__str__()
        )

        return repr_str

    def __str__(self):
        s = "{}{}".format(
            self.start_coords,
            self.end_coords
        )

        return s

    def apply(self, board: Board):
        piece = board.get_square(self.start_coords)
        end_piece = board.get_square(self.end_coords)

        # Used for undo
        self.end_piece = end_piece
        self.has_piece_moved_before = piece.has_moved

        piece.has_moved = True
        board.set_square(None, self.start_coords)
        board.set_square(piece, self.end_coords)
        return

    def undo(self, board: Board):
        piece = board.get_square(self.end_coords)

        board.set_square(piece, self.start_coords)
        board.set_square(self.end_piece, self.end_coords)


    # def long_algebraic(self) -> str:
    #     return f"{self.start_coords}{self.end_coords}"
