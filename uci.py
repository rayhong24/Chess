from engine import Engine

class Uci():
    def __init__(self):
        self.engine = Engine()
        self.debug_mode = False
        self.running = False

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
                fen = None

                if len(tokens) > 2 and tokens[2] == "moves":
                    moves = tokens[3:]
            elif tokens[1] == "fen":
                fen = tokens[2]

                if len(tokens) > 3 and tokens[3] == "moves":
                    moves = tokens[4:]

            self.engine.set_position(fen, moves)

            


        elif tokens[0] == "go":
            engine_move = self.engine.go()

            print(f"bestmove {engine_move}")
            
        elif tokens[0] == "stop":
            pass
        elif tokens[0] == "ponderhit":
            pass
        elif tokens[0] == "quit":
            self.running = False




    def process_debug_command(self, token):
        if token == "on":
            self.debug_mode = True
        elif token == "false":
            self.debug_mode = False

