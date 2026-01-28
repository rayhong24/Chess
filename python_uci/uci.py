import rust_chess 
import time
from enum import Enum

from Engines.Minimax.minimax import Minimax
from python_uci.commands import Command, CommandParser

class Mode(Enum):
    python_minimax = 0
    rust_minimax = 1 

class Uci():
    def __init__(self):
        # self.engine = Minimax()
        # self.mode = Mode.python_minimax
        self.engine = rust_chess.PyMinimax(2, 4, True, True)
        self.mode = Mode.rust_minimax
        self.debug_mode = False
        self.running = False

        self.depth = 1

        self.handlers = {
            "debug": self._handle_debug,
            "isready": self._handle_isready,
            "position": self._handle_position,
            "go": self._handle_go,
            "quit": self._handle_quit,
            "print": self._handle_print,
            "undo": self._handle_undo,
        }

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
            raw = input()
            cmd = CommandParser.parse(raw)
            if cmd.name:
                self.process_command(cmd)

    def process_command(self, cmd: Command):
        handler = self.handlers.get(cmd.name)
        if handler:
            handler(cmd)
        else:
            pass

    def _handle_debug(self, cmd: Command):
        if cmd.args:
            token = cmd.args[0]
            if token == "on":
                self.debug_mode = True
            elif token == "off":
                self.debug_mode = False
                j
    def _handle_isready(self, cmd: Command):
        print("readyok")

    def _handle_position(self, cmd: Command):
        fen = cmd.options.get("fen")
        moves = cmd.options.get("moves", [])
        if fen == "startpos":
            fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        self.engine.set_position(fen, moves)

    def _handle_go(self, cmd: Command):
        start = time.time()
        engine_move = self.engine.go()
        end = time.time()

        print(f"bestmove {engine_move}")
        print(f"total eval time: {end-start}")
        print(f"engine: {self.mode}")

    def _handle_quit(self, cmd: Command):
        self.running = False

    def _handle_print(self, cmd: Command):
        self.engine.print_game_state()

    def _handle_undo(self, cmd: Command):
        self.engine.undo()

