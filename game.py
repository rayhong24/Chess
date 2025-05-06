from board import Board
from player import Player
from enums import *
from coords import Coords
from Moves.move import Move


class Game():
    startpos_fenstr = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    def __init__(self, fenstr=startpos_fenstr):
        self.board = Board()

        self.player_turn = Colour.WHITE

        # Not implemented/ used
        # self.player_white = Player(Colour.WHITE)
        # self.player_black = Player(Colour.BLACK)

        self.setup_fenstr(fenstr)

        self.ended = False

    def setup_fenstr(self, fenstr: str=startpos_fenstr) -> None:
        if not fenstr:
            fenstr = self.startpos_fenstr
        fenstr_sections = fenstr.split(' ')

        # Adds pieces to board
        self.board.set_fenstr(fenstr_sections[0])
        # Sets the turn
        self.player_turn = Colour.WHITE if fenstr_sections[1] == 'w' else Colour.BLACK
        # Sets castling rights
        # TODO Reimplement functionality
        # self.board.set_castling_rights(fenstr_sections[2])
        # Sets enpassant_coords
        if fenstr_sections[3] == "-":
            self.enpassant_coords = None
        else:
            self.enpassant_coords = Coords.init_from_str(fenstr_sections[3])


    def get_valid_moves(self):
        valid_moves = self.board.get_moves(self.player_turn, self.enpassant_coords)

        filtered_moves = list(filter(lambda m: not self.board.is_player_left_in_check(m), valid_moves))

        return filtered_moves

    def get_player(self, colour):
        return self.player_white if colour == colour.WHITE else self.player_black
    
    def make_move(self, move: Move) -> bool:
        # Valid move
        self.board.make_move(move)

        self.switch_player_turn()

    def switch_player_turn(self):
        self.player_turn = Colour.WHITE if self.player_turn == Colour.BLACK else Colour.BLACK

    def is_checkmate(self):
        in_check = self.board.is_player_in_check(self.player_turn)
        moves = self.get_valid_moves()

        return in_check and len(moves) == 0 


    def get_castle_str(self) -> str:
        return "Not implemented"

    def display_game(self) -> None:
        print("="*70)
        self.board.print_board()
        print(f"To move: {self.player_turn}")
        print(f"Castling rights: {self.get_castle_str()}")
        print(f"EnPassant: {self.enpassant_coords}")
        print("="*70)
