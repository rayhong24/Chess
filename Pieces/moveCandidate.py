class MoveCandidate():
    def __init__(self, row_diff, col_diff, dist=1, capture_allowed = False, capture_forced = False):
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