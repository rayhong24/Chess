import unittest

from testPieces import PiecesTestCases
from enums import *

class TestPawn(PiecesTestCases.TestPieces):
    def setUp(self):
        return super().setUp()

    def test_starting_position(self):
        self.game.setup_fenstr()

        for i in range(8):
            piece = self.game.board.board[6][i]
            file_label = File(i).name
            expected_moves = [
                self.move_factory.init_move(f"{file_label}2-{file_label}3", Colour.WHITE),
                self.move_factory.init_move(f"{file_label}2-{file_label}4", Colour.WHITE)
            ]

            self.check_moves(piece, expected_moves)