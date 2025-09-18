from python_chess.GameClasses.board import Board
from python_chess.Moves.move import Move
from python_chess.Moves.promotion import Promotion
from python_chess.Moves.castle import Castle

from python_chess.Pieces.piece import Piece
from python_chess.Pieces.pawn import Pawn
from python_chess.Pieces.king import King

from python_chess.enums import Colour
from python_chess.coords import Coords

class MoveGenerator():
    def generate_pseudo_legal_moves(self, board: Board, player: Colour):
        moves = []

        for coords in board.all_squares_iterator():
            piece = board.get_square(coords)

            if piece and piece.colour == player:
                moves.extend(self.get_piece_moves(board, piece, coords))

        return moves 

    def get_piece_moves(self, board: Board, piece: Piece, coords: Coords):
        moves = []


        candidate_moves = piece.get_candidate_moves(coords)

        for candidate in candidate_moves:
            for end_coords in candidate.generate_coords(coords):
                if board.get_square(end_coords) == None:
                    if candidate.capture_forced:
                        break
                    if type(piece) == Pawn and (end_coords.rank == 1 or end_coords.rank == 8):
                        moves.extend(self.generate_promotion_moves(piece, coords, False, end_coords))
                    elif type(piece) == King and abs(end_coords.file.value - coords.file.value) == 2:
                        moves.append(Castle(piece.colour, coords, end_coords))
                    else:
                        moves.append(Move(piece.colour, coords, False, end_coords))

                else:
                    blocking_piece = board.get_square(end_coords)
                    if candidate.capture_allowed and blocking_piece.colour != piece.colour:
                        if type(piece) == Pawn and (end_coords.rank == 1 or end_coords.rank == 8):
                            moves.extend(self.generate_promotion_moves(piece, coords, True, end_coords))
                        else:
                            moves.append(Move(piece.colour, coords, True, end_coords))
                    break

        return moves

    def generate_promotion_moves(self, piece, start_coords, capture, end_coords):
        moves = []

        moves.append(Promotion(piece.colour, start_coords, capture, end_coords, "q"))
        moves.append(Promotion(piece.colour, start_coords, capture, end_coords, "r"))
        moves.append(Promotion(piece.colour, start_coords, capture, end_coords, "b"))
        moves.append(Promotion(piece.colour, start_coords, capture, end_coords, "n"))

        return moves
