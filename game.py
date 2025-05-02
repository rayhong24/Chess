from board import Board
from player import Player
from enums import *
from coords import Coords
from Moves.move import Move


class Game():
    startpos_fenstr = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    def __init__(self, fenstr=None):
        self.board = Board()

        self.player_turn = Colour.WHITE
        self.player_white = Player(Colour.WHITE)
        self.player_black = Player(Colour.BLACK)


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
        if fenstr_sections[3] == "-":
            self.enpassant_coords = None
        else:
            self.enpassant_coords = Coords.init_from_str(fenstr_sections[3])


    def get_valid_moves(self):

        valid_moves = []

        # for piece in current_player_pieces:
            # valid_moves.extend(piece.get_moves(self))

        return valid_moves

    def get_player(self, colour):
        return self.player_white if colour == colour.WHITE else self.player_black

    def start_game(self):
        while not self.ended:
            current_player = self.get_player(self.player_turn)

            # Assuming valid move is returned
            chosen_move = current_player.choose_move(self)

            self.make_move(chosen_move)

    
    def make_move(self, move: Move) -> bool:
        # Valid move
        self.board.move_piece(move.start_coords, move.end_coords)

        self.switch_player_turn()

    def switch_player_turn(self):
        self.player_turn = Colour.WHITE if self.player_turn == Colour.BLACK else Colour.BLACK


    def is_king_in_check(self, colour: Colour):
        king_repr = 'K' if colour == Colour.WHITE else 'k'

        for i in range(8):
            for j in range(8):
                square = self.board.board[i][j]
                if square is not None and square.get_representation() == king_repr:
                    return self.is_square_in_check(Coords.init_from_indices(i, j), colour)

    def is_square_in_check(self, coords,colour):
        opponent_pieces = self.board.black_pieces if colour == Colour.WHITE else self.board.white_pieces

        for piece in opponent_pieces:
            for move in piece.get_moves(self):
                if move.end_coords == coords:
                    return True
        
        return False


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
        print(f"EnPassant: {self.enpassant_coords}")
        print("="*70)
