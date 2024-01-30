from Tests.TestGame.gameTestCases import GameTestCases
from enums import *

class TestCastling(GameTestCases.TestGameClass):
    def test_kingside_castle(self):
        # Might need to fix the castling rights fenstring check
        self.game.setup_fenstr("4k2r/8/8/8/8/8/8/4K2R w KQkq - 0 1")

        b = [
                ['','','','','k','','','r'],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','K','','','R']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        # Might need to fix the castling rights fenstring check
        self.check_castling_rights("Kk")

        move = self.move_factory.init_move("O-O", self.game.player_turn)
        move.make_move(self.game)

        b = [
                ['','','','','k','','','r'],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','R','K','']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("k")
