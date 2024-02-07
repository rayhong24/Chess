from Tests.TestGame.gameTestCases import GameTestCases
from enums import *

class TestCastling(GameTestCases.TestGameClass):
    def test_kingside_castle(self):
        # Might need to fix the castling rights fenstring check
        self.game.setup_fenstr("4k2r/8/8/8/8/8/8/4K2R w KQkq - 0 1")

        b = [
                ['','','','','k','','','r'],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','K','','','R']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        # Might need to fix the castling rights fenstring check
        self.check_castling_rights("Kk")

        move = self.move_factory.init_move("O-O", self.game.player_turn)
        move.make_move(self.game)

        b = [
                ['','','','','k','','','r'],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','R','K','']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("k")

    def test_castle_through_check1(self):
        self.game.setup_fenstr("4k2r/8/8/8/8/8/8/4R3 b k - 0 1")

        b = [
                ['','','','','k','','','r'],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','R','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("k")

        move = self.move_factory.init_move("O-O", Colour.BLACK)
        self.assertFalse(move.make_move(self.game))

        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("k")

    def test_castle_through_check2(self):
        self.game.setup_fenstr("4k2r/8/8/8/8/8/8/5R2 b k - 0 1")

        b = [
                ['','','','','k','','','r'],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','R','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("k")

        move = self.move_factory.init_move("O-O", Colour.BLACK)
        self.assertFalse(move.make_move(self.game))

        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("k")

    def test_castle_through_check3(self):
        self.game.setup_fenstr("4k2r/8/8/8/8/8/8/6R1 b k - 0 1")

        b = [
                ['','','','','k','','','r'],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','R','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("k")

        move = self.move_factory.init_move("O-O", Colour.BLACK)
        self.assertFalse(move.make_move(self.game))

        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("k")