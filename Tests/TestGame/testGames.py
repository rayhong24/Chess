from Tests.TestGame.gameTestCases import GameTestCases
from enums import *

class TestGames(GameTestCases.TestGameClass):
    def test_game(self):
        self.game.setup_fenstr()

        b = [
                ['r','n','b','q','k','b','n','r'],\
                ['p','p','p','p','p','p','p','p'],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['P','P','P','P','P','P','P','P'],\
                ['R','N','B','Q','K','B','N','R']
            ]

        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("KQkq")

        move = self.move_factory.init_move_from_str("e2-e4", self.game.player_turn, self.game)
        self.game.make_move(move)
        b = [
                ['r','n','b','q','k','b','n','r'],\
                ['p','p','p','p','p','p','p','p'],\
                ['','','','','','','',''],\
                ['','','','','','','',''],\
                ['','','','','P','','',''],\
                ['','','','','','','',''],\
                ['P','P','P','P','','P','P','P'],\
                ['R','N','B','Q','K','B','N','R']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("KQkq")

        move = self.move_factory.init_move_from_str("e7-e5", self.game.player_turn, self.game)
        self.game.make_move(move)
        b = [
                ['r','n','b','q','k','b','n','r'],\
                ['p','p','p','p','','p','p','p'],\
                ['','','','','','','',''],\
                ['','','','','p','','',''],\
                ['','','','','P','','',''],\
                ['','','','','','','',''],\
                ['P','P','P','P','','P','P','P'],\
                ['R','N','B','Q','K','B','N','R']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("KQkq")

        move = self.move_factory.init_move_from_str("Ng1-f3", self.game.player_turn, self.game)
        self.game.make_move(move)
        b = [
                ['r','n','b','q','k','b','n','r'],\
                ['p','p','p','p','','p','p','p'],\
                ['','','','','','','',''],\
                ['','','','','p','','',''],\
                ['','','','','P','','',''],\
                ['','','','','','N','',''],\
                ['P','P','P','P','','P','P','P'],\
                ['R','N','B','Q','K','B','','R']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("KQkq")


        move = self.move_factory.init_move_from_str("Nb8-c6", self.game.player_turn, self.game)
        self.game.make_move(move)
        b = [
                ['r','','b','q','k','b','n','r'],\
                ['p','p','p','p','','p','p','p'],\
                ['','','n','','','','',''],\
                ['','','','','p','','',''],\
                ['','','','','P','','',''],\
                ['','','','','','N','',''],\
                ['P','P','P','P','','P','P','P'],\
                ['R','N','B','Q','K','B','','R']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("KQkq")

        move = self.move_factory.init_move_from_str("Bf1-c4", self.game.player_turn, self.game)
        self.game.make_move(move)
        b = [
                ['r','','b','q','k','b','n','r'],\
                ['p','p','p','p','','p','p','p'],\
                ['','','n','','','','',''],\
                ['','','','','p','','',''],\
                ['','','B','','P','','',''],\
                ['','','','','','N','',''],\
                ['P','P','P','P','','P','P','P'],\
                ['R','N','B','Q','K','','','R']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("KQkq")

        move = self.move_factory.init_move_from_str("Bf8-c5", self.game.player_turn, self.game)
        self.game.make_move(move)
        b = [
                ['r','','b','q','k','','n','r'],\
                ['p','p','p','p','','p','p','p'],\
                ['','','n','','','','',''],\
                ['','','b','','p','','',''],\
                ['','','B','','P','','',''],\
                ['','','','','','N','',''],\
                ['P','P','P','P','','P','P','P'],\
                ['R','N','B','Q','K','','','R']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("KQkq")

        move = self.move_factory.init_move_from_str("O-O", self.game.player_turn, self.game)
        self.game.make_move(move)
        b = [
                ['r','','b','q','k','','n','r'],\
                ['p','p','p','p','','p','p','p'],\
                ['','','n','','','','',''],\
                ['','','b','','p','','',''],\
                ['','','B','','P','','',''],\
                ['','','','','','N','',''],\
                ['P','P','P','P','','P','P','P'],\
                ['R','N','B','Q','','R','K','']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("kq")

        move = self.move_factory.init_move_from_str("Ng8-f6", self.game.player_turn, self.game)
        self.game.make_move(move)
        b = [
                ['r','','b','q','k','','','r'],\
                ['p','p','p','p','','p','p','p'],\
                ['','','n','','','n','',''],\
                ['','','b','','p','','',''],\
                ['','','B','','P','','',''],\
                ['','','','','','N','',''],\
                ['P','P','P','P','','P','P','P'],\
                ['R','N','B','Q','','R','K','']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("kq")

        move = self.move_factory.init_move_from_str("Nb1-c3", self.game.player_turn, self.game)
        self.game.make_move(move)
        b = [
                ['r','','b','q','k','','','r'],\
                ['p','p','p','p','','p','p','p'],\
                ['','','n','','','n','',''],\
                ['','','b','','p','','',''],\
                ['','','B','','P','','',''],\
                ['','','N','','','N','',''],\
                ['P','P','P','P','','P','P','P'],\
                ['R','','B','Q','','R','K','']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights("kq")

        move = self.move_factory.init_move_from_str("O-O", self.game.player_turn, self.game)
        self.game.make_move(move)
        b = [
                ['r','','b','q','','r','k',''],\
                ['p','p','p','p','','p','p','p'],\
                ['','','n','','','n','',''],\
                ['','','b','','p','','',''],\
                ['','','B','','P','','',''],\
                ['','','N','','','N','',''],\
                ['P','P','P','P','','P','P','P'],\
                ['R','','B','Q','','R','K','']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights("-")
