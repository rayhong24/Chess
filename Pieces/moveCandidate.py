from coords import Coords
class MoveCandidate():
    def __init__(self, row_diff, col_diff, dist=1, capture_allowed = True, capture_forced = False):
        self._row_diff: int = row_diff
        self._col_diff: int = col_diff

        self._dist = dist

        self.capture_allowed = capture_allowed
        self.capture_forced = capture_forced

    def __repr__(self):
        return self.__str__()


    def __str__(self):
        s = "{} {} {}".format(
            self._row_diff,
            self._col_diff,
            self._dist
        )

        return s

    def generate_coords(self, start_coords: Coords):
        curr_coords = start_coords
        for _ in range(self._dist):
            if not curr_coords.diff_inbounds(self._row_diff, self._col_diff):
                return
            curr_coords = curr_coords.get_neighbour(self._row_diff, self._col_diff)
            yield(curr_coords)


            
