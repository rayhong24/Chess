import unittest

from game import Game
from Moves.moveFactory import MoveFactory
from enums import *
from coords import Coords

class GameTestCases:
    class TestGameClass(unittest.TestCase):
        def setUp(self):
            self.game = Game()
            self.move_factory = MoveFactory()




        
        # def test_widget_resize(self):
        #     self.widget.resize(100,150)
        #     self.assertEqual(self.widget.size(), (100,150),
        #                      'wrong size after resize')