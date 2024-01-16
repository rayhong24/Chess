import unittest

from testPieces import PiecesTestCases
from enums import *

class TestPawn(PiecesTestCases.TestPieces):
    def setUp(self):
        return super().setUp()

    def test_e_pawn_simple(self):
        self.game.setup_fenstr()
        piece = self.game.board.board[6][4]
        expected_moves = [
            self.move_factory.init_move("e2-e3", Colour.WHITE),
            self.move_factory.init_move("e2-e4", Colour.WHITE)
        ]

        self.check_moves(piece, expected_moves)