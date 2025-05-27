from GameClasses.board import Board
from GameClasses.gameState import GameState

from enums import *
from coords import Coords
from Moves.move import Move


class Game():
    startpos_fenstr = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    # startpos_fenstr = "8/7P/8/8/8/8/8/k6K w KQkq - 0 1"
    # startpos_fenstr = "1r1qkb1r/pB1p1p1p/6p1/2p1p3/P7/6P1/1PPP1P1P/RNBQK1NR w KQk - 1 9"
    # startpos_fenstr = "r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 5 4"
    # startpos_fenstr = "r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4"
    # startpos_fenstr = "rnb1k1nr/pppp1ppp/5q2/2b1p3/4P3/1PN5/PBPP1PPP/R2QKBNR b KQkq - 2 4"
    # startpos_fenstr = "rnb1k1nr/pppp1ppp/8/2b1p3/4P3/1PN5/PBPP1qPP/R2QKBNR w KQkq - 0 5"
    # startpos_fenstr = "rnb1k1nr/pppp1ppp/5q2/2b1p3/4P3/1PN5/PBPP1PPP/R2QKBNR b KQkq - 2 4"
    # startpos_fenstr = "k7/8/8/8/8/8/PPPPPrPP/R3K2R w KQkq - 0 1"
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

        self.state.undo()

    def display_game(self) -> None:
        print("="*70)
        self.board.print_board()
        print(f"To move: {self.state.to_move}")
        print(f"Castling rights: {self.state.castling_rights}")
        print(f"EnPassant: {self.state.en_passant_target}")
        print("="*70)
