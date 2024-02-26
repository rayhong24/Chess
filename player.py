import interface
from Moves.move import Move
from Moves.moveFactory import MoveFactory
from enums import *

class Player():
    def __init__(self, colour: Colour) -> None:
        self.interface = interface.Interface()
        self.move_factory = MoveFactory()
        self.colour = colour

        # attributes below are not used right now
        self.name = None
        self.time = None

    def add_piece(self, piece) -> None:
        self.pieces.add(piece)

    def remove_piece(self, piece) -> None:
        self.pieces.remove(piece)

    def choose_move(self, game) -> Move:
        chosen_move = None

        while chosen_move not in game.get_valid_moves():
            usr_input = self.interface.get_player_input(game)

            try:
                chosen_move = self.move_factory.init_move_from_str(usr_input, self.colour, game)

            except:
                print("Illegal move. Try again")

        return chosen_move


