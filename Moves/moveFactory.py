from enums import *
from coords import Coords

from GameClasses.game import Game

from Pieces.piece import Piece
from Pieces.king import King
from Pieces.pawn import Pawn

from Moves.move import Move
from Moves.castle import Castle
from Moves.promotion import Promotion

class MoveFactory:
    def init_move_from_str(self, move_str: str, game: Game) -> Move:
        initializer = self._get_move_initializer(move_str, game)

        return initializer(move_str, game.state.to_move)


    def _init_normal(self, move_str: str, to_move) -> Move:
        return Move(
            to_move,
            Coords.init_from_str(move_str[:2]),
            False,
            Coords.init_from_str(move_str[2:4]),
        )

    def _init_castle(self, move_str: str, player_to_move: Colour):
        start_coords, capture, end_coords = self._split_move_information(move_str)

        if move_str == "e1g1":
            rook_start = Coords.init_from_str("h1")
            rook_end = Coords.init_from_str("f1")
        elif move_str == "e1c1":
            rook_start = Coords.init_from_str("a1")
            rook_end = Coords.init_from_str("d1")
        elif move_str == "e8g8":
            rook_start = Coords.init_from_str("h8")
            rook_end = Coords.init_from_str("f8")
        elif move_str == "e8c8":
            rook_start = Coords.init_from_str("a8")
            rook_end = Coords.init_from_str("d8")

        return Castle(
            player_to_move, 
            start_coords,
            False,
            end_coords,
            rook_start,
            rook_end
        )

    def _init_promotion(self, move_str: str, player_to_move: Colour):
        return Promotion(
            player_to_move,
            Coords.init_from_str(move_str[:2]),
            False,
            Coords.init_from_str(move_str[2:4]),
            move_str[4]
        )

    def _get_move_initializer(self, move_str: str, game: Game):
        castle_strs = ["e1g1", "e1c1", "e8g8", "e8c8"]

        if len(move_str) == 5:
            return self._init_promotion

        start_coords = Coords.init_from_str(move_str[:2])

        piece = game.board.get_square(start_coords)

        if type(piece) == King and move_str in castle_strs:
            return self._init_castle
        
        else:
            return self._init_normal

        

    def _split_move_information(self, move_str: str):
        return (Coords.init_from_str(move_str[:2]), 
                False, 
                Coords.init_from_str(move_str[2:4]))






    
