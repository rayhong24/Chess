from python_chess.enums import *
from python_chess.coords import Coords

from python_chess.GameClasses.game import Game

from python_chess.Pieces.piece import Piece
from python_chess.Pieces.king import King
from python_chess.Pieces.pawn import Pawn

from python_chess.Moves.move import Move
from python_chess.Moves.castle import Castle
from python_chess.Moves.promotion import Promotion

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
        return Castle(
            player_to_move, 
            start_coords,
            end_coords
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






    
