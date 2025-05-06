from uci import Uci

from Engines.engine import Engine
from game import Game

from Moves.moveFactory import MoveFactory


def main():
    greeting_string = f"Hello, what would you like to do? (help for options)\nYour Input: "
    game = Game()

    mf = MoveFactory()
    while True:
        usr_input = input(greeting_string)

        # usr_input = self.sanitize(usr_input)
        if game is not None:
            if usr_input == "print":
                game.display_game()

            elif usr_input == "moves":
                print(game.get_valid_moves())

            else:
                move = mf.init_long_algebraic(usr_input, game.player_turn)
                game.make_move(move)

        
        if usr_input == "help":
            print("start - start a 2-player game")
            print("exit - exit the program")
            print()

        elif usr_input == "uci":
            interface = Uci()
            interface.start_uci()


        elif usr_input == "exit":
            return

if __name__ == "__main__":
    main()