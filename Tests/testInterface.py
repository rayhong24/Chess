import unittest

from interface import Interface
from Moves.moveFactory import MoveFactory
from game import Game
from enums import *

class TestInterface(unittest.TestCase):
    def setUp(self):
        self.interface = Interface()

    def test_add_dash(self):
        before = "e1g1"
        after = self.interface.add_dash(before)
        
        self.assertEqual("e1-g1", after)


        before = ""
