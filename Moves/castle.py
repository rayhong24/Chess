from GameClasses.board import Board
from Moves.move import Move

from coords import Coords

class Castle(Move):
    def __init__(self, player_to_move, start_coords, end_coords):
        super().__init__(player_to_move, start_coords, False, end_coords)

        if str(start_coords) == "e1":
            if str(end_coords) == "g1":
                self.rook_start_coords = Coords.init_from_str("h1")
                self.rook_end_coords = Coords.init_from_str("f1")
            elif str(end_coords) == "c1":
                self.rook_start_coords = Coords.init_from_str("a1")
                self.rook_end_coords = Coords.init_from_str("d1")
        elif str(start_coords) == "e8":
            if str(end_coords) == "g8":
                self.rook_start_coords = Coords.init_from_str("h8")
                self.rook_end_coords = Coords.init_from_str("f8")
            elif str(end_coords) == "c8":
                self.rook_start_coords = Coords.init_from_str("a8")
                self.rook_end_coords = Coords.init_from_str("d8")



    def apply(self, board: Board):
        super().apply(board)

        rook = board.get_square(self.rook_start_coords)

        rook.has_moved = True
        board.set_square(None, self.rook_start_coords)
        board.set_square(rook, self.rook_end_coords)

    def undo(self, board: Board):
        super().undo(board)

        rook = board.get_square(self.rook_end_coords)

        board.set_square(rook, self.rook_start_coords)
        board.set_square(None, self.rook_end_coords)



