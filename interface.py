from game import Game
class Interface:
    def __init__(self):
        self.game = Game()

    def start_regular(self):
        while not self.game.ended:
            self.game.display_game()
            usr_input = input("Input a valid move (currently no error checking) or exit to stop the game: ")
            
            if usr_input == "exit":
                self.game.ended = True
            else:
                if not self.game.make_move(usr_input):
                    print("Invalid move. Try again")


    # TODO: Implement to UCI standards
    # Talks to a uci interface
    def start_uci(self):
        return


       