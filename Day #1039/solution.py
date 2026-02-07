# Day 1039: Fenwick/BIT over 24 hours: point update, prefix-sum range query.
# update O(log n), query O(log n).


class BIT:
    def __init__(self, n):
        self.n = n
        self.tree = [0] * (n + 1)

    def update(self, hour, value):
        i = hour + 1
        while i <= self.n:
            self.tree[i] += value
            i += i & (-i)

    def prefix(self, idx):  # sum of [0..idx]
        s = 0
        i = idx + 1
        while i > 0:
            s += self.tree[i]
            i -= i & (-i)
        return s

    def query(self, start, end):  # inclusive
        return self.prefix(end) - (self.prefix(start - 1) if start > 0 else 0)


def main():
    bit = BIT(24)
    bit.update(0, 5)
    bit.update(3, 10)
    bit.update(23, 2)
    bit.update(3, 1)
    print("query(0, 3) = " + str(bit.query(0, 3)))
    print("query(0, 23) = " + str(bit.query(0, 23)))
    print("query(4, 23) = " + str(bit.query(4, 23)))
    print("query(3, 3) = " + str(bit.query(3, 3)))


if __name__ == "__main__":
    main()
