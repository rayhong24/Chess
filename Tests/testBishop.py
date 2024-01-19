import unittest

from testPieces import PiecesTestCases
from enums import *

class testBishop(PiecesTestCases.TestPieces):
    def setUp(self):
        return super().setUp()

    def test_starting_position(self):
        self.game.setup_fenstr()

        bishop1 = self.game.board.board[7][2]
        expected_moves = []
        self.check_moves(bishop1, expected_moves)

        bishop2 = self.game.board.board[7][5]
        expected_moves = []
        self.check_moves(bishop2, expected_moves)

    def test_bishop_middle_of_board(self):
        self.game.setup_fenstr("8/8/8/8/4B3/8/8/8 w - 0 1")
        piece = self.game.board.board[4][4]
        expected_moves = [
            # Up and left
            self.move_factory.init_move("Be4-d5", Colour.WHITE),
            self.move_factory.init_move("Be4-c6", Colour.WHITE),
            self.move_factory.init_move("Be4-b7", Colour.WHITE),
            self.move_factory.init_move("Be4-a8", Colour.WHITE),
            # Up and right
            self.move_factory.init_move("Be4-f5", Colour.WHITE),
            self.move_factory.init_move("Be4-g6", Colour.WHITE),
            self.move_factory.init_move("Be4-h7", Colour.WHITE),
            # Down and left
            self.move_factory.init_move("Be4-d3", Colour.WHITE),
            self.move_factory.init_move("Be4-c2", Colour.WHITE),
            self.move_factory.init_move("Be4-b1", Colour.WHITE),
            # Down and right
            self.move_factory.init_move("Be4-f3", Colour.WHITE),
            self.move_factory.init_move("Be4-g2", Colour.WHITE),
            self.move_factory.init_move("Be4-h1", Colour.WHITE),
        ]

        self.check_moves(piece, expected_moves)