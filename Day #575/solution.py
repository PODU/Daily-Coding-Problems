# Day 575: 2D iterator over an array of arrays without flattening/cloning.
# Track (row, col); advance over empty rows. next/has_next amortized O(1).


class Iterator2D:
    def __init__(self, data):
        self.data = data
        self.row = 0
        self.col = 0
        self._skip_empty()

    def _skip_empty(self):
        while self.row < len(self.data) and self.col >= len(self.data[self.row]):
            self.row += 1
            self.col = 0

    def has_next(self):
        return self.row < len(self.data)

    def next(self):
        if not self.has_next():
            raise StopIteration("no more elements")
        v = self.data[self.row][self.col]
        self.col += 1
        self._skip_empty()
        return v


if __name__ == "__main__":
    it = Iterator2D([[1, 2], [3], [], [4, 5, 6]])
    out = []
    while it.has_next():
        out.append(it.next())
    print(", ".join(map(str, out)))  # 1, 2, 3, 4, 5, 6
