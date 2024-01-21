import unittest

from testPieces import PiecesTestCases
from enums import *

class TestPawn(PiecesTestCases.TestPieces):
    def setUp(self):
        return super().setUp()

    def test_starting_position(self):
        self.game.setup_fenstr()

        for i in range(8):
            pawn = self.game.board.board[6][i]
            file_label = File(i).name
            expected_moves = [
                self.move_factory.init_move(f"{file_label}2-{file_label}3", Colour.WHITE),
                self.move_factory.init_move(f"{file_label}2-{file_label}4", Colour.WHITE)
            ]

            self.check_moves(pawn, expected_moves)
        
    def test_middle(self):
        self.game.setup_fenstr("8/8/8/8/4P3/8/8/8 w - - 0 1")

        pawn = self.game.board.board[4][4]
        expected_moves = [
            self.move_factory.init_move("e4-e5", Colour.WHITE)
        ]

        self.check_moves(pawn, expected_moves)