import random
import Engines.Minimax as Minimax

from .Evaluation.evaluator import Evaluator


from Engines.engine import Engine
from Pieces.knight import Knight

from coords import Coords
from enums import Colour

class Minimax(Engine):
    evaluator = Evaluator()

    def __init__(self):
        super().__init__()

    def go(self):
        move_evals = []
        best_move = None
        best_eval = -self.evaluator.checkmate_eval-1 if self.game.state.to_move == Colour.WHITE else self.evaluator.checkmate_eval+1

        self.rules_engine.is_checkmate(self.game)
        

        moves = self.rules_engine.get_valid_moves(self.game)

        if moves:
            random.shuffle(moves)

        for move in moves:
            self.game.make_move(move)
            eval = self.minimax()
            self.game.undo_move()

            move_evals.append((move, eval))

            if self.game.state.to_move == Colour.WHITE and eval > best_eval:
                best_move = move
                best_eval = eval

            elif self.game.state.to_move == Colour.BLACK and eval < best_eval:
                best_move = move
                best_eval = eval           

        move_evals.sort(key=lambda x: x[1])
        print(f"{move_evals=}")

        return best_move



    def minimax(self, depth=2, alpha=-40001, beta=40001):
        if self.rules_engine.is_checkmate(self.game):
            return self.evaluator.checkmate_eval if self.game.state.to_move == Colour.BLACK else -self.evaluator.checkmate_eval
        elif depth == 0:
            return self.evaluator.evaluate_game(self.game)


        if self.game.state.to_move == Colour.WHITE:
            value = -self.evaluator.checkmate_eval

            for move in self.rules_engine.get_valid_moves(self.game):
                self.game.make_move(move)
                value = max(value, self.minimax(depth-1))
                self.game.undo_move()

                if value >= beta:
                    break

                alpha = max(alpha, value)

            return value

        else:
            value = self.evaluator.checkmate_eval 
            
            for move in self.rules_engine.get_valid_moves(self.game):
                self.game.make_move(move)
                value = min(value, self.minimax(depth-1))
                self.game.undo_move()

                if value <= alpha:
                    break

                beta = min(beta, value)

            return value
        



