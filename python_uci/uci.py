import rust_chess 
import time
from enum import Enum

from Engines.Minimax.minimax import Minimax

class Mode(Enum):
    python_minimax = 0
    rust_minimax = 1 

class Uci():
    def __init__(self):
        # self.engine = Minimax()
        # self.mode = Mode.python_minimax
        self.engine = rust_chess.PyMinimax(3, 8, True, True)
        self.mode = Mode.rust_minimax
        self.debug_mode = False
        self.running = False


        self.depth = 1

    def sanitize(self, input):
        return input.strip()

    def start_uci(self):
        # Send engine info to GUI
        print(f"id name FairyPenguin")
        print(f"id author RayHong")

        # Send engine options here
        # None so far
        print("uciok")

        self.running = True

        while self.running:
            usr_input = input()

            if len(usr_input) > 0:
                self.process_command(usr_input)

    def process_command(self, command: str):
        command = self.sanitize(command)

        tokens = command.split()

        if tokens[0] == "debug":
            self.process_debug_command(tokens[1])
        elif tokens[0] == "isready":
            print("readyok")
            pass
        elif tokens[0] == "setoption":
            pass
        elif tokens[0] == "register":
            pass
        elif tokens[0] == "ucinewgame":
            pass
        elif tokens[0] == "position":
            fen = None
            moves = []
            if tokens[1] == "startpos":
                fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"

                if len(tokens) > 2 and tokens[2] == "moves":
                    moves = tokens[3:]
            elif tokens[1] == "fen":
                fen = tokens[2]

                if len(tokens) > 3 and tokens[3] == "moves":
                    moves = tokens[4:]


            self.engine.set_position(fen, moves)

            


        elif tokens[0] == "go":
            # if self.mode == Mode.python_minimax:
            start = time.time()
            engine_move = self.engine.go()
            end = time.time()

            print(f"bestmove {engine_move}")
            print(f"total eval time: {end-start}")
            print(f"engine: {self.mode}")
                # self.engine.print_game_state()

            # else:
            #     start = time.time()
            #     engine_move = self.engine.go()
            #     end = time.time()

            #     print(f"bestmove {engine_move}")
            #     print(f"total eval time: {end-start}")
            #     self.engine.print_game_state()

                
        elif tokens[0] == "stop":
            pass
        elif tokens[0] == "ponderhit":
            pass
        elif tokens[0] == "quit":
            self.running = False

        elif tokens[0] == "print":
            self.engine.print_game_state()

        elif tokens[0] == "undo":
            self.engine.undo()




    def process_debug_command(self, token):
        if token == "on":
            self.debug_mode = True
        elif token == "false":
            self.debug_mode = False

