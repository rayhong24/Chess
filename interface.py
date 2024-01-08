from game import Game
from Moves.moveFactory import MoveFactory

class Interface:
    def __init__(self):
        self.game = Game()
        self.move_factory = MoveFactory()

    def start_regular(self):
        while not self.game.ended:
            self.game.display_game()
            usr_input = input("Input a valid move (currently no error checking) or exit to stop the game: ")
            
            if usr_input == "exit":
                self.game.ended = True
            else:
                try:
                    move = self.move_factory.init_move(usr_input, self.game.player_turn)

                    if not move.make_move(self.game):
                        print("Invalid move. Try again")
                except:
                    print("Error")
                    pass


    # TODO: Implement to UCI standards
    # Talks to a uci interface
    def start_uci(self):
        return


       