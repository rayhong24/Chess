from Tests.TestPieces.pieceTestCases import PiecesTestCases
from enums import *

class testKing(PiecesTestCases.TestPieces):
    def setUp(self):
        return super().setUp()

    def test_starting_position(self):
        self.game.setup_fenstr()

        king = self.game.board.board[7][4]
        expected_moves = []

        self.check_moves(king, expected_moves)

    def test_middle(self):
        self.game.setup_fenstr("8/8/8/8/4K3/8/8/k7 w - - 0 1")
        piece = self.game.board.board[4][4]

        expected_moves = [
            self.move_factory.init_move("Ke4-d5", Colour.WHITE, self.game),
            self.move_factory.init_move("Ke4-e5", Colour.WHITE, self.game),
            self.move_factory.init_move("Ke4-f5", Colour.WHITE, self.game),
            self.move_factory.init_move("Ke4-d4", Colour.WHITE, self.game),
            self.move_factory.init_move("Ke4-f4", Colour.WHITE, self.game),
            self.move_factory.init_move("Ke4-d3", Colour.WHITE, self.game),
            self.move_factory.init_move("Ke4-e3", Colour.WHITE, self.game),
            self.move_factory.init_move("Ke4-f3", Colour.WHITE, self.game),
        ]

        self.check_moves(piece, expected_moves)

    def test_king_castle_white(self):
        self.game.setup_fenstr("8/8/8/8/8/8/3PPP2/R3K2R w KQkq - 0 1")

        # White king
        piece = self.game.board.board[7][4]

        expected_moves = [
            self.move_factory.init_move("O-O", Colour.WHITE, self.game),
            self.move_factory.init_move("O-O-O", Colour.WHITE, self.game),
            self.move_factory.init_move("Ke1-d1", Colour.WHITE, self.game),
            self.move_factory.init_move("Ke1-f1", Colour.WHITE, self.game),
        ]


        self.check_moves(piece, expected_moves)

    def test_king_castle_black(self):
        self.game.setup_fenstr("r3k2r/3ppp2/8/8/8/8/8/8 b KQkq - 0 1")

        # Black king
        piece = self.game.board.board[0][4]

        expected_moves = [
            self.move_factory.init_move("O-O", Colour.BLACK, self.game),
            self.move_factory.init_move("O-O-O", Colour.BLACK, self.game),
            self.move_factory.init_move("Ke8-d8", Colour.BLACK, self.game),
            self.move_factory.init_move("Ke8-f8", Colour.BLACK, self.game),
        ]

        self.check_moves(piece, expected_moves)