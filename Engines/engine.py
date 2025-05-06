from random import choice
from game import Game
from Moves.moveFactory import MoveFactory

class Engine():
    def __init__(self):
        self.game = Game()
        self.move_factory = MoveFactory()

    def set_position(self, fenstr, moves):
        self.game.setup_fenstr(fenstr)

        for move_str in moves:
            # print(move_str)
            move = self.move_factory.init_long_algebraic(move_str, self.game.player_turn)
            self.game.make_move(move)

    def go(self):
        moves = self.game.get_valid_moves()

        if len(moves) > 0:
            return choice(moves)

        return []

    def print_game_state(self):
        self.game.display_game()


        eval = self.game.evaluate_state()
        print(f"{eval=}")

        moves = self.game.get_valid_moves()
        print(f"{moves=}")
        is_checkmate = self.game.is_checkmate()
        print(f"{is_checkmate}")

    




