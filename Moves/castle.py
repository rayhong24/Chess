from GameClasses.board import Board
from Moves.move import Move
class Castle(Move):
    def __init__(self, player_to_move, start_coords, capture, end_coords, rook_start_coords, rook_end_coords):
        super().__init__(player_to_move, start_coords, capture, end_coords)

        self.rook_start_coords = rook_start_coords
        self.rook_end_coords = rook_end_coords

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



