from random import choice
from GameClasses.game import Game
from Moves.moveFactory import MoveFactory
from GameClasses.rulesEngine import rulesEngine

class Engine():
    def __init__(self):
        self.game = Game()
        self.move_factory = MoveFactory()
        self.rules_engine = rulesEngine()

    def set_position(self, fenstr, moves):
        self.game.setup_fenstr(fenstr)

        for move_str in moves:
            move = self.move_factory.init_move_from_str(move_str, self.game)
            self.game.make_move(move)

    def go(self):
        moves = self.rules_engine.get_valid_moves(self.game)

        if len(moves) > 0:
            return choice(moves)

        return []

    def print_game_state(self):
        self.game.display_game()

        moves = self.rules_engine.get_valid_moves(self.game)

        print(f"{moves=}")
    
    def undo(self):
        self.game.undo_move()




