from Tests.TestGame.gameTestCases import GameTestCases
from enums import Colour

class testGameFunctions(GameTestCases.TestGameClass):
    def setUp(self):
        return super().setUp()
    
    def test_starting_position(self):
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

    def test_fenstring_setup(self):
        fenstr = "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50"
        self.game.setup_fenstr(fenstr)

        b = [
                ['','','','','','','',''],\
                ['','','','','','k','',''],\
                ['','','','p','','','',''],\
                ['','p','','P','p','','','p'],\
                ['p','P','','','P','p','','P'],\
                ['P','','','','','P','','K'],\
                ['','','','','','','',''],\
                ['','','','','','','','']
            ]
        self.check_board_equal(b)
        self.check_move(Colour.BLACK)
        self.check_castling_rights('-')

    def test_get_valid_moves_starting_position(self):
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
        expected_moves = [
            # Pawn moves
            self.move_factory.init_move("a2-a3", Colour.WHITE),
            self.move_factory.init_move("a2-a4", Colour.WHITE),
            self.move_factory.init_move("b2-b3", Colour.WHITE),
            self.move_factory.init_move("b2-b4", Colour.WHITE),
            self.move_factory.init_move("c2-c3", Colour.WHITE),
            self.move_factory.init_move("c2-c4", Colour.WHITE),
            self.move_factory.init_move("d2-d3", Colour.WHITE),
            self.move_factory.init_move("d2-d4", Colour.WHITE),
            self.move_factory.init_move("e2-e3", Colour.WHITE),
            self.move_factory.init_move("e2-e4", Colour.WHITE),
            self.move_factory.init_move("f2-f3", Colour.WHITE),
            self.move_factory.init_move("f2-f4", Colour.WHITE),
            self.move_factory.init_move("g2-g3", Colour.WHITE),
            self.move_factory.init_move("g2-g4", Colour.WHITE),
            self.move_factory.init_move("h2-h3", Colour.WHITE),
            self.move_factory.init_move("h2-h4", Colour.WHITE),

            # Knight moves
            self.move_factory.init_move("Nb1-a3", Colour.WHITE),
            self.move_factory.init_move("Nb1-c3", Colour.WHITE),
            self.move_factory.init_move("Ng1-f3", Colour.WHITE),
            self.move_factory.init_move("Ng1-h3", Colour.WHITE),
        ]

        self.check_board_equal(b)
        self.check_move(Colour.WHITE)
        self.check_castling_rights('KQkq')
        self.check_valid_moves(self.game, expected_moves)
               

