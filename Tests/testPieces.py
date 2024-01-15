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

    def test_bishop_simple(self):
        self.game.setup_fenstr()
        piece = self.game.board.board[7][2]
        expected_moves = []

        self.check_moves(piece, expected_moves)


    def test_e_pawn_simple(self):
        self.game.setup_fenstr()
        piece = self.game.board.board[6][4]
        expected_moves = [
            self.move_factory.init_move("e2-e3", Colour.WHITE),
            self.move_factory.init_move("e2-e4", Colour.WHITE)
        ]

        self.check_moves(piece, expected_moves)

    def test_king_simple(self):
        self.game.setup_fenstr("8/8/8/4K3/8/8/8/k7 w - 0 1")
        piece = self.game.board.board[3][4]

        expected_moves = [
            self.move_factory.init_move("Ke5-d6", Colour.WHITE),
            self.move_factory.init_move("Ke5-e6", Colour.WHITE),
            self.move_factory.init_move("Ke5-f6", Colour.WHITE),
            self.move_factory.init_move("Ke5-d5", Colour.WHITE),
            self.move_factory.init_move("Ke5-f5", Colour.WHITE),
            self.move_factory.init_move("Ke5-d4", Colour.WHITE),
            self.move_factory.init_move("Ke5-e4", Colour.WHITE),
            self.move_factory.init_move("Ke5-f4", Colour.WHITE),
        ]

        self.check_moves(piece, expected_moves)

    def test_king_castle_white(self):
        self.game.setup_fenstr("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq 0 1")

        piece = self.game.board.board[7][4]

        expected_moves = [
            self.move_factory.init_move("O-O", Colour.WHITE),
            self.move_factory.init_move("O-O-O", Colour.WHITE),
            self.move_factory.init_move("Ke1-d1", Colour.WHITE),
            self.move_factory.init_move("Ke1-f1", Colour.WHITE),
        ]


        self.check_moves(piece, expected_moves)

    def test_king_castle_black(self):
        self.game.setup_fenstr("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R b KQkq 0 1")

        piece = self.game.board.board[0][4]

        expected_moves = [
            self.move_factory.init_move("O-O", Colour.BLACK),
            self.move_factory.init_move("O-O-O", Colour.BLACK),
            self.move_factory.init_move("Ke8-d8", Colour.BLACK),
            self.move_factory.init_move("Ke8-f8", Colour.BLACK),
        ]


        self.check_moves(piece, expected_moves)



    


