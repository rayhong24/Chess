import unittest

from game import Game
from Pieces.piece import Piece 

class TestPieces(unittest.TestCase):
    def setUp(self):
        self.game = Game()

    def check_moves(self, piece: Piece, expected_moves: list):
        piece_moves = set(piece.get_moves(self.game.board.board))

        for move in expected_moves:
            with self.subTest(move=move):
                self.assertIn(move, piece_moves, f"{Piece} is missing move {move}")
        
        for piece_move in piece_moves:
            with self.subTest(piece_move=piece_move):
                self.assertIn(piece_move, expected_moves, f"{Piece} listing extra move {move}")

    def test_bishop(self):
        self.game.setup_fenstr()
        piece = self.game.board.board[6][4]

    def test_e_pawn(self):
        piece = self.game.board.board[6][4]
        expected_moves = ["e2e3", "e2e4"]

        self.check_moves(piece, expected_moves)

    def test_bishop(self):

    


