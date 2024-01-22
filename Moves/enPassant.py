from Moves.move import Move

class EnPassant(Move):
    def __init__(self, player_to_move, start_coords, end_coords) -> None:
        super().__init__(player_to_move, 'P', start_coords, True, end_coords)

    def __eq__(self, other):
        if not isinstance(other, EnPassant):
            return False
        return super().__eq__(other)

    def __hash__(self) -> int:
        return super().__hash__()

    def __repr__(self):
        return super().__repr__()

    def check_valid(self, game) -> bool:
        return super().check_valid(game)

    def make_move(self, game) -> bool:
        if not self.check_valid(game):
            return False

        super().make_move(game)

        game.board.remove_piece(
            self.start_coords[0],
            self.end_coords[1],
        )




        
