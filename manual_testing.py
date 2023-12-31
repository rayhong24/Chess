import unittest

from game import Game
from board import Board
from Pieces.piece import Piece

def main():
    game = Game()
    game.display_game()
    game.board.handle_move("e2-e4")
    game.display_game()
    game.board.handle_move("d7-d5")
    game.display_game()
    game.board.handle_move("e4xd5")
    game.display_game()
    game.board.handle_move("d5-d6")
    game.display_game()
    game.board.handle_move("d6-d7")
    game.display_game()
    game.board.handle_move("d7xc8=Q")
    game.display_game()

if __name__ == "__main__":
    main()