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
        self.to_move = Colour.WHITE if move.player_to_move == Colour.BLACK else Colour.BLACK
        return

    def undo(self):
        self.to_move = Colour.WHITE if self.to_move == Colour.BLACK else Colour.BLACK





