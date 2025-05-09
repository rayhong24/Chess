from typing import Optional

from enums import Colour
from coords import Coords

from Moves.move import Move

from GameClasses.board import Board

class GameState:
    to_move: Colour
    castling_rights: str
    en_passant_target: Optional[Coords]

    def update(self, move: Move, board: Board):
        to_moves = Colour.WHITE if move.player_to_move == Colour.BLACK else Colour.BLACK
        return




