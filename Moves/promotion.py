from Moves.move import Move

class Promotion(Move):
    def __init__(self, player_to_move, start_coords, capture, end_coords, promotion_piece_str):
        super().__init__(
           player_to_move,
           'P',
           start_coords,
           capture,
           end_coords
        )

        self.promotion_piece_str = promotion_piece_str

    def __eq__(self, other):
        if not isinstance(other, Promotion):
            return False
        return super().__eq__(other) and self.promotion_piece_str == other.promotion_piece_str

    def __repr__(self):
        repr_str = super().__repr__() + self.promotion_piece
        return repr_str

    def __str__(self):
        s = "{}={}".format(
            super().__str__(),
            self.promotion_piece_str
        )

        return s

    def check_valid(self, game) -> bool:
        return super().check_valid(game)

    def make_move(self, game) -> bool:
        if not self.check_valid(game):
            return False

        game.board.move_piece(
            self.start_coords[0],
            self.start_coords[1],
            self.end_coords[0],
            self.end_coords[1]
        )

        game.board.promote_piece(
            self.end_coords[0],
            self.end_coords[1],
            self.promotion_piece_str   
        )

        game.switch_player_turn()



        

        