from string import ascii_uppercase

from board import Board
from player import Player
from enums import Colour
from enums import File
from Moves.move import Move


class Game():
    startpos_fenstr = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    def __init__(self, fenstr=None):
        self.board = Board()

        self.player_turn = Colour.WHITE
        self.player_white = Player()
        self.player_black = Player()

        self.enpassant_coords = None

        self.setup_fenstr(fenstr)

        self.ended = False

    def setup_fenstr(self, fenstr: str=None) -> None:
        if not fenstr:
            fenstr = self.startpos_fenstr
        fenstr_sections = fenstr.split(' ')

        # Adds pieces to board
        self.board.add_fenstr_pieces(fenstr_sections[0])
        # Sets the turn
        self.player_turn = Colour.WHITE if fenstr_sections[1] == 'w' else Colour.BLACK
        # Sets castling rights
        self.board.set_castling_rights(fenstr_sections[2])
        # Sets enpassant_coords
        self.board
    
    # INPUT: move - string in long algebraic notation
    def make_move(self, move: Move) -> bool:
        # Valid move
        if move.make_move(self.board):
            self.player_turn = Colour.WHITE if self.player_turn == Colour.BLACK else Colour.BLACK
            return True
        else:
            return False

    def switch_player_turn(self):
        self.player_turn = Colour.WHITE if self.player_turn == Colour.BLACK else Colour.BLACK


    # TODO: Refactor
    def get_castle_str(self) -> str:
        out = ""

        # TODO: Fix if no piece on hardcoded squares like (self.board.board[7][7] == None)
        if self.board.board[7][7] and self.board.board[7][7].has_moved == False and\
        self.board.board[7][4] and self.board.board[7][4].has_moved == False:
            out += "K"
        if self.board.board[7][0] and self.board.board[7][0].has_moved == False and\
        self.board.board[7][4] and self.board.board[7][4].has_moved == False:
            out += "Q"
        if self.board.board[0][7] and self.board.board[0][7].has_moved == False and\
        self.board.board[0][4] and self.board.board[0][4].has_moved == False:
            out += "k"
        if self.board.board[0][0] and self.board.board[0][0].has_moved == False and\
        self.board.board[0][4] and self.board.board[0][4].has_moved == False:
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
    



