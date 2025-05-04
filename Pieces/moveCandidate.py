class MoveCandidate():
    def __init__(self, capture, row_diff, col_diff, dist=1):
        self.capture = capture
        self._row_diff: int = row_diff
        self._col_diff: int = col_diff

        self._dist = dist

    def __repr__(self):
        return self.__str__()


    def __str__(self):
        s = "{} {} {} {}".format(
            self.capture,
            self._row_diff,
            self._col_diff,
            self._dist
        )

        return s