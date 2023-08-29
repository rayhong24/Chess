from Pieces.piece import Piece

class Player():
    def __init__(self) -> None:
        self.pieces: set[Piece] = set()
        # attributes below are not used right now
        self.name = None
        self.time = None

    def add_piece(self, piece: Piece) -> None:
        self.pieces.add(piece)

    def remove_piece(self, piece: Piece) -> None:
        self.pieces.remove(piece)

    def get_moves(self) -> [str]:
        moves = []

        for piece in self.pieces:
            moves.extend(piece.get_moves())

        return moves
