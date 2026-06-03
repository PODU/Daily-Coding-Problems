# Day 1609: Subscribers-per-hour over 24 hours via Fenwick/BIT. update(hour,val)+=, query(start,end)=inclusive range sum.
# Time O(log n) per op, Space O(n).


class Fenwick:
    def __init__(self, n: int):
        self.n = n
        self.tree = [0] * (n + 1)

    def update(self, i: int, v: int) -> None:
        i += 1
        while i <= self.n:
            self.tree[i] += v
            i += i & -i

    def _pref(self, i: int) -> int:
        i += 1
        s = 0
        while i > 0:
            s += self.tree[i]
            i -= i & -i
        return s

    def query(self, l: int, r: int) -> int:
        return self._pref(r) - (self._pref(l - 1) if l > 0 else 0)


if __name__ == "__main__":
    bit = Fenwick(24)  # all zeros
    bit.update(0, 5)
    bit.update(3, 10)
    bit.update(23, 2)
    print(bit.query(0, 23))  # 17
    print(bit.query(0, 3))   # 15
    print(bit.query(1, 2))   # 0
