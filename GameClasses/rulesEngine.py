from Moves.moveGenerator import MoveGenerator
from Moves.move import Move
from Moves.castle import Castle

from GameClasses.board import Board
from GameClasses.game import Game

from coords import Coords
from enums import Colour

from Pieces.king import King

class rulesEngine():
    def __init__(self):
        self.move_generator = MoveGenerator()

    def get_valid_moves(self, game: Game):
        pseudo_moves = self.move_generator.generate_pseudo_legal_moves(game.board, game.state.to_move)

        valid_moves = list(filter(lambda m: not self.does_leave_player_in_check(game, m), pseudo_moves))
        valid_moves = list(filter(lambda m: self.is_castle_and_castle_valid(game, m), pseudo_moves))

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

        if type(move) == Castle:
            if move_to_castle_rights[str(move)] not in game.state.castling_rights:
                return False

            rank = move.start_coords.rank

            for file in range(move.start_coords.file.value, move.end_coords.file.value+1):
                check_coord = Coords(rank, file)

                if self.can_player_capture_square(game.board, move.player_to_move.other(), check_coord):
                    return False

        return True


        







        


