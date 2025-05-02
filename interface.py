
from Moves.moveFactory import MoveFactory

class Interface:
    def __init__(self):
        self.move_factory = MoveFactory()



    # TODO: Implement to UCI standards
    # Talks to a uci interface
    def start_uci(self):
        return


    def get_player_input(self, game):
        game.display_game()
        usr_input = input("Input a valid move: ")


        return usr_input

       