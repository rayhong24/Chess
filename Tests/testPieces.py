import unittest

from enums import *
from game import Game
from Moves.moveFactory import MoveFactory
from Pieces.piece import Piece 

class PiecesTestCases:
    class TestPieces(unittest.TestCase):
        def setUp(self):
            self.game = Game()
            self.move_factory = MoveFactory()

        def check_moves(self, piece: Piece, expected_moves: list):
            piece_moves = set(piece.get_moves(self.game.board.board))

            for move in expected_moves:
                self.assertIn(move, piece_moves, f"{piece} is missing move {move}")
            
            for piece_move in piece_moves:
                self.assertIn(piece_move, expected_moves, f"{piece} listing extra move {piece_move}")

        @unittest.skip("Base Test to be implement by subclasses")
        def test_starting_position(self):
            pass






        


