from Tests.TestPieces.pieceTestCases import PiecesTestCases
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
                self.move_factory.init_move_from_str(f"{file_label}2-{file_label}3", Colour.WHITE, self.game),
                self.move_factory.init_move_from_str(f"{file_label}2-{file_label}4", Colour.WHITE, self.game)
            ]

            self.check_moves(pawn, expected_moves)
        
    def test_middle(self):
        self.game.setup_fenstr("8/8/8/8/4P3/8/8/8 w - - 0 1")

        pawn = self.game.board.board[4][4]
        expected_moves = [
            self.move_factory.init_move_from_str("e4-e5", Colour.WHITE, self.game)
        ]

        self.check_moves(pawn, expected_moves)

    def test_en_passant_white(self):
        # Test left
        self.game.setup_fenstr("8/8/8/3pP3/8/8/8/8 w - d6 0 1")

        pawn = self.game.board.board[3][4]
        expected_moves = [
            self.move_factory.init_move_from_str("e5-e6", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("e5xd6", Colour.WHITE, self.game),
        ]

        self.check_moves(pawn, expected_moves)

        # Test right
        self.game.setup_fenstr("8/8/8/4Pp2/8/8/8/8 w - f6 0 1")

        pawn = self.game.board.board[3][4]
        expected_moves = [
            self.move_factory.init_move_from_str("e5-e6", Colour.WHITE, self.game),
            self.move_factory.init_move_from_str("e5xf6", Colour.WHITE, self.game),
        ]

        self.check_moves(pawn, expected_moves)


    def test_en_passant_black(self):
        # Test left
        self.game.setup_fenstr("8/8/8/8/3Pp3/8/8/8 b - d3 0 1")

        pawn = self.game.board.board[4][4]
        expected_moves = [
            self.move_factory.init_move_from_str("e4-e3", Colour.BLACK, self.game),
            self.move_factory.init_move_from_str("e4xd3", Colour.BLACK, self.game),
        ]

        self.check_moves(pawn, expected_moves)

        # Test right
        self.game.setup_fenstr("8/8/8/8/4pP2/8/8/8 b - f3 0 1")

        pawn = self.game.board.board[4][4]
        expected_moves = [
            self.move_factory.init_move_from_str("e4-e3", Colour.BLACK, self.game),
            self.move_factory.init_move_from_str("e4xf3", Colour.BLACK, self.game),
        ]

        self.check_moves(pawn, expected_moves)

    def test_blocked_white(self):
        self.game.setup_fenstr("8/8/8/8/8/N7/P7/8 w - - 0 1")
        pawn = self.game.board.board[6][0]

        expected_moves = []

        self.check_moves(pawn, expected_moves)

