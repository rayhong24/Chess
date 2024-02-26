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

    def _is_inbounds(self, rank: int, file: File):
        return 1<=rank<=8 and 1<=file.value<=8

    # Generator for lazy evaluation
    def get_line(self, dir: tuple[int, int]):
        new_rank, new_file = self.rank+dir[0], File(self.file.value+dir[1])

        while self._is_inbounds(new_rank, new_file):
            yield Coords(new_rank, new_file)





