from GameClasses.board import Board
from Moves.move import Move

from Pieces.piece import Piece

from enums import Colour
from coords import Coords

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
                    moves.append(Move(piece.colour, coords, False, end_coords))

                else:
                    blocking_piece = board.get_square(end_coords)
                    if candidate.capture_allowed and blocking_piece.colour != piece.colour:
                        moves.append(Move(piece.colour, coords, True, end_coords))
                    break

        return moves
