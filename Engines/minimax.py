from Engines.engine import Engine

import random

from enums import Colour

class Minimax(Engine):
    def __init__(self):
        super().__init__()

    # def go(self):
        # best_move = None
        # to_move = self.game.state.to_move

        # best_eval = -1001 if to_move == Colour.WHITE else 1001


        # valid_moves = self.game.get_valid_moves()

        # if valid_moves:
        #     random.shuffle(valid_moves)

        # for move in valid_moves:
        #     self.game.make_move(move)
        #     resulting_eval = self.game.evaluate_state(1)
        #     self.game.undo_move()

        #     if to_move == Colour.WHITE and resulting_eval > best_eval:
        #         best_move = move
        #         best_eval = resulting_eval

        #     elif to_move == Colour.BLACK and resulting_eval < best_eval:
        #         best_move = move
        #         best_eval = resulting_eval

        
        # return best_move

