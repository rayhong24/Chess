from GameClasses.board import Board
from GameClasses.gameState import GameState

from enums import *
from coords import Coords
from Moves.move import Move


class Game():
    startpos_fenstr = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"

    def __init__(self, fenstr=startpos_fenstr):
        self.board = Board()

        self.state = GameState()

        self.move_history = []

        self.setup_fenstr(fenstr)

        self.ended = False

    def setup_fenstr(self, fenstr: str=startpos_fenstr) -> None:
        if not fenstr:
            fenstr = self.startpos_fenstr
        fenstr_sections = fenstr.split(' ')

        # Adds pieces to board
        self.board.set_fenstr(fenstr_sections[0])
        
        # Sets the turn
        self.state.to_move = Colour.WHITE if fenstr_sections[1] == 'w' else Colour.BLACK

        # Sets castling rights
        self.state.castling_rights = fenstr_sections[2]

        # Sets enpassant_coords
        if fenstr_sections[3] == "-":
            self.state.en_passant_target = None
        else:
            self.state.en_passant_target = Coords.init_from_str(fenstr_sections[3])

    def make_move(self, move: Move) -> bool:
        # Valid move
        move.apply(self.board)

        self.move_history.append(move)

        self.state.update(move, self.board)

    def undo_move(self) -> bool:
        last_move = self.move_history.pop()

        last_move.undo(self.board)

        self.state.undo(last_move, self.board)

    def display_game(self) -> None:
        print("="*70)
        self.board.print_board()
        print(f"To move: {self.state.to_move}")
        print(f"Castling rights: {self.state.castling_rights}")
        print(f"EnPassant: {self.state.en_passant_target}")
        print("="*70)
