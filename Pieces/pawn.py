from Pieces.piece import Piece
import Pieces.bishop
import Pieces.queen
import Pieces.rook
from enums import Colour
from coords import Coords

from Moves.move import Move

class Pawn(Piece):
    def __init__(self, colour: Colour, coords: Coords) -> None:
        super().__init__(colour, coords)
        self.has_moved = not (
            (colour == Colour.WHITE and coords.rank == 2) or\
            (colour == Colour.BLACK and coords.rank == 7)
        )
    
    #TODO Change to __str__ or __repr__
    def get_representation(self) -> str:
        return 'p' if self.colour == Colour.BLACK else 'P'

    def get_candidate_moves(self, curr_coords: Coords):
        direction = 1 if self.colour == Colour.WHITE else -1

        # checking forward moves
        moves_forward = 1 if self.has_moved else 2
        for dist in range(moves_forward):
            new_coords = Coords.get_neighbour(direction*dist, 0)
            if new_coords:
                move = self.move_factory.init_normal_move(
                    self.colour,
                    type(self),
                    curr_coords,
                    False,
                    new_coords
                )
                yield(move)
            else:
                break

        
        # check captures
        left_capture_coords = self.coords.get_neighbour(direction, -1)
        right_capture_coords = self.coords.get_neighbour(direction, 1)
        if left_capture_coords:
            move = self.move_factory.init_normal_move(
                self.colour,
                type(self),
                curr_coords,
                True,
                left_capture_coords
            )
            yield(move)
        if right_capture_coords:
            move = self.move_factory.init_normal_move(
                self.colour,
                type(self),
                curr_coords,
                True,
                right_capture_coords
            )
            yield(move)