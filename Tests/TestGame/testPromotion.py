from Tests.TestGame.gameTestCases import GameTestCases
from enums import *

class TestCastling(GameTestCases.TestGameClass):
    def test_promotion(self):
        self.game.setup_fenstr("8/1P6/8/8/8/8/8/k1K5 w - - 0 1")

        b = [
                ['','','','','','','',''],\
                ['','P','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['k','','K','','','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("-")

        move = self.move_factory.init_move("b7-b8=B", self.game.player_turn, self.game)
        move.make_move(self.game)

        b = [
                ['','B','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['k','','K','','','','','']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("-")