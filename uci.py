import interface

class Uci(interface):
    def __init__(self):
        self.debug_mode = False

    def process_command(self, command: str):
        command = self.sanitize(command)

        tokens = command.split()

        if tokens[0] == "debug":
            self.process_debug_command(tokens[1])
        elif tokens[0] == "isready":
            pass
        elif tokens[0] == "setoption":
            pass
        elif tokens[0] == "register":
            pass
        elif tokens[0] == "ucinewgame":
            pass
        elif tokens[0] == "position":
            pass
        elif tokens[0] == "go":
            pass
        elif tokens[0] == "stop":
            pass
        elif tokens[0] == "ponderhit":
            pass
        elif tokens[0] == "quit":
            pass




    def process_debug_command(self, token):
        if token == "on":
            self.debug_mode = True
        elif token == "false":
            self.debug_mode = False

