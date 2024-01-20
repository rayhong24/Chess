import unittest

from testPieces import PiecesTestCases
from enums import *

class testKnight(PiecesTestCases.TestPieces):
    def setUp(self):
        return super().setUp()

    def test_starting_position(self):
        self.game.setup_fenstr()

        knight1 = self.game.board.board[7][1]
        expected_moves = [
            self.move_factory.init_move("Nb1-a3", Colour.WHITE),
            self.move_factory.init_move("Nb1-c3", Colour.WHITE),
        ]
        self.check_moves(knight1, expected_moves)

        knight2 = self.game.board.board[7][6]
        expected_moves = [
            self.move_factory.init_move("Ng1-f3", Colour.WHITE),
            self.move_factory.init_move("Ng1-h3", Colour.WHITE),
        ]
        self.check_moves(knight2, expected_moves)


    def test_middle(self):
        self.game.setup_fenstr("8/8/8/8/4N3/8/8/8 w - 0 1")
        piece = self.game.board.board[4][4]
        expected_moves = [
            self.move_factory.init_move("Ne4-d6", Colour.WHITE),
            self.move_factory.init_move("Ne4-f6", Colour.WHITE),
            self.move_factory.init_move("Ne4-c5", Colour.WHITE),
            self.move_factory.init_move("Ne4-g5", Colour.WHITE),
            self.move_factory.init_move("Ne4-c3", Colour.WHITE),
            self.move_factory.init_move("Ne4-g3", Colour.WHITE),
            self.move_factory.init_move("Ne4-d2", Colour.WHITE),
            self.move_factory.init_move("Ne4-f2", Colour.WHITE),
        ]

        self.check_moves(piece, expected_moves)

    