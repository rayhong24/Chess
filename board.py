from Pieces.piece import Piece
from Pieces.king import King

from strings import *
from enums import *
from coords import Coords

from Moves.move import Move

from Pieces.pieceFactory import PieceFactory

class Board:
    def __init__(self):
        self._piece_factory = PieceFactory()
        self._board = [[None]*8 for _ in range(8)]
        self._previous_moves = []

    def get_square(self, coords: Coords) -> Piece:
        return self._board[8-coords.rank][coords.file.value]

    def set_square(self, value, coords: Coords):
        self._board[8-coords.rank][coords.file.value] = value

    # Input: string from a fenstring (i.e. rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR)
    def set_fenstr(self, s: str) -> None:
        for i, row in enumerate(s.split('/')):
            j = 0
            for c in row:
                if c.isnumeric():
                    for _ in range(int(c)):
                        self._board[i][j] = None
                        j += 1
                else:
                    self._board[i][j] = self._piece_factory.init_piece(c, Coords(8-i, File(j)))
                    j += 1


    def get_square_representation(self, val) -> str:
        if val is None:
            return ""
        else:
            return val.get_representation()

    def print_board(self):
        for i, row in enumerate(self._board):
            print(f"{8-i} {['{:^3}'.format(self.get_square_representation(val)) for val in row]}")
        print()
        print(f"  {['{:^3}'.format(File(i).name) for i in range(8)]}")

    def get_moves(self, player: Colour, enpassant_coords: Coords):
        moves = []

        for coords in self._all_squares_iterator():
            piece = self.get_square(coords)

            if piece and piece.colour == player:
                moves.extend(self.get_piece_moves(piece, coords))

        return moves 

    def get_piece_moves(self, piece, coords):
        moves = []

        candidate_moves = piece.get_candidate_moves(coords)

        for candidate in candidate_moves:
            for end_coords in candidate.generate_coords(coords):
                if self.get_square(end_coords) == None:
                    if candidate.capture_forced:
                        break
                    moves.append(Move(piece.colour, coords, False, end_coords, False))

                else:
                    blocking_piece = self.get_square(end_coords)
                    if candidate.capture_allowed and blocking_piece.colour != piece.colour:
                        moves.append(Move(piece.colour, coords, True, end_coords, False))
                    break

        return moves


    # Assumes the move is valid for now
    def make_move(self, move: Move):
        piece = self.get_square(move.start_coords)
        end_piece = self.get_square(move.end_coords)

        move.end_piece = end_piece

        self._previous_moves.append(move)

        self.set_square(None, move.start_coords)
        self.set_square(piece, move.end_coords)

    def undo_last_move(self):
        if len(self._previous_moves) == 0:
            print("No moves. found")
            return
        last_move = self._previous_moves.pop()

        piece = self.get_square(last_move.end_coords)

        self.set_square(piece, last_move.start_coords)
        self.set_square(last_move.end_piece, last_move.end_coords)

    def is_player_in_check(self, player: Colour):
        player_in_check = False

        # Find the player King
        king_coords = None
        for coords in self._all_squares_iterator():
            piece = self.get_square(coords)

            if piece and type(piece) == King and piece.colour == player:
                king_coords = coords
                break
        else:
            print("Error: No king found")


        # Check if 
        for coords in self._all_squares_iterator():
            piece = self.get_square(coords)

            if piece and piece.colour != player:
                for enemy_moves in self.get_piece_moves(piece, coords):
                    if enemy_moves.end_coords == king_coords:
                        player_in_check = True
                        break

        return player_in_check

    def is_player_left_in_check(self, move: Move):
        # Temporarily make move
        self.make_move(move)

        player_in_check = self.is_player_in_check(move.player_to_move)
        
        # Undo the move
        self.undo_last_move()

        return player_in_check
            
        
    # def _get_all_player_pieces(self, player: Colour):
    #     pieces = []
    #     for coords in self._all_squares_iterator():
    #         piece = self.get_square(coords)

    #         if piece and piece.colour == player:
    #             pieces.append(piece)

    #     return pieces

    def eval_piece_diff(self):
        val = 0

        for coords in self._all_squares_iterator():
            piece = self.get_square(coords)

            if piece is not None:
                mult = 1 if piece.colour == Colour.WHITE else -1

                val += mult * piece.value

        return val


    def _all_squares_iterator(self) -> list[Coords]:
        coords = []
        for i in range(1,len(self._board)+1):
            for j in range(len(self._board[0])):
                coords.append(Coords(i, File(j)))

        return coords