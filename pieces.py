from enums import Colour

class Piece:
    def __init__(self, colour):
        self.colour = colour

class Pawn(Piece):
    def __init__(self, colour):
        super().__init__(colour)
        self.has_moved = False
    
    def get_representation(self):
        return 'p' if self.colour == Colour.BLACK else 'P'

class Rook(Piece):
    def __init__(self, colour):
        super().__init__(colour)

    def get_representation(self):
        return 'r' if self.colour == Colour.BLACK else 'R'

class Knight(Piece):
    def __init__(self, colour):
        super().__init__(colour)
    def get_representation(self):
        return 'n' if self.colour == Colour.BLACK else 'N'

class Bishop(Piece):
    def __init__(self, colour):
        super().__init__(colour)
    def get_representation(self):
        return 'b' if self.colour == Colour.BLACK else 'B'

class Queen(Piece):
    def __init__(self, colour):
        super().__init__(colour)
    def get_representation(self):
        return 'q' if self.colour == Colour.BLACK else 'Q'

class King(Piece):
    def __init__(self, colour):
        super().__init__(colour)
    def get_representation(self):
        return 'k' if self.colour == Colour.BLACK else 'K'
    
