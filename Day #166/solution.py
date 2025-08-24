# Day 166: 2D iterator: track row/col indices, _advance() skips empty subarrays. O(1) amortized per next/has_next, O(1) extra space.

class Iterator2D:
    def __init__(self, data):
        self.data = data
        self.row = 0
        self.col = 0
        self._advance()

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


def main():
    arr = [[1, 2], [3], [], [4, 5, 6]]
    it = Iterator2D(arr)
    out = []
    while it.has_next():
        out.append(str(it.next()))
    print(", ".join(out))


if __name__ == "__main__":
    main()
