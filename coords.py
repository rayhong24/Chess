from enums import *

class Coords:
    def __init__(self, rank: int, file: File):
        self.rank = rank
        self.file = file

    def init_from_str(s: str):
        return Coords(8-int(s[1]), File[s[0]])
    
    def __eq__(self, other):
        if not isinstance(other, Coords):
            return False
        return self.rank == other.rank and self.file == other.file

    def __str__(self) -> str:
        return "{}{}".format(
            self.file.name,
            self.rank+1
        )

    def _is_inbounds(self, i: int, j: int):
        return 0<=i<=7 and 0<=j<=7

    # Generator for lazy evaluation
    def get_line(self, dir: tuple[int, int]):
        i, j = self.rank+dir[0], self.file.value+dir[1]

        while self._is_inbounds(i, j):
            yield Coords(i, File(j))

            i, j = i+dir[0], j+dir[1]





