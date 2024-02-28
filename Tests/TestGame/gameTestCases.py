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

        def check_board_equal(self, b):
            for coords in Coords.get_all_coords():
                with self.subTest(coords=coords):
                    square = self.game.board.get_square(coords)
                    self.assertEqual(self.game.board.get_square_representation(square), b[8-coords.rank][coords.file.value])

        def check_move(self, expected_turn):
            self.assertEqual(self.game.player_turn, expected_turn, 'incorrect player turn')

        def check_castling_rights(self, expected_str):
            self.assertEqual(self.game.get_castle_str(), expected_str)
        
        def check_valid_moves(self, game, expected_moves: list):
            game = set(game.get_valid_moves())

            for move in expected_moves:
                self.assertIn(move, game, f"{game} is missing move {move}")
            
            for piece_move in game:
                self.assertIn(piece_move, expected_moves, f"{game} listing extra move {piece_move}")



        
        # def test_widget_resize(self):
        #     self.widget.resize(100,150)
        #     self.assertEqual(self.widget.size(), (100,150),
        #                      'wrong size after resize')