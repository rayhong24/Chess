from Moves.move import Move

class Promotion(Move):
    def __init__(self, player_to_move, start_coords, capture, end_coords, promotion_str):
        super().__init__(player_to_move, start_coords, capture, end_coords)

        self.promotion_str = promotion_str
        

    def __str__(self):
        s = "{}{}{}".format(
            self.start_coords,
            self.end_coords,
            self.promotion_str
        )
    
    def apply(self, board: Board):
        super().apply(board)

        piece = board.get_square(self.end_coords)
        piece.promote(self.promotion_str)


