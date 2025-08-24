# Day 165: Count smaller elements to the right. Coordinate-compress, then sweep
# right to left with a Fenwick tree (BIT). Time O(n log n), Space O(n).
from bisect import bisect_left


class BIT:
    def __init__(self, n):
        self.t = [0] * (n + 1)

    def update(self, i, v):
        while i < len(self.t):
            self.t[i] += v
            i += i & -i

    def query(self, i):
        s = 0
        while i > 0:
            s += self.t[i]
            i -= i & -i
        return s


def count_smaller(a):
    sorted_unique = sorted(set(a))
    bit = BIT(len(sorted_unique))
    res = [0] * len(a)
    for i in range(len(a) - 1, -1, -1):
        rank = bisect_left(sorted_unique, a[i]) + 1
        res[i] = bit.query(rank - 1)
        bit.update(rank, 1)
    return res


if __name__ == "__main__":
    print(count_smaller([3, 4, 9, 6, 1]))  # [1, 1, 2, 1, 0]
