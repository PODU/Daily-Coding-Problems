# Day 1093: Count smaller elements to the right via Fenwick tree + coordinate compression.
# Time O(n log n), Space O(n).
import bisect


class BIT:
    def __init__(self, n):
        self.t = [0] * (n + 1)

    def update(self, i):
        while i < len(self.t):
            self.t[i] += 1
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
        rank = bisect.bisect_left(sorted_unique, a[i]) + 1
        res[i] = bit.query(rank - 1)
        bit.update(rank)
    return res


if __name__ == "__main__":
    print(count_smaller([3, 4, 9, 6, 1]))
