from Tests.TestGame.gameTestCases import GameTestCases
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
        self.assertFalse(self.game.is_king_in_check(Colour.WHITE))

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
        self.assertTrue(self.game.is_king_in_check(Colour.WHITE))

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
        self.assertFalse(self.game.is_king_in_check(Colour.BLACK))

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
        self.assertTrue(self.game.is_king_in_check(Colour.BLACK))
    
    def test_check_with_rook_white(self):
        self.game.setup_fenstr("r7/8/8/8/4K3/8/8/8 b - - 0 1")

        b = [
                ['r','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','K','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("-")
        self.assertFalse(self.game.is_king_in_check(Colour.WHITE))

        move = self.move_factory.init_move("Ra8-a4", self.game.player_turn)
        move.make_move(self.game)

        b = [
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['r','','','','K','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("-")
        self.assertTrue(self.game.is_king_in_check(Colour.WHITE))

    def test_check_with_rook_black(self):
        self.game.setup_fenstr("8/8/8/8/4k3/8/8/R7 w - - 0 1")

        b = [
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','k','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['R','','','','','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("-")
        self.assertFalse(self.game.is_king_in_check(Colour.BLACK))

        move = self.move_factory.init_move("Ra1-a4", self.game.player_turn)
        move.make_move(self.game)

        b = [
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['R','','','','k','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("-")
        self.assertTrue(self.game.is_king_in_check(Colour.BLACK))

    def test_check_with_knight_white(self):
        self.game.setup_fenstr("1n6/8/8/8/3K4/8/8/8 b - - 0 1")

        b = [
                ['','n','','','','','',''],\
                ['','','','','','','',''],\
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
        self.assertFalse(self.game.is_king_in_check(Colour.WHITE))

        move = self.move_factory.init_move("Nb8-c6", self.game.player_turn)
        move.make_move(self.game)

        b = [
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','n','','','','',''],\
                ['','','','','','','',''],\
                ['','','','K','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("-")
        self.assertTrue(self.game.is_king_in_check(Colour.WHITE))

    def test_check_with_knight_black(self):
        self.game.setup_fenstr("8/8/8/8/4k3/8/8/1N6 w - - 0 1")

        b = [
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','k','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','N','','','','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("-")
        self.assertFalse(self.game.is_king_in_check(Colour.BLACK))

        move = self.move_factory.init_move("Nb1-c3", self.game.player_turn)
        move.make_move(self.game)

        b = [
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','k','','',''],\
                ['','','N','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("-")
        self.assertTrue(self.game.is_king_in_check(Colour.BLACK))

    def test_check_with_bishop_white(self):
        self.game.setup_fenstr("2b5/8/8/8/4K3/8/8/8 b - - 0 1")

        b = [
                ['','','b','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','K','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("-")
        self.assertFalse(self.game.is_king_in_check(Colour.WHITE))

        move = self.move_factory.init_move("Bc8-b7", self.game.player_turn)
        move.make_move(self.game)

        b = [
                ['','','','','','','',''],\
                ['','b','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','K','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("-")
        self.assertTrue(self.game.is_king_in_check(Colour.WHITE))

    def test_check_with_bishop_black(self):
        self.game.setup_fenstr("8/8/8/8/3k4/8/8/2B5 w - - 0 1")

        b = [
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','k','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','B','','','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("-")
        self.assertFalse(self.game.is_king_in_check(Colour.BLACK))

        move = self.move_factory.init_move("Bc1-b2", self.game.player_turn)
        move.make_move(self.game)

        b = [
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','k','','','',''],\
                ['','','','','','','',''],\
                ['','B','','','','','',''],\
                ['','','','','','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("-")
        self.assertTrue(self.game.is_king_in_check(Colour.BLACK))

    def test_check_with_queen_white(self):
        self.game.setup_fenstr("2q5/8/8/8/4K3/8/8/8 b - - 0 1")

        b = [
                ['','','q','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','K','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("-")
        self.assertFalse(self.game.is_king_in_check(Colour.WHITE))

        move = self.move_factory.init_move("Qc8-e6", self.game.player_turn)
        move.make_move(self.game)

        b = [
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','q','','',''],\
                ['','','','','','','',''],\
                ['','','','','K','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("-")
        self.assertTrue(self.game.is_king_in_check(Colour.WHITE))

    def test_check_with_bishop_black(self):
        self.game.setup_fenstr("8/8/8/8/3k4/8/8/2Q5 w - - 0 1")

        b = [
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','k','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','Q','','','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("-")
        self.assertFalse(self.game.is_king_in_check(Colour.BLACK))

        move = self.move_factory.init_move("Qc1-d2", self.game.player_turn)
        move.make_move(self.game)

        b = [
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','k','','','',''],\
                ['','','','','','','',''],\
                ['','','','Q','','','',''],\
                ['','','','','','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("-")
        self.assertTrue(self.game.is_king_in_check(Colour.BLACK))
    
    def test_pin_with_rook(self):
        self.game.setup_fenstr("4k3/8/4q3/8/8/8/8/3KR3 b - - 0 1")

        b = [
                ['','','','','k','','',''],\
                ['','','','','','','',''],\
                ['','','','','q','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','K','R','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("-")
        self.assertFalse(self.game.is_king_in_check(Colour.BLACK))

        move = self.move_factory.init_move("Qe6-a6", self.game.player_turn)
        self.assertFalse(move.make_move(self.game))

        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("-")
        self.assertFalse(self.game.is_king_in_check(Colour.BLACK))


    def test_enpassant_while_in_check(self):
        self.game.setup_fenstr("8/8/8/3pP3/8/8/8/rK6 w - d3 0 1")

        b = [
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','p','P','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['r','K','','','','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("-")

        move = self.move_factory.init_enPassant_from_str("e4xd3", Colour.WHITE)
        self.assertFalse(move.make_move(self.game))

        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("-")