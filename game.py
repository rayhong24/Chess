from string import ascii_uppercase

from board import Board
from enums import Colour
from enums import File


class Game():
    def __init__(self):
        self.board = Board()
        self.player_turn = Colour.WHITE

    def _get_file_input(self, orig=True):
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




    
    def make_move(self):
        self.player_turn = not self.player_turn


