from Pieces.piece import Piece
from Pieces.moveCandidate import MoveCandidate
from enums import *
from coords import Coords

class King(Piece):
    def __init__(self, colour: Colour, coords: Coords) -> None:
        super().__init__(colour, coords)

    def get_representation(self) -> str:
        return 'k' if self.colour == Colour.BLACK else 'K'

    def get_candidate_moves(self, coords: Coords):
        # list of tuples of new coordinates the piece can go
        candidates = []

        for di, dj in [
            [-1, -1], [-1, 1], [1, -1], [1, 1],
            [-1, 0], [1, 0], [0, -1], [0, 1]
            ]:

            # Normal
            candidate = MoveCandidate(di, dj, 1)
            candidates.append(candidate)

            # Castling
            if (str(coords) == "e1" or str(coords) == "e8") and dj == 0:
                castle_cand = MoveCandidate(di*2, dj, 1, False, False)
                candidates.append(castle_cand)

        return candidates
    # def get_moves(self, game) -> [str]:
    #     # list of tuples of new coordinates the piece can go
    #     valid_moves = []

    #     for surrounding_coords in self.coords.get_surrounding():
    #         square = game.board.get_square(surrounding_coords)
    #         if square == None or square.colour != self.colour:
    #             move = self.move_factory.init_normal_move(
    #                 self.colour,
    #                 'K',
    #                 self.coords,
    #                 square is not None,
    #                 surrounding_coords
    #             )
    #             valid_moves.append(move)

    #     # Check king side castle
    #     if self.coords.file == File['e'] and not self.has_moved \
    #         and game.board.get_square(Coords(self.coords.rank, File['f'])) == None \
    #         and game.board.get_square(Coords(self.coords.rank, File['g'])) == None \
    #         and type(game.board.get_square(Coords(self.coords.rank, File['h']))) == Rook \
    #         and not game.board.get_square(Coords(self.coords.rank, File['h'])).has_moved:
    #             move = self.move_factory.init_move_from_str("O-O", self.colour, game)
    #             valid_moves.append(move)

    #     # Check queen side castle
    #     if self.coords.file == File['e'] and not self.has_moved \
    #         and game.board.get_square(Coords(self.coords.rank, File['d'])) == None \
    #         and game.board.get_square(Coords(self.coords.rank, File['c'])) == None \
    #         and game.board.get_square(Coords(self.coords.rank, File['b'])) == None \
    #         and type(game.board.get_square(Coords(self.coords.rank, File['a']))) == Rook \
    #         and not game.board.get_square(Coords(self.coords.rank, File['a'])).has_moved:
    #             move = self.move_factory.init_move_from_str("O-O-O", self.colour, game)
    #             valid_moves.append(move)
        
    #     return valid_moves