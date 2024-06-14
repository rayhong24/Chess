import game as gm
from Moves.moveFactory import MoveFactory

class Interface:
    def __init__(self):
        self.move_factory = MoveFactory()

    def sanitize(self, input):
        return input.strip()

    def add_dash(self, input):
        if input == "O-O" or input == "O-O-O":
            return input

        ind_after_num = 2 if input[1].isdigit() else 3
        if input[ind_after_num] != 'x' and input[ind_after_num] != "-":
            input = input[:ind_after_num] + '-' + input[ind_after_num:]

        return input

    def change_to_castle_notation(self, input):
        if input == "e1-g1" or input == "e8-g8":
            input =  "O-O"
        elif input == "e1-c1" or input == "e8-c8":
            input = "O-O-O"

        return input

    def start_regular(self):
        greeting_string = f"Hello, what would you like to do? (help for options)\nYour Input: "
        while True:
            
            usr_input = input(greeting_string)

            usr_input = self.sanitize(usr_input)
            
            if usr_input == "help":
                print("start - start a 2-player game")
                print("exit - exit the program")
                print()

            elif usr_input == "start":
                game = gm.Game()

                game.start_game()

            elif usr_input == "exit":
                return


    # TODO: Implement to UCI standards
    # Talks to a uci interface
    def start_uci(self):
        return


    def get_player_input(self, game):
        game.display_game()
        usr_input = input("Input a valid move: ")

        usr_input = self.sanitize(usr_input)
        usr_input = self.add_dash(usr_input)
        usr_input = self.change_to_castle_notation(usr_input)

        return usr_input

       