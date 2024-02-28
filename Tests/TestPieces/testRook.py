from Tests.TestPieces.pieceTestCases import PiecesTestCases
from enums import *

class testRook(PiecesTestCases.TestPieces):
    def setUp(self):
        return super().setUp()

    def test_starting_position(self):
        self.game.setup_fenstr()

        rook1 = self.game.board.board[7][0]
        expected_moves = []
        self.check_moves(rook1, expected_moves)

        rook2 = self.game.board.board[7][7]
        self.check_moves(rook2, expected_moves)

    def test_middle(self):
        self.game.setup_fenstr("8/8/8/8/4R3/8/8/8 w - - 0 1")

        rook = self.game.board.board[4][4]
        expected_moves = [
            # Up
            self.move_factory.init_move_from_str("Re4-e5", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Re4-e6", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Re4-e7", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Re4-e8", Colour.WHITE, self.game),
            # Down 
            self.move_factory.init_move_from_str("Re4-e3", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Re4-e2", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Re4-e1", Colour.WHITE, self.game),
            # Left
            self.move_factory.init_move_from_str("Re4-d4", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Re4-c4", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Re4-b4", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Re4-a4", Colour.WHITE, self.game),
            # Right
            self.move_factory.init_move_from_str("Re4-f4", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Re4-g4", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Re4-h4", Colour.WHITE, self.game),
        ]

        self.check_moves(rook, expected_moves)