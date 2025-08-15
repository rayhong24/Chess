import random

from Engines.engine import Engine
from Pieces.knight import Knight

from coords import Coords
from enums import Colour

class Minimax(Engine):
    def __init__(self):
        super().__init__()

    def go(self):
        move_evals = []
        best_move = None
        best_eval = -1001 if self.game.state.to_move == Colour.WHITE else 1001


        self.rules_engine.is_checkmate(self.game)
        

        moves = self.rules_engine.get_valid_moves(self.game)

        # if moves:
        #     random.shuffle(moves)

        for move in moves:
            self.game.make_move(move)
            eval = self.minimax()
            self.game.undo_move()

            if type(self.game.board.get_square(Coords.init_from_str("a5"))) == Knight:
                self.game.display_game()


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



    def minimax(self, depth=1, alpha=-1001, beta=1001):
        if self.rules_engine.is_checkmate(self.game):
            return 1000 if self.game.state.to_move == Colour.BLACK else -1000
        elif depth == 0:
            return self.state_heuristic()


        if self.game.state.to_move == Colour.WHITE:
            value = -1000

            for move in self.rules_engine.get_valid_moves(self.game):
                self.game.make_move(move)
                value = max(value, self.minimax(depth-1))
                self.game.undo_move()

                if value >= beta:
                    break

                alpha = max(alpha, value)

            return value

        else:
            value = 1000
            
            for move in self.rules_engine.get_valid_moves(self.game):
                self.game.make_move(move)
                value = min(value, self.minimax(depth-1))
                self.game.undo_move()

                if value <= alpha:
                    break

                beta = min(beta, value)

            return value
        
    def state_heuristic(self):
        value = 0

        for coords in self.game.board.all_squares_iterator():
            piece = self.game.board.get_square(coords)

            if piece:
                # mult = 1 if piece.colour == Colour.WHITE else -1
                value += piece.get_value(coords) 

        return value



