from Tests.TestPieces.pieceTestCases import PiecesTestCases
from enums import *
from coords import Coords

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

    def test_middle(self):
        self.game.setup_fenstr("8/8/8/8/4B3/8/8/8 w - - 0 1")
        bishop = self.game.board.get_square(Coords(4, File['e']))
        expected_moves = [
            # Up and left
            self.move_factory.init_move_from_str("Be4-c6", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Be4-d5", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Be4-b7", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Be4-a8", Colour.WHITE, self.game),
            # Up and right
            self.move_factory.init_move_from_str("Be4-f5", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Be4-g6", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Be4-h7", Colour.WHITE, self.game),
            # Down and left
            self.move_factory.init_move_from_str("Be4-d3", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Be4-c2", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Be4-b1", Colour.WHITE, self.game),
            # Down and right
            self.move_factory.init_move_from_str("Be4-f3", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Be4-g2", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Be4-h1", Colour.WHITE, self.game),
        ]

        self.check_moves(bishop, expected_moves)