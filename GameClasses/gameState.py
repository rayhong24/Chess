from typing import Optional

from enums import Colour
from coords import Coords

from Moves.move import Move

from GameClasses.board import Board

class GameState:
    to_move: Colour
    castling_rights: str
    en_passant_target: Optional[Coords]

    def __init__(self):
        self.to_move = Colour.WHITE
        self.castling_rights = "KQkq"
        self.en_passant_target = None

        self.castling_rights_history = []

    def update(self, move: Move, board: Board):
        self.to_move = Colour.WHITE if move.player_to_move == Colour.BLACK else Colour.BLACK
        self.castling_rights_history.append(self.castling_rights)

        rook_coords_to_castle = {"a1": "Q", "h1": "K", "a8": "q", "h8": "k"}

        for rook_coord, castle_str in rook_coords_to_castle.keys():
            if str(move.start_coords) == rook_coord or str(move.end_coords) == rook_coord:
                self.castling_rights.replace(castle_str, "")

        if str(move.start_coords) == "e1":
            self.castling_rights.replace("K", "")
            self.castling_rights.replace("Q", "")

        if str(move.start_coords) == "e8":
            self.castling_rights.replace("k", "")
            self.castling_rights.replace("q", "")

        return

    def undo(self):
        self.to_move = Colour.WHITE if self.to_move == Colour.BLACK else Colour.BLACK
        self.castling_rights = self.castling_rights_history.pop()





