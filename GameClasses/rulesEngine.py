from Moves.moveGenerator import MoveGenerator
from Moves.move import Move

from GameClasses.board import Board
from GameClasses.game import Game

from enums import Colour

from Pieces.king import King

class rulesEngine():
    def __init__(self):
        self.move_generator = MoveGenerator()

    def get_valid_moves(self, game: Game):
        pseudo_moves = self.move_generator.generate_pseudo_legal_moves(game.board, game.state.to_move)

        valid_moves = list(filter(lambda m: not self.does_leave_player_in_check(game, m), pseudo_moves))

        return valid_moves


    def is_in_check(self, board: Board, player: Colour):
        player_in_check = False

        # Find the player King
        king_coords = None
        for coords in board.all_squares_iterator():
            piece = board.get_square(coords)

            if piece and type(piece) == King and piece.colour == player:
                king_coords = coords
                break
        else:
            print("Error: No king found")


        # Check if king can be captured
        for coords in board.all_squares_iterator():
            piece = board.get_square(coords)

            if piece and piece.colour != player:
                for enemy_moves in self.move_generator.get_piece_moves(board, piece, coords):
                    if enemy_moves.end_coords == king_coords:
                        player_in_check = True
                        break


        return player_in_check

    def is_checkmate(self, game: Game):
        in_check = self.is_in_check(game.board, game.state.to_move)
        moves = self.get_valid_moves(game)

        return in_check and len(moves) == 0

    def does_leave_player_in_check(self, game: Game, move: Move):
        game.make_move(move)

        out = self.is_in_check(game.board, move.player_to_move)
        game.undo_move()

        return out
        


