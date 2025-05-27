from enums import *

class Coords:
    def __init__(self, rank: int, file: File):
        assert(type(file) == File)
        self.rank = rank
        self.file = file

    def init_from_str(s: str):
        return Coords(int(s[1]), File[s[0]])

    def __eq__(self, other):
        if not isinstance(other, Coords):
            return False
        return self.rank == other.rank and self.file == other.file

    def __repr__(self) -> str:
        return self.__str__()

    def __str__(self) -> str:
        return "{}{}".format(
            self.file.name,
            self.rank
        )


    def diff_inbounds(self, di: int, dj: int):
        return 1<=self.rank + di<=8 and 0<=self.file.value + dj<=7

    # Does not have to be a direct neighbour
    def get_neighbour(self, rank_diff, file_diff):
        if self.diff_inbounds(rank_diff, file_diff):
            return Coords(self.rank+rank_diff, File(self.file.value+file_diff))

        else:
            return None

    # Generator for lazy evaluation
    # def get_line(self, di, dj):
    #     i, j = self.rank+di, self.file.value+dj

    #     while self._is_inbounds(i, j):
    #         yield Coords(i, File(j))

    #         i, j = i+di, j+dj


    # def get_surrounding(self):
    #     for di, dj in [
    #         [-1, -1], [-1, 1], [1, -1], [1, 1],
    #         [-1, 0], [1, 0], [0, -1], [0, 1]
    #     ]:
    #         i, j = self.rank+di, self.file.value+dj
    #         if self._is_inbounds(i, j):
    #             yield Coords(i, File(j))

    # def get_knight_jumps(self):
    #     for di, dj in [
    #         [-2, -1], [-2, 1], [-1, -2], [-1, 2],
    #         [1, -2], [1, 2], [2, -1], [2, 1]
    #     ]:
    #         i, j = self.rank+di, self.file.value+dj

    #         if self._is_inbounds(i, j):
    #             yield Coords(i, File(j))




