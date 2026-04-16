# Day 1373: 2D iterator with lazy outer/inner pointers (no flatten/clone).
# next() & has_next() amortized O(1), Space O(1) extra.


class Iterator2D:
    def __init__(self, data):
        self.data = data
        self.outer = 0
        self.inner = 0

    def has_next(self):
        while self.outer < len(self.data) and self.inner >= len(self.data[self.outer]):
            self.outer += 1
            self.inner = 0
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
    print(", ".join(map(str, out)))
