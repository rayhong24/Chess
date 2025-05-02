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
            move = self.move_factory.init_move_from_str(move_str, self.game.player_turn, self.game)
            move.make_move(self.game)

    def go(self):
        moves = self.game.get_valid_moves()

        return choice(moves)
        
    




