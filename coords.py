from enums import *

class Coords:
    def __init__(self, rank: int, file: File):
        self.rank = rank
        self.file = file
    

    def __str__(self) -> str:
        return "{}{}".format(
            self.file.name,
            self.rank+1
        )
