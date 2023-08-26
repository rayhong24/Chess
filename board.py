from enums import Colour
from enums import File
from Pieces.piece import Piece
from Pieces.pawn import Pawn
from Pieces.rook import Rook
from Pieces.knight import Knight
from Pieces.bishop import Bishop
from Pieces.queen import Queen
from Pieces.king import King

class Board:
    def __init__(self):
        self.board = [[None]*8 for _ in range(8)]
        # Maybe change to a dictionary
        self.white_pieces = []
        self.black_pieces = []

        self.__setup_board()

    def __add_piece(self, piece_type: Piece, colour: Colour, i: int, j: int):
        piece = piece_type(colour, i, j)
        self.board[i][j] = piece
        
        if colour == Colour.WHITE:
            self.white_pieces.append(piece)
        else:
            self.black_pieces.append(piece)


    def __setup_board(self):
        # Place pawns
        for j in range(8):
            self.__add_piece(Pawn, Colour.BLACK, 1, j)

            self.__add_piece(Pawn, Colour.WHITE, 6, j)

        # Place rooks
        self.__add_piece(Rook, Colour.BLACK, 0, 0)
        self.__add_piece(Rook, Colour.BLACK, 0, 7)
        self.__add_piece(Rook, Colour.WHITE, 7, 0)
        self.__add_piece(Rook, Colour.WHITE, 7, 7)

        # Place knights
        self.__add_piece(Knight, Colour.BLACK, 0, 1)
        self.__add_piece(Knight, Colour.BLACK, 0, 6)
        self.__add_piece(Knight, Colour.BLACK, 7, 1)
        self.__add_piece(Knight, Colour.BLACK, 7, 6)

        # Place bishops
        self.__add_piece(Bishop, Colour.BLACK, 0, 2)
        self.__add_piece(Bishop, Colour.BLACK, 0, 5)
        self.__add_piece(Bishop, Colour.BLACK, 7, 2)
        self.__add_piece(Bishop, Colour.BLACK, 7, 5)

        # Place Queens
        self.__add_piece(Queen, Colour.BLACK, 0, 3)
        self.__add_piece(Queen, Colour.WHITE, 7, 3)

        # Place Kings
        self.__add_piece(King, Colour.BLACK, 0, 4)
        self.__add_piece(King, Colour.BLACK, 7, 4)
    
    def __get_square_representation(self, val: Piece) -> str:
        if val is None:
            return ""
        else:
            return val.get_representation()

    def print_board(self):
        for i, row in enumerate(self.board):
            print(f"{i} {['{:^3}'.format(self.__get_square_representation(val)) for val in row]}")
        print()
        print(f"  {['{:^3}'.format(File(i).name) for i in range(8)]}")

        print("="*57)
    
    def is_inbounds(self, i: int, j: int) -> bool:
        return (0 <= i < 8) and (0 <= j < 8)

    def handle_move(self, move: str) -> bool:
        start_i, start_j = int(move[1]), File[move[0]].value
        new_i, new_j = int(move[3]), File[move[2]].value
        self.move_piece(start_i, start_j, new_i, new_j)

    def move_piece(self, orig_i: int, orig_j: int, new_i: int, new_j: int):
        self.board[orig_i][orig_j].move(new_i, new_j)
        self.board[orig_i][orig_j], self.board[new_i][new_j] = None, self.board[orig_i][orig_j]

