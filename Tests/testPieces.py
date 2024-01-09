import unittest

from enums import *
from game import Game
from Moves.moveFactory import MoveFactory
from Pieces.piece import Piece 

class TestPieces(unittest.TestCase):
    def setUp(self):
        self.game = Game()
        self.move_factory = MoveFactory()

    def check_moves(self, piece: Piece, expected_moves: list):
        piece_moves = set(piece.get_moves(self.game.board.board))

        for move in expected_moves:
            self.assertIn(move, piece_moves, f"{piece} is missing move {move}")
        
        for piece_move in piece_moves:
            self.assertIn(piece_move, expected_moves, f"{piece} listing extra move {piece_move}")

    def test_bishop(self):
        self.game.setup_fenstr()
        piece = self.game.board.board[7][2]
        expected_moves = []

        self.check_moves(piece, expected_moves)


    def test_e_pawn(self):
        self.game.setup_fenstr()
        piece = self.game.board.board[6][4]
        expected_moves = [
            self.move_factory.init_move("e2-e3", Colour.WHITE),
            self.move_factory.init_move("e2-e4", Colour.WHITE)
        ]

        self.check_moves(piece, expected_moves)


    


