from Moves.move import Move
class Castle(Move):
    def __init__(self, player_to_move, start_coords, capture, end_coords, rook_start_coords, rook_end_coords):
        super().__init__(player_to_move, start_coords, capture, end_coords)

        self.rook_start_coords = rook_start_coords
        self.rook_end_coords = rook_end_coords