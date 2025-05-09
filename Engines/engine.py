from random import choice
from GameClasses.game import Game
from Moves.moveFactory import MoveFactory
from Moves.moveGenerator import MoveGenerator

class Engine():
    def __init__(self):
        self.game = Game()
        self.move_factory = MoveFactory()
        self.move_generator = MoveGenerator()

    def set_position(self, fenstr, moves):
        self.game.setup_fenstr(fenstr)

        for move_str in moves:
            # print(move_str)
            move = self.move_factory.init_long_algebraic(move_str, self.game.state.to_move)
            self.game.make_move(move)

    def go(self):
        moves = self.move_generator.get_all_moves(self.game)

        if len(moves) > 0:
            return choice(moves)

        return []

    def print_game_state(self):
        self.game.display_game()

        moves = self.move_generator.get_all_moves(self.game)

        print(f"{moves=}")
    




