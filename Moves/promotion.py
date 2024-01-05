from Moves.move import Move

class Promotion(Move):
    def __init__(self, player_to_move, start_coords, capture, end_coords, promotion_piece):
        super().__init__(
           player_to_move,
           'P',
           start_coords,
           capture,
           end_coords
        )

        self.promotion_piece = promotion_piece
