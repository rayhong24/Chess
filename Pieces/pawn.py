from Pieces.piece import Piece
from enums import Colour

from Moves.move import Move

class Pawn(Piece):
    def __init__(self, colour: Colour, row: int, column: int) -> None:
        super().__init__(colour, row, column)
        self.has_moved = not (
            (colour == Colour.WHITE and row == 6) or\
            (colour == Colour.BLACK and row == 1)
        )
    
    def get_representation(self) -> str:
        return 'p' if self.colour == Colour.BLACK else 'P'

    def get_moves(self, game) -> [str]:
        def append_promotion_move(move_str):
            for piece in "QRBN":
                new_s = f"{move_str}={piece}"
                move = self.move_factory.init_move(new_s, self.colour)
                valid_moves.append(move)

        # list of tuples of new coordinates the piece can go
        valid_moves = []

        direction = -1 if self.colour == Colour.WHITE else 1

        # checking forward moves
        moves_forward = 1 if self.has_moved else 2
        for di in range(1, moves_forward+1):
            i, j = self.row+(di*direction), self.column
            if self.is_inbounds(i, j) and game.board.board[i][j] == None:
                move_str = self.get_move_str(self.row, self.column, i, j, False)
                # For promotions
                if i == 0 or i == 7:
                    append_promotion_move(move_str)
                else:
                    move = Move(self.colour,
                        self.get_representation().upper(),
                        (self.row, self.column),
                        False,
                        (i, j))
                    valid_moves.append(move)
        
        # check captures
        i = self.row+direction
        j_left = self.column-1
        j_right = self.column+1
        if self.is_inbounds(i, j_left):
            square_to_check1 = game.board.board[i][j_left]
            if square_to_check1 is not None and square_to_check1.colour != self.colour:
                move_str = self.get_move_str(self.row, self.column, i, j_left, True)
                # For promotions
                if i == 0 or i == 7:
                    append_promotion_move(move_str)
                else:
                    move = self.move_factory.init_move(move_str)
                    valid_moves.append(move)
        
        if self.is_inbounds(i, j_right):
            square_to_check2 = game.board.board[i][j_right]
            if  square_to_check2 is not None and square_to_check2.colour != self.colour:
                move_str = self.get_move_str(self.row, self.column, i, j_right, True)
                # For promotions
                if i == 0 or i == 7:
                    append_promotion_move(move_str)
                else:
                    move = self.move_factory.init_move(move_str)
                    valid_moves.append(move)

        
        return valid_moves