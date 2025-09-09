# Day 242: Fenwick/Binary Indexed Tree over 24 hours.
# update: O(log n), query (prefix-diff): O(log n). Space O(n).
class BIT:
    def __init__(self, n):
        self.n = n
        self.tree = [0] * (n + 1)

    def add(self, i, v):            # 0-based index
        i += 1
        while i <= self.n:
            self.tree[i] += v
            i += i & (-i)

    def prefix(self, i):           # sum of [0..i], 0-based
        i += 1
        s = 0
        while i > 0:
            s += self.tree[i]
            i -= i & (-i)
        return s

    def query(self, l, r):         # inclusive [l..r]
        return self.prefix(r) - (self.prefix(l - 1) if l > 0 else 0)

    def update(self, hour, value):
        self.add(hour, value)


if __name__ == "__main__":
    bit = BIT(24)
    bit.update(2, 5)
    bit.update(5, 3)
    bit.update(23, 10)
    print("query(2,5) =", bit.query(2, 5))
    print("query(0,23) =", bit.query(0, 23))
