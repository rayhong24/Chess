from game import Game
class Interface:
    def __init__(self):
        self.game = Game()

    def start_regular(self):
        while not self.game.ended:
            self.game.display_game()
            input = input("Input a valid move (currently no error checking) or end to stop the game: ")
            
            if input == "exit":
                self.game.ended = True
            else:
                self.game.make_move(input)

    # TODO: Implement to UCI standards
    # Talks to a uci interface
    def start_uci(self):
        return


       