from utils import *
from enums import *
from coords import Coords

from Moves.move import Move
from Moves.castle import Castle
from Moves.promotion import Promotion
from Moves.enPassant import EnPassant

class MoveFactory:
    def init_move_from_str(self, move_str: str, player_to_move: Colour, game) -> Move:
        initializer = self._get_move_initializer(move_str, game)

        return initializer(move_str, player_to_move)

    def init_normal_move(self, player_to_move, piece_str, start_coords, capture, end_coords):
        return Move(
            player_to_move,
            piece_str,
            start_coords, 
            capture,
            end_coords
        )

    def init_enPassant(self, player_to_move, start_coords, end_coords):
        return EnPassant(player_to_move, start_coords, end_coords)

    def init_enPassant_from_str(self, move_str, player_to_move):
        coords = move_str.split("x")
        start = Coords.init_from_str(coords[0])
        end = Coords.init_from_str(coords[1])
        return EnPassant(player_to_move, start, end)


    def _get_move_initializer(self, move_str, game):
        if move_str == "O-O" or move_str == "O-O-O":
            return self._init_castle_from_str
        elif "=" in move_str:
             return self._init_promotion_from_str
        else:
            piece, start_coords, capture, end_coords = self._split_move_str(move_str)
            
            if piece == "P" and capture and game.board.board[end_coords[0]][end_coords[1]] is None:
                return self.init_enPassant_from_str

            return self._init_normal_from_str
    
    def _split_move_str(self, move_str):
        piece = 'P'
        if "-" in move_str:
            start, end = move_str.split('-')

            if len(start) == 3:
                piece = start[0]
                start_i, start_j = to_coords(start[1:])
            else:
                start_i, start_j = to_coords(start[:2])

            capture = False
            end_i, end_j = to_coords(end[:2])
        elif "x" in move_str:
            start, end = move_str.split('x')

            if len(start) == 3:
                piece = start[0]
                start_i, start_j = to_coords(start[1:])
            else:
                start_i, start_j = to_coords(start[:2])

            capture = True
            end_i, end_j = to_coords(end[:2])
        else:
            raise ValueError("Invalid move.")

        return (piece,
            (start_i, start_j),
            capture,
            (end_i, end_j)
        )
    
    def _init_normal_from_str(self, move_str: str, player_to_move: Colour) -> Move:
        piece, start_coords, capture, end_coords = self._split_move_str(move_str)

        return Move(
            player_to_move,
            piece,
            start_coords,
            capture,
            end_coords
        )


    def _init_castle_from_str(self, move_str: str, player_to_move: Colour) -> Move:
        return Castle(
            move_str,
            player_to_move,
        )

    def _init_promotion_from_str(self, move_str: str, player_to_move: Colour) -> Move:
        move, promotion_piece = move_str.split('=')
        _, start_coords, capture, end_coords = self._split_move_str(move)

        return Promotion(
            player_to_move,
            start_coords,
            capture,
            end_coords,
            promotion_piece
        )

