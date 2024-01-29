import unittest

from game import Game
from Moves.moveFactory import MoveFactory
from enums import *
from utils import *

class TestGameClass(unittest.TestCase):
    def setUp(self):
        self.game = Game()
        self.move_factory = MoveFactory()

    def check_board_equal(self, b):
        for i in range(8):
            for j in range(8):
                with self.subTest(i=i, j=j):
                    piece = self.game.board.board[i][j]
                    self.assertEqual(self.game.board.get_square_representation(piece), b[i][j],\
                                    f"Incorrect board set-up on ({coords_to_square(i, j)})")

    def check_move(self, expected_turn):
        self.assertEqual(self.game.player_turn, expected_turn, 'incorrect player turn')

    def check_castling_rights(self, expected_str):
        self.assertEqual(self.game.get_castle_str(), expected_str)
    
    def test_starting_position(self):
        b = [
                ['r','n','b','q','k','b','n','r'],\
                ['p','p','p','p','p','p','p','p'],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['P','P','P','P','P','P','P','P'],\
                ['R','N','B','Q','K','B','N','R']
            ]


        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("KQkq")

    def test_fenstring_setup(self):
        fenstr = "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50"
        self.game.setup_fenstr(fenstr)

        b = [
                ['','','','','','','',''],\
                ['','','','','','k','',''],\
                ['','','','p','','','',''],\
                ['','p','','P','p','','','p'],\
                ['p','P','','','P','p','','P'],\
                ['P','','','','','P','','K'],\
                ['','','','','','','',''],\
                ['','','','','','','','']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights('-')

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

    def test_promotion(self):
        self.game.setup_fenstr("8/1P6/8/8/8/8/8/k1K5 w - - 0 1")

        b = [
                ['','','','','','','',''],\
                ['','P','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['k','','K','','','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("-")

        move = self.move_factory.init_move("b7-b8=B", self.game.player_turn)
        move.make_move(self.game)

        b = [
                ['','B','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['k','','K','','','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("-")

    def test_check_with_pawn(self):
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

    
    def test_game(self):
        self.game.setup_fenstr()

        b = [
                ['r','n','b','q','k','b','n','r'],\
                ['p','p','p','p','p','p','p','p'],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['P','P','P','P','P','P','P','P'],\
                ['R','N','B','Q','K','B','N','R']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("KQkq")

        move = self.move_factory.init_move("e2-e4", self.game.player_turn)
        move.make_move(self.game)
        b = [
                ['r','n','b','q','k','b','n','r'],\
                ['p','p','p','p','p','p','p','p'],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','P','','',''],\
                ['','','','','','','',''],\
                ['P','P','P','P','','P','P','P'],\
                ['R','N','B','Q','K','B','N','R']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("KQkq")

        move = self.move_factory.init_move("e7-e5", self.game.player_turn)
        move.make_move(self.game)
        b = [
                ['r','n','b','q','k','b','n','r'],\
                ['p','p','p','p','','p','p','p'],\
                ['','','','','','','',''],\
                ['','','','','p','','',''],\
                ['','','','','P','','',''],\
                ['','','','','','','',''],\
                ['P','P','P','P','','P','P','P'],\
                ['R','N','B','Q','K','B','N','R']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("KQkq")

        move = self.move_factory.init_move("Ng1-f3", self.game.player_turn)
        move.make_move(self.game)
        b = [
                ['r','n','b','q','k','b','n','r'],\
                ['p','p','p','p','','p','p','p'],\
                ['','','','','','','',''],\
                ['','','','','p','','',''],\
                ['','','','','P','','',''],\
                ['','','','','','N','',''],\
                ['P','P','P','P','','P','P','P'],\
                ['R','N','B','Q','K','B','','R']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("KQkq")


        move = self.move_factory.init_move("Nb8-c6", self.game.player_turn)
        move.make_move(self.game)
        b = [
                ['r','','b','q','k','b','n','r'],\
                ['p','p','p','p','','p','p','p'],\
                ['','','n','','','','',''],\
                ['','','','','p','','',''],\
                ['','','','','P','','',''],\
                ['','','','','','N','',''],\
                ['P','P','P','P','','P','P','P'],\
                ['R','N','B','Q','K','B','','R']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("KQkq")

        move = self.move_factory.init_move("Bf1-c4", self.game.player_turn)
        move.make_move(self.game)
        b = [
                ['r','','b','q','k','b','n','r'],\
                ['p','p','p','p','','p','p','p'],\
                ['','','n','','','','',''],\
                ['','','','','p','','',''],\
                ['','','B','','P','','',''],\
                ['','','','','','N','',''],\
                ['P','P','P','P','','P','P','P'],\
                ['R','N','B','Q','K','','','R']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("KQkq")

        move = self.move_factory.init_move("Bf8-c5", self.game.player_turn)
        move.make_move(self.game)
        b = [
                ['r','','b','q','k','','n','r'],\
                ['p','p','p','p','','p','p','p'],\
                ['','','n','','','','',''],\
                ['','','b','','p','','',''],\
                ['','','B','','P','','',''],\
                ['','','','','','N','',''],\
                ['P','P','P','P','','P','P','P'],\
                ['R','N','B','Q','K','','','R']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("KQkq")

        move = self.move_factory.init_move("O-O", self.game.player_turn)
        move.make_move(self.game)
        b = [
                ['r','','b','q','k','','n','r'],\
                ['p','p','p','p','','p','p','p'],\
                ['','','n','','','','',''],\
                ['','','b','','p','','',''],\
                ['','','B','','P','','',''],\
                ['','','','','','N','',''],\
                ['P','P','P','P','','P','P','P'],\
                ['R','N','B','Q','','R','K','']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("kq")

        move = self.move_factory.init_move("Ng8-f6", self.game.player_turn)
        move.make_move(self.game)
        b = [
                ['r','','b','q','k','','','r'],\
                ['p','p','p','p','','p','p','p'],\
                ['','','n','','','n','',''],\
                ['','','b','','p','','',''],\
                ['','','B','','P','','',''],\
                ['','','','','','N','',''],\
                ['P','P','P','P','','P','P','P'],\
                ['R','N','B','Q','','R','K','']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("kq")

        move = self.move_factory.init_move("Nb1-c3", self.game.player_turn)
        move.make_move(self.game)
        b = [
                ['r','','b','q','k','','','r'],\
                ['p','p','p','p','','p','p','p'],\
                ['','','n','','','n','',''],\
                ['','','b','','p','','',''],\
                ['','','B','','P','','',''],\
                ['','','N','','','N','',''],\
                ['P','P','P','P','','P','P','P'],\
                ['R','','B','Q','','R','K','']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("kq")

        move = self.move_factory.init_move("O-O", self.game.player_turn)
        move.make_move(self.game)
        b = [
                ['r','','b','q','','r','k',''],\
                ['p','p','p','p','','p','p','p'],\
                ['','','n','','','n','',''],\
                ['','','b','','p','','',''],\
                ['','','B','','P','','',''],\
                ['','','N','','','N','',''],\
                ['P','P','P','P','','P','P','P'],\
                ['R','','B','Q','','R','K','']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("-")

    # def test_widget_resize(self):
    #     self.widget.resize(100,150)
    #     self.assertEqual(self.widget.size(), (100,150),
    #                      'wrong size after resize')