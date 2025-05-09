from GameClasses.game import Game
from Moves.move import Move

class MoveGenerator():
    def get_all_moves(self, game: Game):
        moves = []

        for coords in game.board.all_squares_iterator():
            piece = game.board.get_square(coords)

            if piece and piece.colour == game.state.to_move:
                moves.extend(self.get_piece_moves(game, piece, coords))

        return moves 

    def get_piece_moves(self, game, piece, coords):
        moves = []

        candidate_moves = piece.get_candidate_moves(coords)

        for candidate in candidate_moves:
            for end_coords in candidate.generate_coords(coords):
                if game.board.get_square(end_coords) == None:
                    if candidate.capture_forced:
                        break
                    moves.append(Move(piece.colour, coords, False, end_coords))

                else:
                    blocking_piece = game.board.get_square(end_coords)
                    if candidate.capture_allowed and blocking_piece.colour != piece.colour:
                        moves.append(Move(piece.colour, coords, True, end_coords))
                    break

        return moves