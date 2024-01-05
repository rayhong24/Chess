import Pieces

from enums import Colour
from utils import *

class Move:
    def __init__(self, player_to_move, piece_str, start_coords, capture, end_coords) -> None:
        self.player_to_move = player_to_move

        self.piece_str = piece_str
        self.start_coords = start_coords
        self.capture = capture
        self.end_coords = end_coords



