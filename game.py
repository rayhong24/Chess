from string import ascii_uppercase

from board import Board
from player import Player
from enums import Colour
from enums import File


class Game():
    startpos_fenstr = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    def __init__(self, fenstr=None):
        if not fenstr:
            fenstr = self.startpos_fenstr
        self.board = Board()

        self.player_turn = Colour.WHITE
        self.player_white = Player()
        self.player_black = Player()

        self.setup_fenstr(fenstr)

    def setup_fenstr(self, fenstr: str) -> None:
        fenstr_sections = fenstr.split(' ')

        # Adds pieces to board
        self.board.add_fenstr_pieces(fenstr_sections[0])
        # Sets the turn
        self.player_turn = Colour.WHITE if fenstr_sections[1] == 'w' else Colour.BLACK
        # Sets castling rights
        self.board.set_castling_rights(fenstr_sections[2])

    def _get_file_input(self, orig=True) -> None:
        while True:
            if orig:
                orig_file = input("File of piece start. (Letters A-H): ")
            else:
                orig_file = input("File of piece destination (Letters A-H): ")

            if orig_file not in File._member_names_:
                print("Invalid File. Try again")
            else:
                return File[orig_file]
            
    def _get_row_input(self, orig=True):
        while True:
            if orig:
                row = input("Row of piece start. (1-8): ")
            else:
                row = input("Row of piece destination (1-8): ")
            

            if not (row.isdigit() and 1<= int(row) <= 8):
                print("Invalid row. Try again")
            else:
                return int(row)-1
    
    def _is_move_valid(self, orig_i, orig_j, new_i, new_j):
        orig_square = self.board[orig_i][orig_j]
        if orig_square is None or orig_square.colour != self.player_turn:
            return False
        orig_square.get_moves(self.board)
    
        return

    def _get_input(self):
        while True:
            orig_i = self._get_row_input(True)
            orig_j = self._get_file_input(True)

            new_i = self._get_row_input(False)
            new_j = self._get_file_input(False)

            if self._is_move_valid(orig_i, orig_j, new_i, new_j):
                break
            else:
                print("Invalid move. Try again.")
    
    # TODO: implement
    def make_move(self):
        self.player_turn = Colour.WHITE if self.player_turn == Colour.BLACK else Colour.BLACK

    # TODO: Refactor
    def get_castle_str(self) -> str:
        out = ""

        # TODO: Fix if no piece on hardcoded squares like (self.board.board[7][7] == None)
        if self.board.board[7][7].has_moved == False and\
        self.board.board[7][4].has_moved == False:
            out += "K"
        if self.board.board[7][0].has_moved == False and\
        self.board.board[7][4].has_moved == False:
            out += "Q"
        if self.board.board[0][7].has_moved == False and\
        self.board.board[0][4].has_moved == False:
            out += "k"
        if self.board.board[0][0].has_moved == False and\
        self.board.board[0][4].has_moved == False:
            out += "q"

        if out == "":
            return "-"
        else:
            return out

    def display_game(self) -> None:
        print("="*70)
        self.board.print_board()
        print(f"To move: {self.player_turn}")
        print(f"Castling rights: {self.get_castle_str()}")
        print("="*70)
    



