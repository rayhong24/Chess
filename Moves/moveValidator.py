from Moves.moveGenerator import MoveGenerator

from GameClasses.board import Board
from enums import Colour

from Pieces.king import King

class moveValidator():
    def __init__(self):
        self.move_generator = MoveGenerator()

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


