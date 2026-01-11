# Day 881: 2D iterator over array of arrays without flattening. next/has_next amortized O(1).

class Iterator2D:
    def __init__(self, data):
        self.data = data
        self.row = 0
        self.col = 0

    def _advance(self):
        while self.row < len(self.data) and self.col >= len(self.data[self.row]):
            self.row += 1
            self.col = 0

    def has_next(self):
        self._advance()
        return self.row < len(self.data)

    def next(self):
        if not self.has_next():
            raise StopIteration("no more elements")
        val = self.data[self.row][self.col]
        self.col += 1
        return val


if __name__ == "__main__":
    it = Iterator2D([[1, 2], [3], [], [4, 5, 6]])
    out = []
    while it.has_next():
        out.append(str(it.next()))
    print(", ".join(out))
