from Engines.engine import Engine

class Minimax(Engine):
    def __init__(self):
        super().__init__()

    def go(self, depth=2):
        if depth == 0 or 