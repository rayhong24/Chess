import unittest

from game import Game
from enums import *

class TestGameClass(unittest.TestCase):
    def setUp(self):
        self.game = Game()

    def test_player_turn(self):
        self.assertEqual(self.game.player_turn, Colour.WHITE,
                         'incorrect player turn')
    
    def test_piece_placement(self):
        b = ['r','n','b','q','k','b','n','r'],\
            ['p','p','p','p','p','p','p','p'],\
            ['','','','','','','',''],\
            ['','','','','','','',''],\
            ['','','','','','','',''],\
            ['','','','','','','',''],\
            ['P','P','P','P','P','P','P','P'],\
            ['R','N','B','Q','K','B','N','R']

        for i in range(8):
            for j in range(8):
                with self.subTest(i=i, j=j):
                    piece = self.game.board.board[i][j]
                    self.assertEqual(self.game.board.get_square_representation(piece), b[i][j],\
                                     "Incorrect board set-up")

    def test_move(self):
        self.game.make_move()
        self.assertEqual(self.game.player_turn, Colour.BLACK,
                         'incorrect player turn')
                    
        

    # def test_widget_resize(self):
    #     self.widget.resize(100,150)
    #     self.assertEqual(self.widget.size(), (100,150),
    #                      'wrong size after resize')