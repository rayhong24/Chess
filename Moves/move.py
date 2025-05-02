from enums import Colour
from coords import Coords

class Move:
    def __init__(self, player_to_move, piece_str, start_coords, capture, end_coords) -> None:
        self.player_to_move = player_to_move

        self.piece_str = piece_str

        self.start_coords = start_coords
        self.capture = capture
        self.end_coords = end_coords
    
    def __hash__(self) -> int:
        return hash(repr(self))

    def __eq__(self, other) -> bool:
        if not isinstance(other, Move):
            return False
        return self.player_to_move == other.player_to_move\
        and self.piece_str == other.piece_str\
        and self.start_coords == other.start_coords\
        and self.capture == other.capture\
        and self.end_coords == other.end_coords

    def __repr__(self):
        repr_str = "{}{}".format(
            self.player_to_move,
            self.__str__()
        )

        return repr_str

    def __str__(self):
        piece_str = "" if self.piece_str == 'P' else self.piece_str
        capture_str = "x" if self.capture else "-"
        s = "{}{}{}{}".format(
            piece_str,
            self.start_coords,
            capture_str,
            self.end_coords
        )

        return s

    def long_algebraic(self) -> str:
        return f"{self.start_coords}{self.end_coords}"
