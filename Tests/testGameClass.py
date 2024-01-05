import unittest

from game import Game
from enums import *

class TestGameClass(unittest.TestCase):
    def setUp(self):
        self.game = Game()

    def check_board_equal(self, b):
        for i in range(8):
            for j in range(8):
                with self.subTest(i=i, j=j):
                    piece = self.game.board.board[i][j]
                    self.assertEqual(self.game.board.get_square_representation(piece), b[i][j],\
                                    f"Incorrect board set-up on ({i},{j})")

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

        self.game.make_move("e2-e4")
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

        self.game.make_move("e7-e5")
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

        self.game.make_move("Ng1-f3")
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


        self.game.make_move("Nb8-c6")
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

        self.game.make_move("Bf1-c4")
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

        self.game.make_move("Bf8-c5")
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

        self.game.make_move("O-O")
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

        self.game.make_move("Ng8-f6")
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

        self.game.make_move("Nb1-c3")
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

        self.game.make_move("O-O")
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