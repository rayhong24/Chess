
from Tests.gameTestCases import GameTestCases
from enums import *

class TestCheck(GameTestCases.TestGameClass):
    def test_check_with_pawn_white(self):
        self.game.setup_fenstr("8/4p3/8/8/3K4/8/8/8 b - - 0 1")

        b = [
                ['','','','','','','',''],\
                ['','','','','p','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','K','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("-")
        self.assertEqual(False, self.game.is_king_in_check(Colour.WHITE))

        move = self.move_factory.init_move("e7-e5", self.game.player_turn)
        move.make_move(self.game)

        b = [
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','p','','',''],\
                ['','','','K','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("-")
        self.assertEqual(True, self.game.is_king_in_check(Colour.WHITE))

    def test_check_with_pawn_black(self):
        self.game.setup_fenstr("8/8/8/3k4/8/8/4P3/8 w - - 0 1")

        b = [
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','k','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','P','','',''],\
                ['','','','','','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("-")
        self.assertEqual(False, self.game.is_king_in_check(Colour.BLACK))

        move = self.move_factory.init_move("e2-e4", self.game.player_turn)
        move.make_move(self.game)

        b = [
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','k','','','',''],\
                ['','','','','P','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("-")
        self.assertEqual(True, self.game.is_king_in_check(Colour.BLACK))