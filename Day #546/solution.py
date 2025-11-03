# Day 546: Count smaller elements to the right via Fenwick/BIT + coordinate compression.
# Time O(n log n), Space O(n).
import bisect


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
    n = len(a)
    sorted_uniq = sorted(set(a))
    bit = BIT(len(sorted_uniq))
    res = [0] * n
    for i in range(n - 1, -1, -1):
        rank = bisect.bisect_left(sorted_uniq, a[i]) + 1
        res[i] = bit.query(rank - 1)
        bit.update(rank, 1)
    return res


def main():
    a = [3, 4, 9, 6, 1]
    res = count_smaller(a)
    print("[" + ", ".join(str(x) for x in res) + "]")


if __name__ == "__main__":
    main()
