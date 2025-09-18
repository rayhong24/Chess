from python_chess.Moves.moveGenerator import MoveGenerator
from python_chess.Moves.move import Move
from python_chess.Moves.castle import Castle

from python_chess.GameClasses.board import Board
from python_chess.GameClasses.game import Game

from python_chess.coords import Coords
from python_chess.enums import Colour
from python_chess.enums import File

from python_chess.Pieces.king import King
from python_chess.Pieces.rook import Rook

class rulesEngine():
    def __init__(self):
        self.move_generator = MoveGenerator()

    def get_valid_moves(self, game: Game):
        valid_moves = self.move_generator.generate_pseudo_legal_moves(game.board, game.state.to_move)

        valid_moves = list(filter(lambda m: self.is_castle_and_castle_valid(game, m), valid_moves))
        valid_moves = list(filter(lambda m: not self.does_leave_player_in_check(game, m), valid_moves))

        return valid_moves


    def can_player_capture_square(self, board: Board, player: Colour, capture_coords: Coords):
        for coords in board.all_squares_iterator():
            piece = board.get_square(coords)

            if piece and piece.colour == player:
                for moves in self.move_generator.get_piece_moves(board, piece, coords):
                    if moves.end_coords == capture_coords:
                        return True

        return False

    def is_in_check(self, board: Board, player: Colour):
        player_in_check = False

        # Find the player King
        king_coords = None
        for coords in board.all_squares_iterator():
            piece = board.get_square(coords)

            if piece and type(piece) == King and piece.colour == player:
                king_coords = coords
                break
        else:
            print("Error: No king found")
            board.print_board()
            print()


        # Check if king can be captured
        player_in_check = self.can_player_capture_square(board, player.other(), king_coords)

        return player_in_check

    def is_checkmate(self, game: Game):
        in_check = self.is_in_check(game.board, game.state.to_move)
        moves = self.get_valid_moves(game)

        return in_check and len(moves) == 0

    def does_leave_player_in_check(self, game: Game, move: Move):
        game.make_move(move)

        out = self.is_in_check(game.board, move.player_to_move)
        game.undo_move()

        return out

    def is_castle_and_castle_valid(self, game: Game, move: Move):
        move_to_castle_rights = {"e1g1": "K", 
                                 "e1c1": "Q",
                                 "e8g8": "k",
                                 "e8c8": "q"}

    
        # Check if the move is a castle move
        if type(move) != Castle:
            return True

        # Check if player still has castling rights
        if move_to_castle_rights[str(move)] not in game.state.castling_rights:
            return False

        # Check if rook is in the right position
        rook = game.board.get_square(move.rook_start_coords)
        if rook == None or type(rook) != Rook  or rook.colour != move.player_to_move:
            return False

        rank = move.start_coords.rank

        # Check if the king moves through check
        range_start = min(move.start_coords.file.value, move.end_coords.file.value)
        range_end = max(move.start_coords.file.value, move.end_coords.file.value)
        for file in range(range_start, range_end+1):
            check_coord = Coords(rank, File(file))
            if self.can_player_capture_square(game.board, move.player_to_move.other(), check_coord):
                return False

        # Check that all squares between the king and rook are empty
        range_start = min(move.start_coords.file.value, move.rook_start_coords.file.value)
        range_end = max(move.start_coords.file.value, move.rook_start_coords.file.value)
        for file in range(range_start + 1, range_end):
            check_coord = Coords(rank, File(file))
            if game.board.get_square(check_coord) != None:
                return False

        return True


        







        


