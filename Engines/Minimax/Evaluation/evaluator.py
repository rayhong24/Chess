from GameClasses.board import Board

from Pieces.piece import Piece
from Pieces.pawn import Pawn
from Pieces.knight import Knight
from Pieces.bishop import Bishop
from Pieces.rook import Rook
from Pieces.queen import Queen
from Pieces.king import King

from enums import Colour

class Evaluator():
    def evaluate_board_state(self, board: Board):
        for coords in board.all_squares_iterator():
            piece = self.get_square(coords)

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

        return evaluation_modifiers[type(piece)](piece, coords)

    def evaluate_pawn(self, piece: Piece, coords):
        pawn_piece_square_table = 
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
            evaluation = 20000

        return evaluation


            
            


        

