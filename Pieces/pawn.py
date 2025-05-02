import itertools

from Pieces.piece import Piece
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
    
    def get_representation(self) -> str:
        return 'p' if self.colour == Colour.BLACK else 'P'

    def get_moves(self, game):
        def append_promotion_moves(end_coords, capture):
            for piece in "QRBN":
                move = self.move_factory.init_promotion(
                    self.colour,
                    self.coords,
                    capture,
                    end_coords,
                    piece
                )
                valid_moves.append(move)

        # list of tuples of new coordinates the piece can go
        valid_moves = []

        direction = 1 if self.colour == Colour.WHITE else -1

        # checking forward moves
        moves_forward = 1 if self.has_moved else 2
        for new_coords in itertools.islice(self.coords.get_line(direction, 0), moves_forward):
            square = game.board.get_square(new_coords)
            if square == None:
                # For promotions
                if new_coords.rank == 1 or new_coords.rank == 8:
                    append_promotion_moves(new_coords, False)
                else:
                    move = self.move_factory.init_normal_move(
                        self.colour,
                        self.get_representation().upper(),
                        self.coords,
                        False,
                        new_coords
                    )
                    valid_moves.append(move)
            else:
                break

        
        # check captures
        left_capture_coords = self.coords.get_neighbour(direction, -1)
        right_capture_coords = self.coords.get_neighbour(direction, 1)
        if left_capture_coords:
            if left_capture_coords == game.enpassant_coords:
                enpassant_move = self.move_factory.init_enPassant(
                    game.player_turn, 
                    self.coords, 
                    left_capture_coords
                )
                valid_moves.append(enpassant_move)
            square_to_check_left = game.board.get_square(left_capture_coords)
            if square_to_check_left is not None and square_to_check_left.colour != self.colour:
                # For promotions
                if left_capture_coords.rank == 1 or left_capture_coords.rank == 8:
                    append_promotion_moves(left_capture_coords, True)
                else:
                    move = self.move_factory.init_normal_move(
                        self.colour,
                        self.get_representation().upper(),
                        self.coords,
                        True,
                        left_capture_coords
                    )
                    valid_moves.append(move)
        
        if right_capture_coords:
            if right_capture_coords == game.enpassant_coords:
                enpassant_move = self.move_factory.init_enPassant(
                    game.player_turn,
                    self.coords,
                    right_capture_coords
                )
                valid_moves.append(enpassant_move)
            square_to_check_right = game.board.get_square(right_capture_coords)
            if  square_to_check_right is not None and square_to_check_right.colour != self.colour:
                # For promotions
                if right_capture_coords.coords.rank == 1 or right_capture_coords.rank == 8:
                    append_promotion_moves(right_capture_coords, True)
                else:
                    move = self.move_factory.init_normal_move(
                        self.colour,
                        self.get_representation().upper(),
                        self.coords,
                        True,
                        right_capture_coords
                    )
                    valid_moves.append(move)

        
        return valid_moves