# Day 1584: 2D iterator over array of arrays (no flatten/clone).
# Maintain (outer,inner) indices; _skip advances past empty inner arrays. Time: O(1) amortized; Space: O(1).


class Iterator2D:
    def __init__(self, data):
        self.data = data
        self.outer = 0
        self.inner = 0
        self._skip()

    def _skip(self):
        while self.outer < len(self.data) and self.inner >= len(self.data[self.outer]):
            self.outer += 1
            self.inner = 0

    def has_next(self):
        self._skip()
        return self.outer < len(self.data)

    def next(self):
        if not self.has_next():
            raise StopIteration("no more elements")
        val = self.data[self.outer][self.inner]
        self.inner += 1
        return val


if __name__ == "__main__":
    it = Iterator2D([[1, 2], [3], [], [4, 5, 6]])
    out = []
    while it.has_next():
        out.append(it.next())
    print(", ".join(map(str, out)))  # 1, 2, 3, 4, 5, 6
