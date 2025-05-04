from enums import Colour
# from Pieces.piece import Piece

class Move:
    def __init__(self, player_to_move, capture) -> None:
        self.player_to_move: Colour = player_to_move

        self.capture = capture

        # self.piece_type: Piece = piece

        # self.capture = capture
        # self.end_coords = end_coords

        # self.promotion = promotion
    
    def __hash__(self) -> int:
        return hash(repr(self))

    def __eq__(self, other) -> bool:
        if not isinstance(other, Move):
            return False
        return self.player_to_move == other.player_to_move
        # and self.piece_type == other.piece_type
        # and self.start_coords == other.start_coords\
        # and self.capture == other.capture\
        # and self.end_coords == other.end_coords

    def __repr__(self):
        repr_str = "{}".format(
            self.__str__()
        )

        return repr_str

    def __str__(self):
        s = "{}".format(
            self.player_to_move
        )
        # capture_str = "x" if self.capture else "-"
        # s = "{}{}{}".format(
        #     self.start_coords,
        #     capture_str,
        #     self.end_coords
        # )

        return s

    # def long_algebraic(self) -> str:
    #     return f"{self.start_coords}{self.end_coords}"
