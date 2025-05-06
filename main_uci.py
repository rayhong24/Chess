from uci import Uci

from Engines.engine import Engine
from game import Game


def main():
    interface = Uci()
    interface.start_uci()
    # greeting_string = f"Hello, what would you like to do? (help for options)\nYour Input: "
    # while True:
        
    #     usr_input = input(greeting_string)

    #     # usr_input = self.sanitize(usr_input)
        
    #     if usr_input == "help":
    #         print("start - start a 2-player game")
    #         print("exit - exit the program")
    #         print()

    #     elif usr_input == "start":
    #         game = Game()
    #         game.start_game()
        
    #     elif usr_input == "uci":
    #         interface = Uci()
    #         interface.start_uci()



            

    #     elif usr_input == "exit":
    #         return

if __name__ == "__main__":
    main()