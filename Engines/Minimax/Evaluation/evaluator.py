from python_chess.GameClasses.board import Board
from python_chess.GameClasses.game import Game
from .pieceSquareTables import PieceSquareTables

from python_chess.Pieces.piece import Piece
from python_chess.Pieces.pawn import Pawn
from python_chess.Pieces.knight import Knight
from python_chess.Pieces.bishop import Bishop
from python_chess.Pieces.rook import Rook
from python_chess.Pieces.queen import Queen
from python_chess.Pieces.king import King

from python_chess.enums import Colour

class Evaluator():
    checkmate_eval = 40000
    piece_square_tables = PieceSquareTables()

    def evaluate_game(self, game: Game):
        eval = 0

        for coords in game.board.all_squares_iterator():
            piece = game.board.get_square(coords)

            if piece:
                eval += self.evaluate_piece(piece, coords)


        return eval

    def evaluate_piece(self, piece, coords):
        if piece is None:
            return 0

        evaluation_modifiers = {
            Pawn: self.evaluate_pawn,
            Knight: self.evaluate_knight,
            Bishop: self.evaluate_bishop,
            Rook: self.evaluate_rook,
            Queen: self.evaluate_queen,
            King: self.evaluate_king
        }

        return evaluation_modifiers[type(piece)](piece, coords) + self.piece_square_tables.get_value(piece, coords)

    def evaluate_pawn(self, piece: Piece, coords):
        if piece.colour == Colour.WHITE:
            evaluation = 100
        else:
            evaluation = -100

        return evaluation

    def evaluate_knight(self, piece, coords):
        if piece.colour == Colour.WHITE:
            evaluation = 320 
        else:
            evaluation = -320

        return evaluation

    def evaluate_bishop(self, piece, coords):
        if piece.colour == Colour.WHITE:
            evaluation = 330 
        else:
            evaluation = -330

        return evaluation

    def evaluate_rook(self, piece, coords):
        if piece.colour == Colour.WHITE:
            evaluation = 500 
        else:
            evaluation = -500

        return evaluation
    def evaluate_queen(self, piece, coords):
        if piece.colour == Colour.WHITE:
            evaluation = 900 
        else:
            evaluation = -900

        return evaluation
    def evaluate_king(self, piece, coords):
        if piece.colour == Colour.WHITE:
            evaluation = 20000
        else:
            evaluation = -20000

        return evaluation


            
            


        

