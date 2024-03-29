from Tests.TestPieces.pieceTestCases import PiecesTestCases
from enums import *

class testQueen(PiecesTestCases.TestPieces):
    def setUp(self):
        return super().setUp()

    def test_starting_position(self):
        self.game.setup_fenstr()

        queen = self.game.board.board[7][3]
        expected_moves = []
        self.check_moves(queen, expected_moves)
    
    def test_middle(self):
        self.game.setup_fenstr("8/8/8/8/4Q3/8/8/8 w - - 0 1")
        queen = self.game.board.board[4][4]
        expected_moves = [
            # Up and left
            self.move_factory.init_move_from_str("Qe4-d5", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Qe4-c6", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Qe4-b7", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Qe4-a8", Colour.WHITE, self.game),
            # Up and right
            self.move_factory.init_move_from_str("Qe4-f5", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Qe4-g6", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Qe4-h7", Colour.WHITE, self.game),
            # Down and left
            self.move_factory.init_move_from_str("Qe4-d3", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Qe4-c2", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Qe4-b1", Colour.WHITE, self.game),
            # Down and right
            self.move_factory.init_move_from_str("Qe4-f3", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Qe4-g2", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Qe4-h1", Colour.WHITE, self.game),
            # Up
            self.move_factory.init_move_from_str("Qe4-e5", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Qe4-e6", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Qe4-e7", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Qe4-e8", Colour.WHITE, self.game),
            # Down 
            self.move_factory.init_move_from_str("Qe4-e3", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Qe4-e2", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Qe4-e1", Colour.WHITE, self.game),
            # Left
            self.move_factory.init_move_from_str("Qe4-d4", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Qe4-c4", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Qe4-b4", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Qe4-a4", Colour.WHITE, self.game),
            # Right
            self.move_factory.init_move_from_str("Qe4-f4", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Qe4-g4", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("Qe4-h4", Colour.WHITE, self.game),
        ]
        self.check_moves(queen, expected_moves)